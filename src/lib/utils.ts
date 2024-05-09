import type { State, Arrow } from "./interfaces";

export const roundToNearest = (numberToRound: number, roundTo: number): number => {
    const remainder = numberToRound % roundTo;
    const half = Math.floor(roundTo / 2);
    if(remainder >= half){
        return numberToRound + (roundTo - remainder)
    }else{
        return numberToRound - remainder
    }

}

const drawArrow = (context: CanvasRenderingContext2D, connection: Arrow, index: number, selectedArrowIndex: number | null): void =>{
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
    if(selectedArrowIndex === index){
        context.strokeStyle = "#00008B"
    }else{
        context.strokeStyle = "black"
    }
    context.beginPath();
    context.moveTo(startX, startY);
    context.lineTo(endX, endY);
    context.moveTo(endX, endY);
    context.lineTo(endX - headSize * Math.cos(angle - Math.PI / 6), endY - headSize * Math.sin(angle - Math.PI / 6));
    context.moveTo(endX, endY);
    context.lineTo(endX - headSize * Math.cos(angle + Math.PI / 6), endY - headSize * Math.sin(angle + Math.PI / 6));
    context.stroke();
    context.font = "40px Arial";
    context.fillStyle = "black";
;
    // Needed since text is drawn from bottom left
    context.fillText(connection.character, startX + (endX - startX)/2 + 50 * Math.sin(angle), startY + (endY - startY)/2 - 50 * Math.cos(angle));

}

const drawNode = (context: CanvasRenderingContext2D, index: number, startStatePosition: number, node: State) => {
    context.lineWidth = 3;
    if(node.is_final){
        context.beginPath();
        context.strokeStyle = "black";
        context.arc(node.x_pos, node.y_pos, 42, 0, 2*Math.PI);
        context.stroke();
        context.closePath();
    }
    // For some reason fills with the wrong colour unless I do this, no idea why
    if(index === startStatePosition){
        context.fillStyle = "rgb(22, 163, 74)";
        console.log(node, index, startStatePosition, context.fillStyle);
        context.beginPath();
        context.arc(node.x_pos, node.y_pos, 35, 0, 2*Math.PI);
        context.fill();
        context.stroke();
        context.closePath();
    }
    else{
        context.fillStyle = "rgb(234, 88, 12)";
        context.beginPath();
        context.arc(node.x_pos, node.y_pos, 35, 0, 2*Math.PI);
        context.fill();
        context.stroke();
        context.closePath();
        context.fillStyle = "rgb(22, 163, 74)";
    }
} 

export const draw = (context: CanvasRenderingContext2D, 
    width: number, height: number, nodes: Array<State>,
    connections: Array<Arrow>, 
    startStatePosition: number, selectedArrowIndex: number | null) => {
    
    // Needed so position specified for where characters are drawn is not drawn differently depending on if its offset is above/below or
    // left/right of the arrow
    context.textBaseline = "middle";
    context.textAlign = "center"
    context.strokeStyle = "black"
    context.clearRect(0, 0, width, height);
    nodes.forEach((node, index)=>{
        drawNode(context, index, startStatePosition, node);
    })
    connections.forEach((connection, index)=>{
        drawArrow(context, connection, index, selectedArrowIndex);
    })

}


// Returns index of line in array of the line closest to the given x, y coords
export const closestLineToPoint = (xPos: number, yPos: number, lines: Array<Arrow>): number => {

    let indexOfClosestLine = 0;
    let minimumDistance = Infinity;
    for(let i = 0; i < lines.length; i++){
        const distance = distanceFromLineToPoint(xPos, yPos, 
            [[lines[i].x1_pos, lines[i].y1_pos], [lines[i].x2_pos, lines[i].y2_pos]]);
        if(distance < minimumDistance){
            indexOfClosestLine = i;
            minimumDistance = distance;
        }else if(distance == minimumDistance && Math.random() > .5){
            // Simple way to alternate between the two (albeit randomly) if two arrows are the same distance from
            // The point
            // Dumb workaround will change later
            indexOfClosestLine = i;
        }
    }

    return indexOfClosestLine;

}

const distanceFromLineToPoint = (xPos: number, yPos: number, line: [[number, number], [number, number]]): number => {
    // y = mx + b (slope intercept form)
    // Ax + By + C = 0 (standard form) (C = -b, A = -slope)
    // Gigantic thanks  to this answer from stackoverflow answer https://stackoverflow.com/a/1501725 for easy to follow solution
    // To this problem
    let x1 = line[0][0];
    let y1 = line[0][1];
    let x2 = line[1][0];
    let y2 = line[1][1];

    const l2 =  (x1 - x2)**2 + (y1 - y2)**2;
    if (l2 == 0){
        return Math.sqrt((x1 - xPos)**2 + (y1 - yPos)**2);
    }
    let t = ((xPos - x1) * (x2 - x1) + (yPos - y1) * (y2 - y1)) / l2;
    t = Math.max(0, Math.min(1, t));

    return Math.sqrt((xPos - (x1 + t*(x2 - x1)))**2 + (yPos - (y1 + t*(y2 - y1)))**2); 

}