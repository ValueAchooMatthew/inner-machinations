mod regex_models;
use std::collections::HashMap;

use app::{create_unique_state_coordinates, remove_all_epsilon_transitions};
use regex_models::{BinaryOperator, ConcatenatedExpression, KleeneOperator, Operator, OrOperator, ParsingError, Token, UnaryOperator};

use app::models::{State, Coordinate};

use crate::{advanced_automata_funcs::reconstruct_nfa_state_positions, testing_automata_funcs::test_string_nfa};
mod tests;

#[tauri::command]
pub fn test_string_regex(regex: &str, string_to_check: String) -> Result<bool, ParsingError> {

  let parse_tree = build_parse_tree(regex)?;

  let mut state_positions = HashMap::new();

  let start_state_coords  = create_unique_state_coordinates(&state_positions.keys().cloned().collect());
  let start_state = State::new(start_state_coords, true, false);
  state_positions.insert(start_state_coords.into(), start_state.to_owned());

  let end_state_coords = create_unique_state_coordinates(&state_positions.keys().cloned().collect());
  let end_state = State::new(end_state_coords, false, true);
  state_positions.insert(end_state_coords.into(), end_state.to_owned());

  convert_parse_tree_to_nfa(
    &mut state_positions,
    start_state_coords.into(),
    parse_tree,
    end_state_coords
  );

  remove_all_epsilon_transitions(&mut state_positions);

  let state_positions = reconstruct_nfa_state_positions(&state_positions, start_state_coords.into());

  return Ok(test_string_nfa(state_positions, start_state_coords.into(), string_to_check).0);

}

#[tauri::command]
pub fn build_parse_tree(regex: &str) -> Result<Token, ParsingError> {
  let (tokenized_expression, _) = tokenize_regular_expression(regex);
  let parse_tree = parse_tokens(tokenized_expression)?;
  verify_syntactic_correctness_of_parse_tree(&parse_tree)?;
  return Ok(parse_tree);
}

fn convert_parse_tree_to_nfa(
  state_positions: &mut HashMap<String, State>, 
  current_state_coords: Coordinate, 
  token_to_convert: Token, 
  end_state_coords: Coordinate) {

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
      panic!("Grouped expressions must be evaluated in the parse tree step!")
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

      convert_parse_tree_to_nfa(state_positions, current_state_coords, left_token.to_owned(), end_state_coords);
      convert_parse_tree_to_nfa(state_positions, current_state_coords, right_token.to_owned(), end_state_coords);
    },

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
        
        current_state.add_connection("ϵ", new_state_coords);

        let mut new_state = State::new(new_state_coords, false, false);
        new_state.add_connection("ϵ", end_state_coords);
        state_positions.insert(new_state_coords.into(), new_state);

        handle_kleene_token_to_nfa_conversion(current_state_coords, state_positions, 
          inner_argument.to_owned(), new_state_coords);

    },
    Token::ConcatenatedExpression(concatenated_expression) => {
      // We are once again assuming the parse tree has been accurately constructed such that
      // concatenated expressions have both some arguments
      let left_token = concatenated_expression.get_left_argument().cloned();
      let right_token = concatenated_expression.get_right_argument().cloned();

      let right_token_start_state_coords = create_unique_state_coordinates(&state_positions.keys().cloned().collect());

      state_positions
        .insert(right_token_start_state_coords.into(), 
        State::new(right_token_start_state_coords, false, false));

      convert_parse_tree_to_nfa(state_positions, 
        current_state_coords, 
        left_token.unwrap(), 
        right_token_start_state_coords);

      convert_parse_tree_to_nfa(state_positions, 
        right_token_start_state_coords, 
        right_token.unwrap(),
        end_state_coords);

    }

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

      let new_state = state_positions
        .get_mut::<String>(&new_state_coords.into())
        .expect("Failed to retrieve the requested state");

      new_state
        .add_connection("ϵ", current_state_coords);
      
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
      Token::ConcatenatedExpression(concatenated_expression)=> {
        let left_token = concatenated_expression.get_left_argument().cloned();
        let right_token = concatenated_expression.get_right_argument().cloned();
  
        let right_token_start_state_coords = create_unique_state_coordinates(&state_positions.keys().cloned().collect());
  
        state_positions
          .insert(right_token_start_state_coords.into(), 
          State::new(right_token_start_state_coords, false, false));
  
        convert_parse_tree_to_nfa(state_positions, 
          current_state_coords, 
          left_token.unwrap(),
           right_token_start_state_coords);
  
        convert_parse_tree_to_nfa(state_positions, 
          right_token_start_state_coords, 
          right_token.unwrap(),
          coords_of_state_to_loop_to);

      }
  }
 

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
          return Err(ParsingError::EmptyLeftArg);
        } else {
          return Err(ParsingError::EmptyRightArg);
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


// Checks for grouped expression in list of tokens and returns index and owned copy of first grouped expression if found
fn does_contain_grouped_expression(tokens: &Vec<Token>) -> Option<(Token, usize)> {

  for (index, token) in tokens.into_iter().enumerate() {
    match token {
      Token::GroupedExpression(_) => return Some((token.to_owned(), index)),
      _ => continue
    }

  }

  return None;

}

// Checks for kleene token in list of tokens and returns index and owned copy of first grouped expression if found
fn does_contain_kleene_operator(tokens: &Vec<Token>) -> Option<(Token, usize)> {

  for (index, token) in tokens.into_iter().enumerate() {
    match token {
      Token::KleeneOperator(kleene_operator) => {
        // If a kleene operator is already filled we continue
        if kleene_operator.has_empty_arg() {
          return Some((token.to_owned(), index))
        }
      },
        
      _ => continue
    }

  }

  return None;

}

fn parse_tokens(mut tokens: Vec<Token>) -> Result<Token, ParsingError> {
  // Must give priority to grouped expressions
  // Parse grouped expressions first?
  if tokens.len() == 0 {
    return Err(ParsingError::NoInnerArg)
  } else if tokens.len() == 1 {
    match tokens.get(0).expect("The array should have at least a single element") {
      // If it's a grouped expression, do nothing and continue breaking it apart
      Token::GroupedExpression(_) => (),
      _ => {
        return Ok(concatenate_tokens(tokens))
      }
    }
  } else if !can_continue_parsing(&tokens) {
    return Ok(concatenate_tokens(tokens));
  }

  // parsing all regular expressions into their proper form FIRST prior to any operations
  let mut has_grouped_expression = does_contain_grouped_expression(&tokens);
  while has_grouped_expression.is_some() {
    let (grouped_expression, index) = has_grouped_expression.unwrap();
    tokens
      .remove(index);
    match grouped_expression {
      Token::GroupedExpression(grouped_expression) => {
        let parsed_grouped_expression = parse_tokens(*grouped_expression)?;
        tokens.insert(index, parsed_grouped_expression);
      },
      _ => panic!("The supplied token should be a grouped expression!")
    }
    has_grouped_expression = does_contain_grouped_expression(&tokens);
  };

  let mut has_kleene_token = does_contain_kleene_operator(&tokens);
  while has_kleene_token.is_some() {
    let (kleene_token, index) = has_kleene_token.unwrap();
    
    match kleene_token {
      Token::KleeneOperator(mut kleene_operator) => {
        let left_token = tokens
        .get(index.checked_sub(1).ok_or_else( || {ParsingError::NoneTokenProvided})?)
        .cloned();

      kleene_operator
        .insert_token(left_token)?;

      tokens.drain(index-1..=index);

      tokens.insert(index-1, Token::KleeneOperator(kleene_operator));
      },
      _ => panic!("The supplied token should be a grouped expression!")
    }
    has_kleene_token = does_contain_kleene_operator(&tokens);
  };

  // Very gross code will definitely be rewriten in future
  let mut finished = false;
  
  while !finished {
    let duplicate_tokens = tokens.clone();
    finished = true;
    for (index, token) in duplicate_tokens.into_iter().enumerate() {
      match token {
        Token::OrOperator(mut current_or_op) => {
          if !current_or_op.has_empty_arg() {
            continue;
          }
          finished = false;
  
          let left_token = tokens
            .get(index.checked_sub(1).ok_or_else( || {ParsingError::NoneTokenProvided})?)
            .cloned();
    
          let right_token = tokens
            .get(index + 1)
            .cloned();
  
          current_or_op
            .left_insert_token(left_token)?;
          current_or_op
            .right_insert_token(right_token)?;
  
          tokens.drain(index - 1..=index + 1);
          tokens.insert(index - 1, Token::OrOperator(current_or_op));
          break;
  
        },

        Token::GroupedExpression(_) => panic!("All grouped expressions should be parsed prior to this step"),
        _ => continue
      }
    
    };

  }  
  
  return parse_tokens(tokens);

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
    return Token::ConcatenatedExpression(Box::new(ConcatenatedExpression::new(
      Some(first_token.to_owned()), 
      Some(second_token.to_owned())
    )));
  }

  let midpoint = tokens.len().div_ceil(2);
  let first_half_of_tokens = &tokens[..midpoint];
  let second_half_of_tokens = &tokens[midpoint..];

  return Token::ConcatenatedExpression(Box::new(ConcatenatedExpression::new(
    Some(concatenate_tokens(first_half_of_tokens.to_owned())), 
    Some(concatenate_tokens(second_half_of_tokens.to_owned()))
  )));

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