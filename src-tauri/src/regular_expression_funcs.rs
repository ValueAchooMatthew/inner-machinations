mod regex_models;
pub mod saving_regex_funcs;
use std::collections::HashMap;

use app::{create_unique_state_coordinates, remove_all_epsilon_transitions};
use regex_models::{BinaryOperator, KleeneOperator, OrOperator, ParsingError, Token, UnaryOperator, TokenArray, TokenArrayMethods};

use app::models::{State, Coordinate};

use crate::{advanced_automata_funcs::reconstruct_nfa_state_positions, testing_automata_funcs::test_string_nfa};
mod tests;

#[tauri::command(rename_all = "snake_case")]
pub fn test_string_regex(regex: &str, string_to_check: &str) -> bool {

  let parse_tree = match build_parse_tree(regex) {
    Ok(parse_tree) => parse_tree,
    Err(_) => {return false;}
  };

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

  let start_state_key: String = start_state_coords.into();

  let state_positions = reconstruct_nfa_state_positions(&state_positions, &start_state_key);

  return test_string_nfa(state_positions, start_state_coords.into(), string_to_check).0;

}

#[tauri::command(rename_all = "snake_case")]
pub fn build_parse_tree(regex: &str) -> Result<Token, ParsingError> {
  let (tokenized_expression, _) = tokenize_regular_expression(regex)?;
  let parse_tree = tokenized_expression.parse_tokens()?;
  parse_tree.verify_syntactic_correctness()?;
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
        State::new(right_token_start_state_coords, false, false)
      );

      convert_parse_tree_to_nfa(state_positions, 
        current_state_coords, 
        left_token.unwrap(), 
        right_token_start_state_coords
      );

      convert_parse_tree_to_nfa(state_positions, 
        right_token_start_state_coords, 
        right_token.unwrap(),
        end_state_coords
      );
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