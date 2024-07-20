use app::regex_models::*;

#[tauri::command]
pub fn interpret_regex(regex: &str) {

  let (tokens, _) = tokenize_regular_expression(regex);
  // Need to better address how to handle grouped expressions
  // Additionally, I want it such that the leaf nodes of a grouped expression can only ever be literals
  // ^^^Very important
  let parsed_tokens = parse_tokens(tokens);

  println!("{:?}", parsed_tokens);
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
        if index == 0 {
          return Err(ParsingError::NoInnerArg);
        }
        let mut duplicate_tokens = tokens.clone();
        let _ = duplicate_tokens.split_off(index);
        let left_of_kleene_operator = duplicate_tokens;
        tokens.drain(0..=left_of_kleene_operator.len());

        let inner_argument = parse_tokens(left_of_kleene_operator)?;

        current_kleene_op.insert_token(inner_argument)?;

        tokens.insert(0, Token::KleeneOperator(current_kleene_op));

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

      tokens.push(
        regex[index..].into()
      );
    }
    current_working_index += 1;

  }

  return (tokens, None);

}