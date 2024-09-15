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
// Provided by W3 schools and used as page.server files don't seem to work in prod
export function getCookie(cname: string): string {
  let name = cname + "=";
  let decodedCookie = decodeURIComponent(document.cookie);
  let ca = decodedCookie.split(';');
  for(let i = 0; i < ca.length; i++) {
    let c = ca[i];
    while (c.charAt(0) == ' ') {
      c = c.substring(1);
    }
    if (c.indexOf(name) == 0) {
      return c.substring(name.length, c.length);
    }
  }
  return "";
}

export function convertFormDataEntriesToStringArray(form_data: Array<FormDataEntryValue>): Array<string> {

  const stringified_array = form_data.map((data)=> {
    return data.toString()
  })

  return stringified_array;
}