import type { State, Connection, } from "./interfaces";

export const roundToNearest = (numberToRound: number, roundTo: number): number => {
    const remainder = numberToRound % roundTo;
    const half = Math.floor(roundTo / 2);
    if(remainder >= half){
        return numberToRound + (roundTo - remainder)
    }else{
        return numberToRound - remainder
    }

}

export const draw = (context: CanvasRenderingContext2D, width: number, height: number, elements: Array<State | Connection>, startStatePosition: number, finalStatePositions: Array<number>) =>{
    if(!context){
        return;
    }
    context.clearRect(0, 0, width, height);
    elements.forEach((obj, index)=>{
        if(context){
            context.beginPath();
            if(obj.element === "State"){
                context.lineWidth = 3;
                if(index === startStatePosition){
                    console.log("balls");
                    context.fillStyle = "rgb(22 163 74)";
                }else if(finalStatePositions.includes(index)){
                    context.fillStyle = "rgb(37 99 235)"
                }else{
                    context.fillStyle = "rgb(234, 88, 12)";
                }
                context.arc(obj.x_pos, obj.y_pos, 35, 0, 2*Math.PI);
                context.fill();
                context.stroke();
            }else{
                context.lineWidth = 7;
                context.beginPath();
                context.moveTo(obj.x1_pos, obj.y1_pos);
                context.lineTo(obj.x2_pos, obj.y2_pos);
                context.stroke();
            }
            context.closePath();
        }
    })
}