// Reworked so every state has a reference to it's position, to allow for a single state interface for use
// In both drawing states to the canvas and connecting data in the rust
export interface State {
    // Specifying coordinates of node as an identification for a node
    position: Coordinate,
    states_connected_to: Map<String, Array<String>>,
    is_start: boolean,
    is_final: boolean,
    element: "State"
};

export interface Connection {
    curve: BezierCurve,
    connection_character: string,
    element: "Connection"
};

export interface BezierCurve {
    start_point: Coordinate,
    // Coordinates for control points for drawing bezier curves
    control_point_one: Coordinate,
    control_point_two: Coordinate,
    end_point: Coordinate

};

export interface Coordinate {
    x: number,
    y: number
};
