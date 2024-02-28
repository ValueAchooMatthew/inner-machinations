export interface State{
    x_pos: number,
    y_pos: number,
    element: "State",
};

export interface StartState extends State{
    start: true
}


export interface EndState{
    end: true

}


export interface Connection {
    x1_pos: number,
    y1_pos: number,
    x2_pos: number,
    y2_pos: number,
    element: "Connection"
};

export interface Link{
    // Specifying coordinates of node as an identification for a node
    startNode: [number, number]
    nextNode: [number, number],
    character: string
};
