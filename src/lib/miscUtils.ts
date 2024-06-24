import type { Coordinate } from "./interfaces";

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