import { invoke } from "@tauri-apps/api";
import { Automata } from "../types/enums";
import type { CheckedStringResponse, State } from "../types/interfaces";

export const checkInputtedString = async (
  start_state_coordinates: string | null,
  type_of_automata: Automata,
  state_positions: Map<string, State>,
  string_to_check: string | null,
  is_strict_checking: boolean,
  input_alphabet: Array<string>
  ): Promise<CheckedStringResponse> => {

  const is_dfa_valid: boolean = await checkValidityOfDFA(is_strict_checking, type_of_automata, state_positions, input_alphabet);
  if(!is_dfa_valid){
    return {
      dialogue: `Your DFA either does not specify every connection provided in the input alphabet, or specifies them more than once.
        Update the model or disable strict checking`,
      is_string_accepted: null,
      states_traversed: []
    };
  }
  if(!start_state_coordinates || !string_to_check) {
    return {
      dialogue: "",
      is_string_accepted: null,
      states_traversed: []
    };
  }
  // No feedback message needs to be displayed if the automata in question is valid
  let is_string_accepted_after_test: boolean;
  let states_traversed_after_test: Array<State>;
  switch (type_of_automata) {
    // Setting the states traversed when checking 
    // the string and displaying to the user whether the string was accepted
    case Automata.DFA:
      [is_string_accepted_after_test, states_traversed_after_test] = await invoke("test_string_dfa", {
        statePositions: state_positions,
        startStateCoordinates: start_state_coordinates,
        stringToCheck: string_to_check,
      });

      return {
        dialogue: "",
        is_string_accepted: is_string_accepted_after_test,
        states_traversed: states_traversed_after_test
      };

    case Automata.NFA:
      [is_string_accepted_after_test, states_traversed_after_test] = await invoke("test_string_nfa", {
        statePositions: state_positions,
        startStateCoordinates: start_state_coordinates,
        stringToCheck: string_to_check,
      });
      
      return {
        dialogue: "",
        is_string_accepted: is_string_accepted_after_test,
        states_traversed: states_traversed_after_test
      };
  }
}

const checkValidityOfDFA = async (
  is_strict_checking: boolean,
  type_of_automata: Automata,
  state_positions: Map<string, State>,
  input_alphabet: Array<string>
): Promise<boolean> => {
  // If we are not strict checking, or we are not testing a DFA, we should not alert the program
  // if the automata is not strictly valid
  if(!is_strict_checking || type_of_automata !== Automata.DFA){
    return true;
  }
  return await invoke("verify_valid_dfa", {
    stateConnections: state_positions,
    inputAlphabet: input_alphabet,
  });

}