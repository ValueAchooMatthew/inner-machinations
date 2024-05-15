import type { State, Connection, Coordinate, BezierCurve } from "./interfaces";
import { getBezierCurveAngleAtPoint, getPointOnBezierCurveAtDistance } from "./mathFuncs";

export const draw = (context: CanvasRenderingContext2D, 
    width: number, height: number, nodes: Array<State>,
    connections: Array<Connection>, 
    start_statePosition: number, selected_connection_index: number | null) => {
    
    // Needed so position specified for where characters are drawn is not drawn differently depending on if its offset is above/below or
    // left/right of the Connection
    context.textBaseline = "middle";
    context.textAlign = "center"
    context.strokeStyle = "black"
    context.clearRect(0, 0, width, height);

    nodes.forEach((node, index)=>{
        drawNode(context, index, start_statePosition, node);
    });
    connections.forEach((connection, index)=>{
        drawConnection(context, connection, index, selected_connection_index);
    });

}

const drawConnection = (context: CanvasRenderingContext2D, connection: Connection, index: number, selected_connection_index: number | null): void =>{
    context.lineCap = 'round';

    const start_coord: Coordinate = connection.curve.start_point;
    // +1 to make self loops easier to draw
    const end_coord: Coordinate = connection.curve.end_point;
    const curve: BezierCurve = connection.curve;
    const headSize = 30;
    context.lineWidth = 5;

    if(selected_connection_index === index){
        context.strokeStyle = "#00008B"
    }else{
        context.strokeStyle = "black"
    }
    context.beginPath();
    context.moveTo(start_coord.x, start_coord.y);
    // context.lineTo(endX, endY);

    context.bezierCurveTo(curve.control_point_one.x, curve.control_point_one.y, 
    curve.control_point_two.x, curve.control_point_two.y, end_coord.x, end_coord.y);
    
    context.stroke();
    const angle_of_curve_at_end = getBezierCurveAngleAtPoint(curve, .99);
    context.moveTo(end_coord.x, end_coord.y);

    context.lineTo(end_coord.x - headSize * Math.cos((angle_of_curve_at_end) - Math.PI / 6), 
    end_coord.y - headSize * Math.sin((angle_of_curve_at_end) - Math.PI / 6));

    context.moveTo(end_coord.x, end_coord.y);

    context.lineTo(end_coord.x - headSize * Math.cos((angle_of_curve_at_end) + Math.PI / 6), 
    end_coord.y - headSize * Math.sin((angle_of_curve_at_end) + Math.PI / 6));
    
    context.stroke();
    context.font = "40px Arial";
    context.fillStyle = "black";

    const halfway_point = getPointOnBezierCurveAtDistance(curve, .5);

    context.fillText(connection.character, halfway_point.x + 50 * Math.sin(angle_of_curve_at_end), 
    halfway_point.y - 50 * Math.cos(angle_of_curve_at_end));

}

const drawNode = (context: CanvasRenderingContext2D, index: number, start_state_position: number, node: State) => {
    context.lineWidth = 3;
    if(node.is_final){
        context.beginPath();
        context.strokeStyle = "black";
        context.arc(node.position.x, node.position.y, 42, 0, 2*Math.PI);
        context.stroke();
        context.closePath();
    }
    // For some reason fills with the wrong colour unless I do this, no idea why
    if(index === start_state_position){
        context.fillStyle = "rgb(22, 163, 74)";
        context.beginPath();
        context.arc(node.position.x, node.position.y, 35, 0, 2*Math.PI);
        context.fill();
        context.stroke();
        context.closePath();
    }else{
        context.fillStyle = "rgb(234, 88, 12)";
        context.beginPath();
        context.arc(node.position.x, node.position.y, 35, 0, 2*Math.PI);
        context.fill();
        context.stroke();
        context.closePath();
        context.fillStyle = "rgb(22, 163, 74)";
    }
} 

