import { invoke } from "@tauri-apps/api";
import { dialogue_to_user, list_of_connections, state_positions, type_of_automata } from "./automataStores";
import { get } from "svelte/store";
import { Automata } from "../types/enums";

export const saveWorkspace = async (email: string | undefined, workspace_name: string | undefined) => {
  if (!email || !workspace_name) {
    return;
  }
  if (workspace_name === "Untitled Project") {
    dialogue_to_user.set("You must set the name of the project to save.");
    return;
  }
  dialogue_to_user.set("");

  await invoke("save_workspace", {
    workspaceName: workspace_name,
    email: email,
    states: get(state_positions),
    connections: get(list_of_connections),
    typeOfAutomata: Automata[get(type_of_automata)]
  });
};
