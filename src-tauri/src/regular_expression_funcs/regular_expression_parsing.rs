use super::regular_expression_models::{Token, TokenArray, TokenArrayMethods, ParsingError, OrOperator, KleeneOperator, BinaryOperator, UnaryOperator};

#[tauri::command(rename_all = "snake_case")]
pub fn build_parse_tree(regex: &str) -> Result<Token, ParsingError> {
  let (tokenized_expression, _) = tokenize_regular_expression(regex)?;
  let parse_tree = tokenized_expression.parse_tokens()?;
  parse_tree.verify_syntactic_correctness()?;
  return Ok(parse_tree);
}

fn tokenize_regular_expression(regex: &str) -> Result<(TokenArray, Option<usize>), ParsingError> {

  let mut tokens: TokenArray = vec![];
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
      
      // Currently, keeps reiterating over previously accounted for tokens
      let (tokens_in_brackets, number_of_characters_in_brackets) = tokenize_regular_expression(&regex[index+1..])?;

      tokens.push(Token::GroupedExpression(Box::new(tokens_in_brackets)));
      current_working_index += number_of_characters_in_brackets.ok_or_else(|| {
        ParsingError::UnableToConcatenate
      })? + 1;

    } else if c == ')' {
      return Ok((tokens, Some(index)));
    } else if !c.is_whitespace() {
      // We've encountered a character which we will add to our list of tokens
      // If that character is placed beside any other characters without whitespace, we automatically concatenate
      // and build a concatenation tree
      let (tokenized_literal, characters_to_skip) = Token::parse_string_to_tokens(&regex[index..]);
      tokens.push(
        tokenized_literal
      );
      current_working_index += characters_to_skip - 1;
    }
  current_working_index += 1;
  }
  return Ok((tokens, None));
}