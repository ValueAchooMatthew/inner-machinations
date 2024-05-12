import type { BezierCurve, Connection, Coordinate } from "./interfaces";

export const roundToNearest = (number_to_round: number, rounded_to: number): number => {
    const remainder = number_to_round % rounded_to;
    const half = Math.floor(rounded_to / 2);
    if(remainder >= half){
        return number_to_round + (rounded_to - remainder)
    }else{
        return number_to_round - remainder
    }

}

export const getClosestPointIndex = (possible_points: Array<Coordinate>, origin: Coordinate): number => {
    let closestPointIndex = 0;
    let closestPointDistance = distanceBetweenTwoPoints(possible_points[0], origin);
    possible_points.forEach((point, index)=>{
        const distance = distanceBetweenTwoPoints(point, origin);
        if(distance < closestPointDistance){
            closestPointIndex = index;
            closestPointDistance = distance;
        }
    })
    return closestPointIndex;

}

const distanceBetweenTwoPoints = (point_a: Coordinate, point_b: Coordinate): number => {
    return Math.sqrt((point_a.x - point_b.x)**2 + (point_a.y - point_b.y)**2)

}

// Returns index of line in array of the line closest to the given x, y coords
// TODO: change to work with bezier curves instead
export const indexOfClosestLineToPoint = (x_pos: number, y_pos: number, connections: Array<Connection>): number => {
    let indexOfClosestLine = 0;
    let minimumDistance = distanceFromLineToPoint(x_pos, y_pos, connections[0].curve);
    connections.forEach((connection, index)=>{
        const distance = distanceFromLineToPoint(x_pos, y_pos, connection.curve);
        if(distance < minimumDistance){
            indexOfClosestLine = index;
            minimumDistance = distance
        };
    });

    return indexOfClosestLine;

}

const distanceFromLineToPoint = (x_pos: number, y_pos: number, curve: BezierCurve): number => {
    // y = mx + b (slope intercept form)
    // Ax + By + C = 0 (standard form) (C = -b, A = -slope)
    // Gigantic thanks  to this answer from stackoverflow answer https://stackoverflow.com/a/1501725 for easy to follow solution
    // To this problem
    const x1 = curve.start_point.x;
    const y1 = curve.start_point.y;
    const x2 = curve.end_point.x;
    const y2 = curve.end_point.y;

    const l2 = (x1 - x2)**2 + (y1 - y2)**2;
    if (l2 == 0){
        return Math.sqrt((x1 - x_pos)**2 + (y1 - y_pos)**2);
    }
    let t = ((x_pos - x1) * (x2 - x1) + (y_pos - y1) * (y2 - y1)) / l2;
    t = Math.max(0, Math.min(1, t));

    return Math.sqrt((x_pos - (x1 + t*(x2 - x1)))**2 + (y_pos - (y1 + t*(y2 - y1)))**2); 

}

// The formulas used to calculate the angle of the arrowhead are taken from this stackoverflow answer
// https://stackoverflow.com/a/21053913
// Gigantic help and incredibly impressive

export const getBezierCurveAngleAtPoint = (curve: BezierCurve, distance_along_curve: number): number => {
    const pointNearEnd = getPointOnBezierCurveAtDistance(curve, distance_along_curve);
    const distanceToEndX = curve.end_point.x - pointNearEnd.x;
    const distanceToEndY = curve.end_point.y - pointNearEnd.y;
    const angle = Math.atan2(distanceToEndY, distanceToEndX);
    return angle;

}

export const getPointOnBezierCurveAtDistance = (curve: BezierCurve, distance: number): Coordinate => {
    const x = getCoordOnBezierCurve(distance, curve.start_point.x, curve.control_point_one.x, 
    curve.control_point_two.x, curve.end_point.x);
    
    const y = getCoordOnBezierCurve(distance, curve.start_point.y, curve.control_point_one.y,
    curve.control_point_two.y, curve.end_point.y);
    return {x, y};

}

const getCoordOnBezierCurve = (t: number, a: number, b: number, c: number, d: number) => {
    const tSquared = t*t;
    const tCubed = tSquared*t;

    return a + (-a * 3 + t * (3 * a - a * t)) * t
    + (3 * b + t * (-6 * b + b * 3 * t)) * t
    + (c * 3 - c * 3 * t) * tSquared
    + d * tCubed;

}   