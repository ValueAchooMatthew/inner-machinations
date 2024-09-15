import type { BezierCurve, Coordinate } from "$lib/types/interfaces";
import { list_of_regular_automata_connections } from "./regularAutomataStores";

export const roundToNearest = (
  number_to_round: number,
  rounded_to: number,
): number => {
  const remainder = number_to_round % rounded_to;
  const half = Math.floor(rounded_to / 2);
  if (remainder >= half) {
    return number_to_round + (rounded_to - remainder);
  } else {
    return number_to_round - remainder;
  }
};

export const getClosestPointIndex = (
  possible_points: Array<Coordinate>,
  origin: Coordinate,
): number => {
  let closestPointIndex = 0;
  let closestPointDistance = distanceBetweenTwoPoints(
    possible_points[0],
    origin,
  );
  possible_points.forEach((point, index) => {
    const distance = distanceBetweenTwoPoints(point, origin);
    if (distance < closestPointDistance) {
      closestPointIndex = index;
      closestPointDistance = distance;
    }
  });
  return closestPointIndex;
};

const distanceBetweenTwoPoints = (
  point_a: Coordinate,
  point_b: Coordinate,
): number => {
  return Math.sqrt((point_a.x - point_b.x) ** 2 + (point_a.y - point_b.y) ** 2);
};

// Returns index of RegularAutomataConnection in array of the curve closest to the given x, y coords
// The math for finding the exact closest curve would be too computationally intensive, and instead, and average of all of the points
// Making up the curve is used to roughly estimate its position
export const indexOfClosestBezierCurveToPoint = (
  origin: Coordinate,
): number => {

  let connections = new Array();
  list_of_regular_automata_connections.subscribe((value)=>{
    connections = value;
  });

  let indexOfClosestLine = 0;
  let minimumDistance = distanceFromBezierCurveToPoint(
    origin,
    connections[0].curve,
    10,
  );
  connections.forEach((RegularAutomataConnection, index) => {
    const distance = distanceFromBezierCurveToPoint(
      origin,
      RegularAutomataConnection.curve,
      10,
    );
    if (distance < minimumDistance) {
      indexOfClosestLine = index;
      minimumDistance = distance;
    }
  });

  return indexOfClosestLine;
};

// Roughly estimate's a curve's distance to a given point by taking small slices of the bezier curve and returning the distance of the point
// of the slice to the given coordinate
// https://stackoverflow.com/a/34520607 Done with help of this stackoverflow answer
const distanceFromBezierCurveToPoint = (
  origin: Coordinate,
  curve: BezierCurve,
  slices: number,
): number => {
  const tick = 1 / slices;
  let minimum_distance = Infinity;
  for (let i = 0; i <= slices; i++) {
    const t = i * tick;
    // Should be refactored into separate function later
    const x = getCoordOnBezierCurve(
      t,
      curve.start_point.x,
      curve.control_point_one.x,
      curve.control_point_two.x,
      curve.end_point.x,
    );

    const y = getCoordOnBezierCurve(
      t,
      curve.start_point.y,
      curve.control_point_one.y,
      curve.control_point_two.y,
      curve.end_point.y,
    );

    const distance = Math.sqrt((origin.x - x) ** 2 + (origin.y - y) ** 2);
    if (distance < minimum_distance) {
      minimum_distance = distance;
    }
  }
  return minimum_distance;
};

// The formulas used to calculate the angle of the arrowhead are taken from this stackoverflow answer
// https://stackoverflow.com/a/21053913
// Gigantic help and incredibly impressive
export const getBezierCurveAngleAtPoint = (
  curve: BezierCurve,
  distance_along_curve: number,
): number => {
  const pointNearEnd = getPointOnBezierCurveAtDistance(
    curve,
    distance_along_curve,
  );
  const distanceToEndX = curve.end_point.x - pointNearEnd.x;
  const distanceToEndY = curve.end_point.y - pointNearEnd.y;
  const angle = Math.atan2(distanceToEndY, distanceToEndX);
  return angle;
};

export const getPointOnBezierCurveAtDistance = (
  curve: BezierCurve,
  distance: number,
): Coordinate => {
  const x = getCoordOnBezierCurve(
    distance,
    curve.start_point.x,
    curve.control_point_one.x,
    curve.control_point_two.x,
    curve.end_point.x,
  );

  const y = getCoordOnBezierCurve(
    distance,
    curve.start_point.y,
    curve.control_point_one.y,
    curve.control_point_two.y,
    curve.end_point.y,
  );
  return { x, y };
};

const getCoordOnBezierCurve = (
  t: number,
  a: number,
  b: number,
  c: number,
  d: number,
) => {
  const tSquared = t * t;
  const tCubed = tSquared * t;

  return (
    a +
    (-a * 3 + t * (3 * a - a * t)) * t +
    (3 * b + t * (-6 * b + b * 3 * t)) * t +
    (c * 3 - c * 3 * t) * tSquared +
    d * tCubed
  );
};
