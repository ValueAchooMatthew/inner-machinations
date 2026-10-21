import { invokeCommand } from "$lib/utils/backend";
import { automaton, dialogue_to_user, input_alphabet, workspace_name, should_strict_check, should_show_string_traversal,
default_connection_character } from "./automataStores";
import { get } from "svelte/store";
import { convertFormDataEntriesToStringArray } from "./miscUtils";

// Saves the automaton and the alphabet. The backend keeps the alphabet's distinct single characters
export const saveWorkspace = async () => {

  dialogue_to_user.set(null);

  const saved_alphabet: Array<string> = await invokeCommand("save_workspace", {
    workspaceName: get(workspace_name),
    automaton: get(automaton),
    alphabet: get(input_alphabet)
  });
  input_alphabet.set(saved_alphabet);
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
  const sanitized_alphabet: Array<string> = await invokeCommand("update_workspace_alphabet", {
    workspaceName: get(workspace_name), alphabet: stringified_array
  });
  input_alphabet.set(sanitized_alphabet);

  await invokeCommand("update_strict_checking",
    {workspaceName: get(workspace_name), shouldStrictCheck: get(should_strict_check)});

  await invokeCommand("update_showing_string_traversal",
    {workspaceName: get(workspace_name), shouldShowTraversal: get(should_show_string_traversal)});

  const new_default_connection_character = data.get("default_character")?.toString();

  if(!new_default_connection_character) {
    return;
  }

  await invokeCommand("update_default_connection_character",
    {workspaceName: get(workspace_name), defaultConnectionCharacter: new_default_connection_character});

  default_connection_character.set(new_default_connection_character);
}
