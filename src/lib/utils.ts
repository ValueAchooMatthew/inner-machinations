import type { State, Arrow, StateConnection, } from "./interfaces";

export const roundToNearest = (numberToRound: number, roundTo: number): number => {
    const remainder = numberToRound % roundTo;
    const half = Math.floor(roundTo / 2);
    if(remainder >= half){
        return numberToRound + (roundTo - remainder)
    }else{
        return numberToRound - remainder
    }

}

const drawArrow = (context: CanvasRenderingContext2D, connection: Arrow): void =>{
    context.lineCap = 'round';

    const startX = connection.x1_pos;
    const startY = connection.y1_pos;
    const endX = connection.x2_pos;
    const endY = connection.y2_pos;
    
    const headSize = 30;
    const deltaX = endX - startX;
    const deltaY = endY - startY;
    const angle = Math.atan2(deltaY, deltaX);

    context.lineWidth = 5;
    context.beginPath();
    context.moveTo(startX, startY);
    context.lineTo(endX, endY);
    context.moveTo(endX, endY);
    context.lineTo(endX - headSize * Math.cos(angle - Math.PI / 6), endY - headSize * Math.sin(angle - Math.PI / 6));
    context.moveTo(endX, endY);
    context.lineTo(endX - headSize * Math.cos(angle + Math.PI / 6), endY - headSize * Math.sin(angle + Math.PI / 6));
    context.stroke();

}

export const draw = (context: CanvasRenderingContext2D, 
    width: number, 
    height: number, 
    nodes: Array<State>,
    connections: Array<Arrow>, 
    startStatePosition: number, 
    finalStatePositions: Array<number>) => {
    
    context.clearRect(0, 0, width, height);
    nodes.forEach((node, index)=>{
        context.beginPath();

        context.lineWidth = 3;
        if(index === startStatePosition){
            context.fillStyle = "rgb(22 163 74)";
        }else if(finalStatePositions.includes(index)){
            context.fillStyle = "rgb(37 99 235)";
        }else{
            context.fillStyle = "rgb(234, 88, 12)";
        }
        context.arc(node.x_pos, node.y_pos, 35, 0, 2*Math.PI);
        context.fill();
        context.stroke();
        context.closePath();
    })
    connections.forEach((connection)=>{
        drawArrow(context, connection);
    })

}


// Definitely need to refactor in future
export const makeNewState = (elements: Array<State | Arrow>,
    cursor_x_pos: number,
    cursor_y_pos: number,
    states: Array<State>,
    stateConnections: {[key: string]: StateConnection | undefined},
    isStartStateSelected: boolean,
    startStatePosition: number,
    startStateCoordinates: string | null,
    isFinalStateSelected: boolean,
    finalStatePositions: Array<number>): void => {

    const node: State = {x_pos: cursor_x_pos, y_pos: cursor_y_pos, element: "State"};
    let nodeConnection: StateConnection = {nodes_connected_from: [], nodes_connected_to: [], connection_chars: [], is_final_state: false};
    elements.push(node);
    states.push(node);
    if(isStartStateSelected){
        startStatePosition = states.length - 1;
        startStateCoordinates = `${cursor_x_pos}${cursor_y_pos}`;
        isStartStateSelected = false;
    }else if(isFinalStateSelected){
        finalStatePositions.push(states.length - 1);
        nodeConnection.is_final_state = true;
    }
    stateConnections[`${cursor_x_pos}${cursor_y_pos}`] = nodeConnection;
}

export const initalizeNewArrow = (cursor_x_pos: number, cursor_y_pos: number, connections: Array<Arrow>, 
    linkStart: [number, number], previouslySelectedNodeKey: String | null)=> {
    const connection: Arrow = {x1_pos: cursor_x_pos, y1_pos: cursor_y_pos, x2_pos: cursor_x_pos, y2_pos: cursor_y_pos, element: "Connection"}
    connections = [...connections, connection];
    linkStart = [cursor_x_pos, cursor_y_pos];
    previouslySelectedNodeKey = `${cursor_x_pos}${cursor_y_pos}`
}

export const createNewArrow = (cursor_x_pos: number, cursor_y_pos: number, previouslySelectedNodeKey: string, 
    previousNode: StateConnection, currentNode: StateConnection, connections: Array<Arrow>, elements: Array<State | Arrow>,
    ) =>{

    previousNode.nodes_connected_to.push(`${cursor_x_pos}${cursor_y_pos}`);
    previousNode.connection_chars.push("a");
    currentNode.nodes_connected_from.push( previouslySelectedNodeKey);
    const line = connections.pop();
    if(line){
        line.x2_pos = cursor_x_pos;
        line.y2_pos = cursor_y_pos;
        connections = [...connections, line];
        elements = [...elements, line];
    }

}