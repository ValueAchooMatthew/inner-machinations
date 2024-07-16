use app::regex_models::*;

#[tauri::command]
pub fn interpret_regex(regex: &str) {

  let (tokens, _) = tokenize_regular_expression(regex);
  let parsed_tokens = parse_tokens(tokens);

  println!("{:?}", parsed_tokens.unwrap());
}

fn parse_tokens(mut tokens: Vec<Token>) -> Result<Vec<Token>, ParsingError> {

  let mut tree = vec![];
  let mut finished = false;
  while !finished {
    finished = true;
    for (index, token) in tokens.clone().into_iter().enumerate() {
      match token {
        Token::BinaryOperator(op_type) => {
  
          let bin_op;
  
          if op_type == '+' {
            let Some(left_argument) = tokens
              .get(index - 1)
              else {break};
  
            let Some(right_argument) = tokens
              .get(index + 1)
              else {break};
  
            bin_op = OrOperator::new(
              left_argument.to_owned(),
              right_argument.to_owned()
            );
            finished = false;
            tree.push(ParseTree::Or(bin_op).into());
          }
  
        },
        Token::UnaryOperator(op_type) => {
          let un_op;
  
          if op_type == '*' {
            let Some(inner_argument) = tokens
              .get(index - 1)
              else {break};
  
            un_op = KleeneOperator::new(
              inner_argument.to_owned()
            );
            tree.push(ParseTree::Kleene(un_op).into());
            finished = false;
          }
  
  
        },
        _ => continue
      }
    
    }
    tokens = tree.clone();
  
  }
  
  return Ok(tree);

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
        Token::BinaryOperator('+')
      );
    } else if c == '*' {
      tokens.push(
        Token::UnaryOperator('*')
      );
    } else if c == '(' {
      
      // Needs a LOT of work in future
      // Currently, keeps reiterating over previously accounted for tokens
      let tokens_in_brackets = tokenize_regular_expression(&regex[index+1..]);

      tokens.push(tokens_in_brackets.0.into());
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