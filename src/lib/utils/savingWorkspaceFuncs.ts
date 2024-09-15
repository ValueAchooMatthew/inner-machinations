import { invoke } from "@tauri-apps/api";
import { dialogue_to_user, input_alphabet, list_of_connections, 
state_positions, email, workspace_name, should_strict_check, should_show_string_traversal,
default_connection_character } from "./svelteStores";
import { get } from "svelte/store";
import { convertFormDataEntriesToStringArray } from "./miscUtils";

export const saveWorkspace = async () => {

  dialogue_to_user.set(null);

  await invoke("save_regular_automata_workspace", {
    workspace_name: get(workspace_name),
    email: get(email),
    states: get(state_positions),
    connections: get(list_of_connections),
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
  const sanitized_alphabet: Array<string> = await invoke("update_regular_automata_workspace_alphabet", {
    workspace_name: get(workspace_name), email: get(email), alphabet: stringified_array
  });
  input_alphabet.set(sanitized_alphabet);

  await invoke("update_strict_checking", 
    {email: get(email), workspace_name: get(workspace_name), should_strict_check: get(should_strict_check)});

  await invoke("update_showing_string_traversal", 
    {email: get(email), workspace_name: get(workspace_name), should_show_traversal: get(should_show_string_traversal)});

  const new_default_connection_character = data.get("default_character")?.toString();
  
  if(!new_default_connection_character) {
    return;
  }

  await invoke("update_default_connection_character", 
    {email: get(email), workspace_name: get(workspace_name), default_connection_character: new_default_connection_character});
  
  default_connection_character.set(new_default_connection_character);
}