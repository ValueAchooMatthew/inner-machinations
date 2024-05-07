export interface State{
    x_pos: number,
    y_pos: number,
    element: "State",
};

export interface Arrow {
    x1_pos: number,
    y1_pos: number,
    x2_pos: number,
    y2_pos: number,
    element: "Connection"
    character: string;
};

export interface StateConnection {
    // Specifying coordinates of node as an identification for a node
    nodes_connected_to: Array<string>,
    nodes_connected_from: Array<string>,
    connection_chars: Array<string>,
    is_final_state: boolean,
};
