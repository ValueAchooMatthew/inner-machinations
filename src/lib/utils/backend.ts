import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import { dialogue_to_user } from "./automataStores";

// A command failed and the user has been shown why, so whoever was waiting on it can let it go
export class ReportedError extends Error {}

// Commands send a message saying why they failed
export const describeFailure = (error: unknown): string => {
  if (typeof error === "string") {
    return error;
  }
  if (error instanceof Error) {
    return error.message;
  }
  return String(error);
};

// Runs a backend command. If it fails, its message is shown in the notification area, and the promise
// rejects with a ReportedError so the caller stops there
export async function invokeCommand<T = unknown>(command: string, args?: InvokeArgs): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    const message = describeFailure(error);
    dialogue_to_user.set(message);
    throw new ReportedError(message);
  }
}
