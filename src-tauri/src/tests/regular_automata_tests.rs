
#[cfg(test)]
pub mod tests {
  use crate::{miscellaneous::{common_models::State, miscellaneous_utilities::create_state_positions_from_states}, 
  regular_automata_funcs::{regular_automata_linguistics::test_string_dfa, 
  regular_automata_models::{LoopHandling, LoopingTransitionErrors, RegularAutomataTransition}}};

  #[test]
  fn test_dfa_string_checking() {
    let mut state_positions;
    
    let s1 = State::new("0,0", true, true);
    
    state_positions = create_state_positions_from_states(vec![&s1]);

    assert!(test_string_dfa(state_positions.to_owned(), "0,0", "").0);
    
    let s1 = State::new("0,0", true, false);
    
    state_positions = create_state_positions_from_states(vec![&s1]);

    assert!(!test_string_dfa(state_positions.to_owned(), "0,0", "").0);

    let mut s1 = State::new("0,0", true, false);
    let s2 = State::new("1,1", false, true);

    s1.add_connection("a", "1,1");

    state_positions = create_state_positions_from_states(vec![&s1, &s2]);

    assert!(test_string_dfa(state_positions.to_owned(), "0,0", "a").0);
    assert!(!test_string_dfa(state_positions.to_owned(), "0,0", "").0);
    assert!(!test_string_dfa(state_positions.to_owned(), "0,0", "aa").0);
    assert!(!test_string_dfa(state_positions.to_owned(), "0,0", "b").0);

  }

  #[test]
  fn test_transition_looping_validation() {
    let transitions = vec![
      RegularAutomataTransition::new("1,1", "a", "2,2"),
      RegularAutomataTransition::new("2,2", "a", "3,3")
    ];

    assert_eq!(transitions.do_transitions_form_valid_loop(), Err(LoopingTransitionErrors::TransitionsDoNotFormLoop));

    let transitions = vec![
      RegularAutomataTransition::new("1,1", "a", "0,0"),
      RegularAutomataTransition::new("0,0", "a", "2,2"),
      RegularAutomataTransition::new("0,0", "a", "1,1")
    ];

    assert_eq!(transitions.do_transitions_form_valid_loop(), Err(LoopingTransitionErrors::TransitionsNotContinuous));

    let transitions = vec![
      RegularAutomataTransition::new("0,0", "a", "1,1"),
      RegularAutomataTransition::new("1,1", "a", "2,2"),
      RegularAutomataTransition::new("2,2", "a", "0,0")
    ];
    
    assert_eq!(transitions.do_transitions_form_valid_loop(), Ok(()));


  }

  #[test]
  fn test_transition_looping_equality() {
    let transitions_one = vec![
      RegularAutomataTransition::new("0,0", "a", "1,1"),
      RegularAutomataTransition::new("1,1", "a", "2,2"),
      RegularAutomataTransition::new("2,2", "a", "0,0")
    ];

    let transitions_two = vec![
      RegularAutomataTransition::new("1,1", "a", "2,2"),
      RegularAutomataTransition::new("2,2", "a", "0,0"),
      RegularAutomataTransition::new("0,0", "a", "1,1")
    ];

    assert!(transitions_one.are_loops_equivalent(&transitions_two).unwrap());

    let transitions_two = vec![
      RegularAutomataTransition::new("1,1", "a", "2,2"),
      RegularAutomataTransition::new("2,2", "a", "1,1")
    ];

    assert!(!transitions_one.are_loops_equivalent(&transitions_two).unwrap());

  }

}