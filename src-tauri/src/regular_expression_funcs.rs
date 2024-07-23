mod regex_models;
use std::collections::HashMap;


use app::create_unique_state_coordinates;
use regex_models::{BinaryOperator, KleeneOperator, OrOperator, ParsingError, Token, UnaryOperator, Operator};

use app::models::{SmartState, State};
mod tests;

pub fn covert_parse_tree_to_nfa(parse_tree: Token) {
  let mut state_positions = HashMap::new();

  let start_state_coordinates  = create_unique_state_coordinates(&state_positions);

  // Need to change to include epsilon as well
  let start_state = State::new(start_state_coordinates, true, false);

  // Create states representative of the parse tree connected to the start_state
  let mut current_state = start_state;
  let mut current_token = parse_tree;

  match current_token {
    Token::Literal(literal) => {
      // current_state.states_connected_to = 


    },
    Token::GroupedExpression(_) => todo!(),
    Token::OrOperator(_) => todo!(),
    Token::KleeneOperator(_) => todo!(),
  }

}

#[tauri::command]
pub fn interpret_regex(regex: &str) -> Result<Token, ParsingError> {

  let (tokens, _) = tokenize_regular_expression(regex);
  let parsed_tokens = parse_tokens(tokens)?;
  verify_syntactic_correctness_of_parse_tree(&parsed_tokens)?;
  println!("{:?}", &parsed_tokens);
  return Ok(parsed_tokens);
}

fn verify_syntactic_correctness_of_parse_tree(parse_tree: &Token) -> Result<(), ParsingError> {
  // If we come across an operator which has an None argument, a value was not
  // properly supplied to the operator and thus the tree is syntactically incorrect
  // We check for this using DFS

  // Messy, refactor in future
  match parse_tree {
    Token::KleeneOperator(operator) => { 
      if operator.has_empty_arg() {
        return Err(ParsingError::NoInnerArg);
      } else {

        let inner_argument = operator.get_inner_argument().unwrap();
        return verify_syntactic_correctness_of_parse_tree(inner_argument);

      }
    },
    Token::OrOperator(operator) => {
      if operator.has_empty_arg() {
        if operator.get_left_argument().is_none() {
          return Err(ParsingError::NoLeftArg);
        } else {
          return Err(ParsingError::NoRightArg);
        }
      } else {
        // Definitely refactor in future, ugly code
        let left_argument = operator.get_left_argument().unwrap();
        let right_argument = operator.get_right_argument().unwrap();
        let result_of_checking_left_arg = verify_syntactic_correctness_of_parse_tree(left_argument);
        let result_of_checking_right_arg = verify_syntactic_correctness_of_parse_tree(right_argument);
        if result_of_checking_left_arg.is_ok() && result_of_checking_right_arg.is_ok() {
          return Ok(());
        } else if result_of_checking_left_arg.is_err() {
          return result_of_checking_left_arg;
        } else {
          return result_of_checking_right_arg;
        }

      }

    },

    _ => return Ok(())

  }

}

fn parse_tokens(mut tokens: Vec<Token>) -> Result<Token, ParsingError> {

  if tokens.len() == 0 {
    return Err(ParsingError::NoInnerArg)
  } else if tokens.len() == 1 {

    let final_token = tokens
      .get(0)
      .expect("The vec should have at least 1 element")
      .to_owned();

    match final_token {
      Token::GroupedExpression(_) => {
        // do nothing and continue, so the grouped expression continues to be broken apart
      },
      final_token => return Ok(final_token)

    } 
  }

  for (index, token) in tokens.clone().into_iter().enumerate() {
    match token {
      Token::OrOperator(mut current_or_op) => {
        let duplicate_tokens = tokens.clone();
        let (left_argument, right_argument) = duplicate_tokens.split_at(index);
        // Done so we don't include the current token in the right argument
        let mut right_argument = right_argument.to_owned();
        right_argument.remove(0);
        tokens.remove(index);

        if left_argument.len() > 0 {
          tokens.drain(0..left_argument.len());

          let left_argument = parse_tokens(left_argument.to_owned())?;
          current_or_op.left_insert_token(left_argument)?;
          tokens.insert(0, Token::OrOperator(current_or_op.clone()));
          break;
        }

        if right_argument.len() > 0 {
          tokens.drain(0..right_argument.len());
          let right_argument = parse_tokens(right_argument)?;
          current_or_op.right_insert_token(right_argument)?;
          tokens.insert(index, Token::OrOperator(current_or_op));
          break;
        }
      },
      Token::KleeneOperator(mut current_kleene_op) => {

        let mut duplicate_tokens = tokens.clone();
        let _ = duplicate_tokens.split_off(index);
        let left_of_kleene_operator = duplicate_tokens;
        if left_of_kleene_operator.len() > 0 {
          tokens.drain(0..=left_of_kleene_operator.len());
          let inner_argument = parse_tokens(left_of_kleene_operator)?;
          current_kleene_op.insert_token(inner_argument)?;
          tokens.insert(0, Token::KleeneOperator(current_kleene_op));
          break;
        }

      },
      Token::GroupedExpression(token_pointer) => {
        let expanded_tokens = token_pointer
          .to_vec();
        tokens.remove(index);
        tokens.insert(index, parse_tokens(expanded_tokens)?);
      },
      _ => continue
    }
  
  };
  
  return parse_tokens(tokens);

}

fn tokenize_regular_expression(regex: &str) -> (Vec<Token>, Option<usize>) {

  let mut tokens: Vec<Token> = vec![];
  let mut current_working_index: usize = 0;

  for (index, c) in regex.chars().enumerate() {
    if index < current_working_index {
      // We do this to cleverly avoid duplicating the same tokens whenever we enter into
      // a bracketed expression. Whenever we are done grouping the data in brackets into a token
      // We will continue reading from after the closing bracket, hence the return type of the function
      // includes the index of the closing bracket to act as an offset 
      continue;
    } else if c == '+' {
      tokens.push(
        Token::OrOperator(Box::new(OrOperator::new(None, None)))
      );
    } else if c == '*' {
      tokens.push(
        Token::KleeneOperator(Box::new(KleeneOperator::new(None)))
      );
    } else if c == '(' {
      
      // Needs a LOT of work in future
      // Currently, keeps reiterating over previously accounted for tokens
      let tokens_in_brackets = tokenize_regular_expression(&regex[index+1..]);

      tokens.push(Token::GroupedExpression(Box::new(tokens_in_brackets.0)));
      current_working_index += tokens_in_brackets.1
        .expect("The regex should have a closing bracket") + 1;

    } else if c == ')' {
      return (tokens, Some(index));
    } else if !c.is_whitespace() {
      // We've encountered a character which we will add to our list of tokens
      // Since a 'character' in the regex sense could hypothetically be more than one character long
      // Hence the into method on the regex slice starting at the current index
      let tokenized_literal = regex[index..].into();
      match &tokenized_literal {
        Token::Literal(literal) => {
          current_working_index += literal.len() - 1;
        }
        _ => {
          panic!("A literal should be returned");
        }
      }

      tokens.push(
        tokenized_literal
      );
    }
  current_working_index += 1;
  }

  return (tokens, None);

}