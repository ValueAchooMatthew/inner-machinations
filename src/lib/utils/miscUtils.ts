// Values the pages pass to each other for the current session (the open workspace).
// These used to be cookies, but WebKitGTK drops cookies on Tauri 2's tauri://localhost origin
// on Linux. sessionStorage works on every platform and, like those cookies, clears when the app closes.
export function getSessionValue(name: string): string {
  if (typeof sessionStorage === "undefined") {
    return "";
  }
  return sessionStorage.getItem(name) ?? "";
}

export function setSessionValue(name: string, value: string): void {
  sessionStorage.setItem(name, value);
}

export function clearSessionValue(name: string): void {
  sessionStorage.removeItem(name);
}

// The name, or if a workspace has it already, the name numbered from 2 up
export function firstFreeName(name: string, taken_names: Array<string>): string {
  const taken = new Set(taken_names);
  if (!taken.has(name)) {
    return name;
  }
  let number = 2;
  while (taken.has(`${name} ${number}`)) {
    number++;
  }
  return `${name} ${number}`;
}
