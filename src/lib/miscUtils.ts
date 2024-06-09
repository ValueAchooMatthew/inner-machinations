import type { Coordinate } from "./interfaces";

export const convertCoordinateToString = (coordinate: Coordinate): string => {
  return coordinate.x + "," + coordinate.y;
};
