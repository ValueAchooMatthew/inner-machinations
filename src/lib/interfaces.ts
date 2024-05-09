// Reworked so every state has a reference to it's position, to allow for a single state interface for use
// In both drawing states to the canvas and connecting data in the rust
export interface State {
    // Specifying coordinates of node as an identification for a node
    x_pos: number,
    y_pos: number
    nodes_connected_to: Array<string>,
    nodes_connected_from: Array<string>,
    connection_chars: Array<string>,
    is_final: boolean,
    element: "State"
};


export interface Arrow {
    x1_pos: number,
    y1_pos: number,
    x2_pos: number,
    y2_pos: number,
    character: string;
    element: "Connection"
};