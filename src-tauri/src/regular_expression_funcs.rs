mod regex_models;
use std::collections::HashMap;

use app::create_unique_state_coordinates;
use regex_models::{BinaryOperator, KleeneOperator, OrOperator, ParsingError, Token, UnaryOperator, Operator};

use app::models::{SmartState, State, Coordinate};

use crate::{advanced_automata_funcs::convert_nfa_to_dfa, testing_automata_funcs::{test_string_dfa, test_string_nfa}};
mod tests;

#[tauri::command]
pub fn test_string_regex(parse_tree: Token, string_to_check: String) -> bool {

  let mut state_positions = HashMap::new();

  let start_state_coords  = create_unique_state_coordinates(&state_positions.keys().cloned().collect());
  let start_state = State::new(start_state_coords, true, false);
  state_positions.insert(start_state_coords.into(), start_state.to_owned());

  let end_state_coords = create_unique_state_coordinates(&state_positions.keys().cloned().collect());
  let end_state = State::new(end_state_coords, false, true);
  state_positions.insert(end_state_coords.into(), end_state.to_owned());

  convert_token_to_nfa(
    &mut state_positions,
    start_state_coords.into(),
    parse_tree,
    end_state_coords);
  
  println!("{state_positions:?}");

  println!{"{:?}", test_string_nfa(state_positions.clone(), start_state_coords.clone().into(), string_to_check.clone()).0};

  let (_, _, _, state_positions) = convert_nfa_to_dfa(state_positions, start_state_coords.into());

  return test_string_dfa(state_positions.into(), start_state_coords.into(), string_to_check).0;

}

fn convert_token_to_nfa(
  state_positions: &mut HashMap<String, State>, 
  current_state_coords: Coordinate, 
  token_to_convert: Token, 
  end_state_coords: Coordinate) {

  // Need to change to include epsilon as well

  // Create states representative of the parse tree connected to the start_state
  match token_to_convert {
    Token::Literal(literal) => {

      let first_new_state_coords = create_unique_state_coordinates(&state_positions.keys().cloned().collect());

      let mut new_state = State::new(first_new_state_coords, false, false);
      let mut new_state_coords = first_new_state_coords;
      state_positions.insert(new_state_coords.into(), new_state.clone());


      for c in literal[1..].chars() {
        let next_new_state_coords = create_unique_state_coordinates(&state_positions.keys().cloned().collect());
        new_state
          .add_connection(&c.to_string(), next_new_state_coords);
        state_positions.insert(new_state_coords.into(), new_state);

        new_state = State::new(next_new_state_coords, false, false);
        new_state_coords = next_new_state_coords;
        state_positions.insert(new_state_coords.into(), new_state.clone());
      }

      let current_state = state_positions
        .get_mut::<String>(&current_state_coords.into())
        .expect("Failed to retrieve the requested state");

      current_state.add_connection(&literal
        .chars()
        .nth(0)
        .expect("The literal should not be length 0")
        .to_string(), first_new_state_coords);

      let new_state = state_positions
        .get_mut::<String>(&new_state_coords.into())
        .expect("Failed to retrieve the requested state");

      new_state
        .add_connection("ϵ", end_state_coords);

    },
    Token::GroupedExpression(_) => {
      // Unreachable state, should never occur if parse tree is properly verified
      panic!("All grouped expressions should be parsed after generating the parse tree");
    },
    Token::OrOperator(operator) => {

      let left_token = operator
        .get_left_argument()
        .expect("The left argument should not have a None value
        make sure to verify validity of parse tree before running this function");

      let right_token = operator
        .get_right_argument()
        .expect("The right argument should not have a None value
        make sure to verify validity of parse tree before running this function");

      convert_token_to_nfa(state_positions, current_state_coords, left_token.to_owned(), end_state_coords);
      convert_token_to_nfa(state_positions, current_state_coords, right_token.to_owned(), end_state_coords);
    },

    // Will need to alter in future to accomodate concatonation
    Token::KleeneOperator(operator) => {

      let inner_argument = operator
        .get_inner_argument()
        .expect("The inner argument should not have a None value
        make sure to verify validity of parse tree before running this function");

        // Here's my thinking. A kleene operator allows an indefinite number of repeats of characters
        // Thus I am thinking we treat any instances of a kleene operator as almost a separate nfa which, at its end,
        // loops back to the current state that we're on and is connected by the current state via an epsilon transition

        let new_state_coords = create_unique_state_coordinates(&state_positions.keys().cloned().collect());

        let current_state = state_positions
          .get_mut::<String>(&current_state_coords.into())
          .expect("Failed to retrieve the requested state");
        
        current_state.add_connection("ϵ", end_state_coords);
        current_state.add_connection("ϵ", new_state_coords);
        state_positions.insert(new_state_coords.into(), State::new(new_state_coords, false, false));

        handle_kleene_token_to_nfa_conversion(current_state_coords, state_positions, 
          inner_argument.to_owned(), new_state_coords);

    },
    _=> todo!()
  }

}

fn handle_kleene_token_to_nfa_conversion(
  coords_of_state_to_loop_to: Coordinate, 
  state_positions: &mut HashMap<String, State>, 
  current_token: Token, 
  current_state_coords: Coordinate) {

  match current_token {
    Token::Literal(literal) => {
      let first_new_state_coords = create_unique_state_coordinates(&state_positions.keys().cloned().collect());

      let mut new_state = State::new(first_new_state_coords, false, false);
      let mut new_state_coords = first_new_state_coords;
      state_positions.insert(new_state_coords.into(), new_state.clone());

      for c in literal[1..].chars() {
        let next_new_state_coords = create_unique_state_coordinates(&state_positions.keys().cloned().collect());
        new_state
          .add_connection(&c.to_string(), next_new_state_coords);
        state_positions.insert(new_state_coords.into(), new_state);

        new_state = State::new(next_new_state_coords, false, false);
        new_state_coords = next_new_state_coords;
        state_positions.insert(new_state_coords.into(), new_state.clone());
      }

      let current_state = state_positions
        .get_mut::<String>(&current_state_coords.into())
        .expect("Failed to retrieve the requested state");

      current_state.add_connection(&literal
        .chars()
        .nth(0)
        .expect("The literal should not be length 0")
        .to_string(), first_new_state_coords);

      current_state
        .add_connection("ϵ", coords_of_state_to_loop_to);

      let new_state = state_positions
        .get_mut::<String>(&new_state_coords.into())
        .expect("Failed to retrieve the requested state");

      new_state
        .add_connection("ϵ", coords_of_state_to_loop_to);
      
    },
    Token::GroupedExpression(_) => {
      // Unreachable state, should never occur if parse tree is properly verified
      panic!("All grouped expressions should be parsed after generating the parse tree");
    },
    Token::OrOperator(operator) => {
      let left_token = operator
        .get_left_argument()
        .expect("The left argument should not have a None value
        make sure to verify validity of parse tree before running this function");
      
      let right_token = operator
        .get_right_argument()
        .expect("The right argument should not have a None value
        make sure to verify validity of parse tree before running this function");

      handle_kleene_token_to_nfa_conversion(
        coords_of_state_to_loop_to, 
        state_positions, 
        left_token.to_owned(), 
        current_state_coords);

      handle_kleene_token_to_nfa_conversion(
        coords_of_state_to_loop_to, 
        state_positions, 
        right_token.to_owned(), 
        current_state_coords);

    },
    Token::KleeneOperator(operator) => {
      
      let inner_argument = operator
        .get_inner_argument()
        .expect("The inner argument should not have a None value
        make sure to verify validity of parse tree before running this function");

      let new_state_coords = create_unique_state_coordinates(&state_positions.keys().cloned().collect());

      let current_state = state_positions
        .get_mut::<String>(&current_state_coords.into())
        .expect("Failed to retrieve the requested state");
      
      current_state.add_connection("ϵ", new_state_coords);

      handle_kleene_token_to_nfa_conversion(current_state_coords, state_positions, 
        inner_argument.to_owned(), new_state_coords);

      },
      _=> todo!()
  }
 

}

#[tauri::command]
pub fn interpret_regex(regex: &str) -> Result<Token, ParsingError> {

  let (tokens, _) = tokenize_regular_expression(regex);
  let parsed_tokens = parse_tokens(tokens)?;
  let final_parse_tree = concatenate_tokens(parsed_tokens);

  verify_syntactic_correctness_of_parse_tree(&final_parse_tree)?;
  return Ok(final_parse_tree);
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

fn parse_tokens(mut tokens: Vec<Token>) -> Result<Vec<Token>, ParsingError> {

  if tokens.len() == 0 {
    return Err(ParsingError::NoInnerArg)
  } else if tokens.len() == 1 {
    match tokens.get(0).expect("The array should have at least a single element") {
      // If it's a grouped expression, do nothing and continue breaking it apart
      Token::GroupedExpression(_) => (),
      _ => {
        return Ok(tokens)
      }
    }
  } else if !can_continue_parsing(&tokens) {
    return Ok(tokens);
  }
  // Very gross code will definitely be rewriten in future
  for (index, token) in tokens.clone().into_iter().enumerate() {
    match token {
      Token::OrOperator(mut current_or_op) => {
        if !current_or_op.has_empty_arg() {
          continue;
        }

        let duplicate_tokens = tokens.clone();
        let left_argument = &duplicate_tokens[0..index];
        let right_argument = &duplicate_tokens[index+1..];
        tokens.remove(index);
        let mut left_tokens_to_concat = Vec::new();
        let mut right_tokens_to_concat = Vec::new();

        if left_argument.len() > 0 { 
          tokens.drain(0..left_argument.len());
          let left_argument = parse_tokens(left_argument.to_owned())?;
          if left_argument.len() == 1 {
            current_or_op.left_insert_token(left_argument.get(0).cloned());
          } else {
            // If the result of attempting to group the left argument does not result in a single token, 
            // We must resort to bubbling up the parts which we could not adequtely group to be concatonated later
            let retrieved_parsed_operator = 
              get_operator_and_tokens_to_concatenate(left_argument.to_owned()).0;

            left_tokens_to_concat = get_operator_and_tokens_to_concatenate(left_argument.to_owned()).1;
            
            current_or_op.left_insert_token(retrieved_parsed_operator);
          }
        }
        
        if right_argument.len() > 0 {
          tokens.drain(0..right_argument.len());
          let right_argument = parse_tokens(right_argument.to_owned())?;
          if right_argument.len() == 1 {
            current_or_op.right_insert_token(right_argument.get(0).cloned());
          } else {
            // If the result of attempting to group the right argument does not result in a single token, 
            // We must resort to bubbling up the parts which we could not adequtely group to be concatonated later
            let retrieved_parsed_operator = 
              get_operator_and_tokens_to_concatenate(right_argument.to_owned()).0;

            right_tokens_to_concat = get_operator_and_tokens_to_concatenate(right_argument.to_owned()).1;
            
            current_or_op.right_insert_token(retrieved_parsed_operator);
          }
        }
        for t in left_tokens_to_concat {
          tokens.push(t);
        }
        tokens.push(Token::OrOperator(current_or_op));
        for t in right_tokens_to_concat {
          tokens.push(t);
        }
        
        break;
      },
      Token::KleeneOperator(mut current_kleene_op) => {
        if !current_kleene_op.has_empty_arg() {
          continue;
        }
        let left_of_kleene_operator = &tokens.clone()[..index];
        if left_of_kleene_operator.len() > 0 {
          tokens.drain(..=left_of_kleene_operator.len());
          let inner_argument = parse_tokens(left_of_kleene_operator.to_owned())?;
          if inner_argument.len() == 1 {
            current_kleene_op.insert_token(inner_argument.get(0).cloned())?;
            tokens.insert(0, Token::KleeneOperator(current_kleene_op));
          } else {
            let (operator_to_insert, tokens_to_concat) = 
              get_operator_and_tokens_to_concatenate(inner_argument.to_owned());
            current_kleene_op.insert_token(operator_to_insert);
            tokens.insert(0, Token::KleeneOperator(current_kleene_op));
            for t in tokens_to_concat {
              tokens.insert(0, t);
            }
          }
          break;
        }

      },
      Token::GroupedExpression(token_pointer) => {
        let expanded_tokens = token_pointer
          .to_vec();
        tokens.remove(index);
        let mut current_working_index = index;
        for token in parse_tokens(expanded_tokens)? {
          tokens.insert(index, token);
          current_working_index += 1;
        }
        break;
      },
      _ => continue
    }
  
  };
  
  return parse_tokens(tokens);

}

// May have to change to in case token position matters
// Function assumes that the operator in the vec has been fully grouped to the extent it can
fn get_operator_and_tokens_to_concatenate(tokens: Vec<Token>) -> (Option<Token>, Vec<Token>) {

  let mut grouped_op_token = None;
  let mut tokens_to_concatenate = Vec::new();

  for token in tokens {
    match token {
      Token::OrOperator(_) | Token::KleeneOperator(_) => {
        grouped_op_token = Some(token);
      },
      _ => tokens_to_concatenate.push(token)

    }
  };

  return (grouped_op_token, tokens_to_concatenate); 
}


fn can_continue_parsing(tokens: &Vec<Token>) -> bool {
  for token in tokens {
    match token {
      Token::OrOperator(or_operator) => {
        if or_operator.has_empty_arg() {
          return true;
        }
      },
      Token::KleeneOperator(kleene_operator) => {
        if kleene_operator.has_empty_arg() {
          return true;
        }
      },
      // Done so grouped expressions are continued to be broked up in the parsing step
      Token::GroupedExpression(_) => {
        return true;
      },
      _=> continue
    }
  }
  return false;

}

fn concatenate_tokens(tokens: Vec<Token>) -> Token {

  if tokens.len() == 1 {
    return tokens
      .get(0)
      .expect("The array should have at least a single element")
      .to_owned();

  } else if tokens.len() == 2 {

    let first_token = tokens
      .get(0)
      .expect("The array should have at least 2 elements");

    let second_token = tokens
      .get(1)
      .expect("The array should have at least 2 elements");
    return Token::ConcatenatedExpression(Box::new((first_token.to_owned(), second_token.to_owned())));
  }


  let midpoint = tokens.len().div_ceil(2);
  let first_half_of_tokens = &tokens[0..midpoint];
  let second_half_of_tokens = &tokens[midpoint+1..];

  return Token::ConcatenatedExpression(Box::new((
    concatenate_tokens(first_half_of_tokens.to_owned()),
    concatenate_tokens(second_half_of_tokens.to_owned())
  )));

}


// May need to change into has empty left or right arguments respecively for or operator to work
pub fn can_concatenate_tokens(first_token: Token, second_token: Token) -> bool {
  match first_token.clone() {
    Token::Literal(_) | Token::ConcatenatedExpression(_) | Token::GroupedExpression(_) => {
      match second_token.clone() {
        Token::OrOperator(operator) => {
          if !operator.has_empty_arg() {
            return true;
          }
        },
        Token::KleeneOperator(operator) => {
          if !operator.has_empty_arg() {
            return true;
          }
        },
        _ => {
          return true;
        }
      }
    },
    Token::KleeneOperator(kleene_operator) => {
      if kleene_operator.has_empty_arg() {
        return false;
      }
      match second_token.clone() {
        Token::OrOperator(operator) => {
          if !operator.has_empty_arg() {
            return true;
          }
        },
        Token::KleeneOperator(operator) => {
          if !operator.has_empty_arg() {
            return true;
          }
        }, 
        _ => {
          return true;
        }
      }
    },
    Token::OrOperator(or_operator) => {
      if or_operator.has_empty_arg() {
        return false;
      }
      match second_token.clone() {
        Token::OrOperator(operator) => {
          if !operator.has_empty_arg() {
            return true;
          }
        },
        Token::KleeneOperator(operator) => {
          if !operator.has_empty_arg() {
            return true;
          }
        }, 
        _ => {
          return true;
        }
      }
    }
  }

  return false;

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