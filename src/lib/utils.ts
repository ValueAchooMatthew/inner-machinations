import type { State, Arrow, } from "./interfaces";

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