import type { Coordinate } from "../types/interfaces";

export const convertCoordinateToString = (coordinate: Coordinate): string => {
  return coordinate.x + "," + coordinate.y;
};

export const removeFirstElementFromArray = <T>(list: Array<T>, element_to_remove: T): Array<T> => {

  const index_of_element_to_remove = list.indexOf(element_to_remove);
  if(index_of_element_to_remove < 0){
    // negative index implies element is not there, and we thus return the original array
    return list;
  }
  // Splicing the index at the index of the element will return the array except the element at the index
  // because le javascript is weird
  list.splice(index_of_element_to_remove, 1);
  return list;
}
// Values the pages pass to each other for the current session (logged-in email, open workspace).
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


export function convertFormDataEntriesToStringArray(form_data: Array<FormDataEntryValue>): Array<string> {

  const stringified_array = form_data.map((data)=> {
    return data.toString()
  })

  return stringified_array;
}