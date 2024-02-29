export interface State{
    x_pos: number,
    y_pos: number,
    element: "State",
    final: boolean
};

export interface Connection {
    x1_pos: number,
    y1_pos: number,
    x2_pos: number,
    y2_pos: number,
    element: "Connection"
};

export interface Node{
    // Specifying coordinates of node as an identification for a node
    connected_nodes: Array<Node>,
    connection_chars: Array<string>,
    final: boolean,

};
