import { invoke } from "@tauri-apps/api";
import { dialogue_to_user, input_alphabet, list_of_connections, 
state_positions, type_of_automata, email, workspace_name, 
should_strict_check,
should_show_string_traversal,
default_connection_char} from "./automataStores";
import { get } from "svelte/store";
import { Automata } from "../types/enums";
import { convertFormDataEntriesToStringArray } from "./miscUtils";

export const saveWorkspace = async () => {

  if (get(workspace_name) === "Untitled Project") {
    dialogue_to_user.set("You must set the name of the project to save.");
    return;
  }
  dialogue_to_user.set("");

  await invoke("save_workspace", {
    workspaceName: workspace_name,
    email: get(email),
    states: get(state_positions),
    connections: get(list_of_connections),
    typeOfAutomata: Automata[get(type_of_automata)],
    alphabet: get(input_alphabet)
  });
};

export async function saveOptions(form: HTMLFormElement | undefined) {
  // We are assuming the strict checking and traversal stores have both been
  // set by the option menu component but input alphabet and default char 
  // have not as they require additional processing and are more difficult cases
  // to handle
  if(!form) {
    return;
  }
  const data = new FormData(form);
  const alphabet = data.getAll("alphabet");
  const stringified_array = convertFormDataEntriesToStringArray(alphabet);
  // This invoke both saves the alphabet and returns it's sanitized form which was the form in
  // which it was saved to db
  const sanitized_alphabet: Array<string> = await invoke("update_workspace_alphabet", {
    workspaceName: get(workspace_name), email: get(email), alphabet: stringified_array
  });
  input_alphabet.set(sanitized_alphabet);

  await invoke("update_showing_string_traversal", 
    {email: get(email), workspaceName: get(workspace_name), shouldStrictCheck: get(should_strict_check)});

  await invoke("update_strict_checking", 
    {email: get(email), workspaceName: get(workspace_name), shouldShowTraversal: get(should_show_string_traversal)});

  const new_default_connection_character = data.get("default_character")?.toString();
  
  if(!new_default_connection_character) {
    return;
  }

  await invoke("update_default_connection_character", 
    {email: get(email), workspaceName: get(workspace_name), defaultConnectionCharacter: new_default_connection_character});
  
  default_connection_char.set(new_default_connection_character);
}