import { invokeCommand } from "$lib/utils/backend";
import { Automata } from "../types/enums";
import type { Automaton } from "$lib/workbench/automaton";
import { positionKey, startStateKey, toStatePositions, type BackendState } from "$lib/workbench/api";

export interface CheckedStringResponse {
  dialogue: string,
  // The reason we provide null as an option rather than using a boolean is because if the automata being tested is invalid,
  // We do not want to give a response back to the user indicating the string is either accepted OR rejected since it's not possible
  // to tell if we cannot test the automata in the first place.
  is_string_accepted: boolean | null,
  // The ids of the states the string passed through, in order, for showing the traversal
  states_traversed: Array<number>
}

export const checkInputtedString = async (
  automaton: Automaton,
  type_of_automata: Automata,
  string_to_check: string,
  should_strict_check: boolean,
  input_alphabet: Array<string>
  ): Promise<CheckedStringResponse> => {

  const is_dfa_valid: boolean = await checkValidityOfDFA(should_strict_check, type_of_automata, automaton, input_alphabet);
  if(!is_dfa_valid){
    return {
      dialogue: `Your DFA either has no start state, does not specify every connection provided in the input alphabet, or specifies them more than once.
        Update the model or disable strict checking`,
      is_string_accepted: null,
      states_traversed: []
    };
  }
  const start_state_coordinates = startStateKey(automaton);
  if(!start_state_coordinates) {
    return {
      dialogue: "You must specify at least one start state",
      is_string_accepted: null,
      states_traversed: []
    };
  }

  const command = type_of_automata === Automata.DFA ? "test_string_dfa" : "test_string_nfa";
  const [is_string_accepted, states_traversed]: [boolean, Array<BackendState>] = await invokeCommand(command, {
    statePositions: toStatePositions(automaton),
    startStateCoordinates: start_state_coordinates,
    stringToCheck: string_to_check,
  });

  // The backend names states by position
  const id_of_position = new Map(automaton.states.map((state) => [positionKey(state.position), state.id]));
  return {
    dialogue: "",
    is_string_accepted,
    states_traversed: states_traversed.flatMap((state) => id_of_position.get(positionKey(state.position)) ?? []),
  };
}

const checkValidityOfDFA = async (
  should_strict_check: boolean,
  type_of_automata: Automata,
  automaton: Automaton,
  input_alphabet: Array<string>
): Promise<boolean> => {
  // If we are not strict checking, or we are not testing a DFA, we should not alert the program
  // if the automata is not strictly valid
  if(!should_strict_check || type_of_automata !== Automata.DFA){
    return true;
  }
  return await invokeCommand("verify_valid_dfa", {
    statePositions: toStatePositions(automaton),
    inputAlphabet: input_alphabet,
    startStatePosition: startStateKey(automaton),
  });

}
