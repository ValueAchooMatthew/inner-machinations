use std::collections::HashMap;

use crate::{miscellaneous::{common_models::{Coordinate, State}, miscellaneous_utilities::{create_unique_state_coordinates, remove_all_epsilon_transitions}}, 
regular_automata_funcs::{regular_automata_extra_features::reconstruct_nfa_state_positions, regular_automata_linguistics::test_string_dfa}};

use super::{regular_expression_models::{Token, BinaryOperator, UnaryOperator}, regular_expression_parsing::build_parse_tree};

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

  return test_string_dfa(state_positions, &start_state_key, string_to_check).0;

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