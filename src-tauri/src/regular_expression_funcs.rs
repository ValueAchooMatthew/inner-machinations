mod regex_models;

use crate::automaton::Automaton;
use crate::error::{CommandResult, Error};
use regex_models::{ConcatenatedExpression, KleeneOperator, OrOperator, ParsingError, Token};

#[cfg(test)]
mod tests;

// How the parse tree writes the empty string. Typing ε or ϵ in a regular expression means the empty string
pub const EMPTY_STRING: &str = "ε";

#[tauri::command]
pub fn test_string_regex(regex: &str, string_to_check: String) -> CommandResult<bool> {

  let parse_tree = parse_regular_expression(regex).map_err(|error| Error::new(error.to_string()))?;
  let nfa = convert_parse_tree_to_nfa(&parse_tree);

  return Ok(nfa.accepts(&string_to_check));

}

#[tauri::command]
pub fn build_parse_tree(regex: &str) -> CommandResult<Token> {
  parse_regular_expression(regex).map_err(|error| Error::new(error.to_string()))
}

// Precedence, from loosest to tightest: union (+ or |), then concatenation, then the Kleene star.
// Whitespace is ignored
pub fn parse_regular_expression(regex: &str) -> Result<Token, ParsingError> {

  let mut parser = Parser {
    characters: regex.chars().filter(|character| !character.is_whitespace()).collect(),
    position: 0,
    bracket_depth: 0
  };

  // Outside brackets the union only stops at the end of the expression
  return parser.parse_union();

}

struct Parser {
  characters: Vec<char>,
  position: usize,
  // How many brackets are open at the current position
  bracket_depth: usize
}

impl Parser {

  fn peek(&self) -> Option<char> {
    self.characters.get(self.position).copied()
  }

  // union := concatenation (("+" | "|") concatenation)*
  fn parse_union(&mut self) -> Result<Token, ParsingError> {

    let mut left_argument = self.parse_concatenation()?;

    while matches!(self.peek(), Some('+' | '|')) {
      self.position += 1;

      let left = left_argument.ok_or(ParsingError::EmptyLeftArg)?;
      let right = self.parse_concatenation()?.ok_or(ParsingError::EmptyRightArg)?;

      left_argument = Some(Token::OrOperator(Box::new(OrOperator::new(left, right))));
    }

    left_argument.ok_or(ParsingError::EmptyExpression)

  }

  // concatenation := starred*, which is None when there are no factors
  fn parse_concatenation(&mut self) -> Result<Option<Token>, ParsingError> {

    let mut factors = vec![];

    while let Some(character) = self.peek() {
      if character == ')' && self.bracket_depth == 0 {
        return Err(ParsingError::UnmatchedClosingBracket);
      }
      if matches!(character, '+' | '|' | ')') {
        break;
      }
      factors.push(self.parse_starred()?);
    }

    if factors.is_empty() {
      return Ok(None);
    }

    Ok(Some(concatenate_tokens(factors)))

  }

  // starred := atom "*"*
  fn parse_starred(&mut self) -> Result<Token, ParsingError> {

    let mut token = self.parse_atom()?;

    while self.peek() == Some('*') {
      self.position += 1;
      token = Token::KleeneOperator(Box::new(KleeneOperator::new(token)));
    }

    Ok(token)

  }

  // atom := "(" union ")" | a single character
  fn parse_atom(&mut self) -> Result<Token, ParsingError> {

    let Some(character) = self.peek() else {
      return Err(ParsingError::EmptyExpression);
    };
    self.position += 1;

    match character {
      '*' => Err(ParsingError::NoInnerArg),
      '(' => {
        self.bracket_depth += 1;
        let inner_expression = match self.parse_union() {
          Err(ParsingError::EmptyExpression) if self.peek() == Some(')') => Err(ParsingError::EmptyBrackets),
          Err(ParsingError::EmptyExpression) => Err(ParsingError::MissingClosingBracket),
          other => other
        }?;
        if self.peek() != Some(')') {
          return Err(ParsingError::MissingClosingBracket);
        }
        self.position += 1;
        self.bracket_depth -= 1;
        Ok(inner_expression)
      },
      'ε' | 'ϵ' => Ok(Token::Literal(EMPTY_STRING.to_owned())),
      _ => Ok(Token::Literal(character.to_string()))
    }

  }

}

// Splits the tokens in half each time so the drawn tree stays shallow
fn concatenate_tokens(mut tokens: Vec<Token>) -> Token {

  if tokens.len() == 1 {
    return tokens.remove(0);
  }

  let second_half_of_tokens = tokens.split_off(tokens.len().div_ceil(2));

  return Token::ConcatenatedExpression(Box::new(ConcatenatedExpression::new(
    concatenate_tokens(tokens),
    concatenate_tokens(second_half_of_tokens)
  )));

}

// Thompson's construction
pub fn convert_parse_tree_to_nfa(parse_tree: &Token) -> Automaton {

  let mut nfa = Automaton::new();
  let (entry_state, exit_state) = add_token_to_nfa(&mut nfa, parse_tree);

  nfa.set_start(entry_state);
  nfa.set_accepting(exit_state, true);

  return nfa;

}

// Adds states for the token and returns its entry and exit states. The token matches a string
// exactly when that string leads from its entry state to its exit state
fn add_token_to_nfa(nfa: &mut Automaton, token: &Token) -> (usize, usize) {

  match token {
    Token::Literal(literal) => {
      let entry_state = nfa.add_state(false);
      let exit_state = nfa.add_state(false);

      let symbol = if literal == EMPTY_STRING { None } else { literal.chars().next() };
      nfa.add_transition(entry_state, symbol, exit_state);

      (entry_state, exit_state)
    },
    Token::ConcatenatedExpression(concatenated_expression) => {
      let (left_entry_state, left_exit_state) = add_token_to_nfa(nfa, concatenated_expression.get_left_argument());
      let (right_entry_state, right_exit_state) = add_token_to_nfa(nfa, concatenated_expression.get_right_argument());

      nfa.add_transition(left_exit_state, None, right_entry_state);

      (left_entry_state, right_exit_state)
    },
    Token::OrOperator(or_operator) => {
      let entry_state = nfa.add_state(false);
      let (left_entry_state, left_exit_state) = add_token_to_nfa(nfa, or_operator.get_left_argument());
      let (right_entry_state, right_exit_state) = add_token_to_nfa(nfa, or_operator.get_right_argument());
      let exit_state = nfa.add_state(false);

      nfa.add_transition(entry_state, None, left_entry_state);
      nfa.add_transition(entry_state, None, right_entry_state);
      nfa.add_transition(left_exit_state, None, exit_state);
      nfa.add_transition(right_exit_state, None, exit_state);

      (entry_state, exit_state)
    },
    Token::KleeneOperator(kleene_operator) => {
      // The inner expression gets its own entry and exit, so repeating it can never loop back
      // into whatever comes before or after the star
      let entry_state = nfa.add_state(false);
      let (inner_entry_state, inner_exit_state) = add_token_to_nfa(nfa, kleene_operator.get_inner_argument());
      let exit_state = nfa.add_state(false);

      nfa.add_transition(entry_state, None, inner_entry_state);
      nfa.add_transition(entry_state, None, exit_state);
      nfa.add_transition(inner_exit_state, None, inner_entry_state);
      nfa.add_transition(inner_exit_state, None, exit_state);

      (entry_state, exit_state)
    }
  }

}
