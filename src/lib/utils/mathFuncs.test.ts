import { describe, expect, it } from "vitest";
import {
  fitBounds, getBezierCurveAngleAtPoint, getPointOnBezierCurveAtDistance, roundToNearest,
} from "./mathFuncs";

const straightCurve = {
  start_point: { x: 0, y: 0 },
  control_point_one: { x: 0, y: 0 },
  control_point_two: { x: 300, y: 0 },
  end_point: { x: 300, y: 0 },
};

describe("roundToNearest", () => {
  it("rounds to the nearest multiple, with halves going up", () => {
    expect(roundToNearest(149, 100)).toBe(100);
    expect(roundToNearest(150, 100)).toBe(200);
    expect(roundToNearest(251, 100)).toBe(300);
    expect(roundToNearest(0, 100)).toBe(0);
    expect(roundToNearest(29, 20)).toBe(20);
    expect(roundToNearest(30, 20)).toBe(40);
  });
});

describe("Bezier curves", () => {
  it("starts at the start point and ends at the end point", () => {
    expect(getPointOnBezierCurveAtDistance(straightCurve, 0)).toEqual({ x: 0, y: 0 });
    expect(getPointOnBezierCurveAtDistance(straightCurve, 1)).toEqual({ x: 300, y: 0 });
  });

  it("is halfway along a symmetric curve halfway through", () => {
    expect(getPointOnBezierCurveAtDistance(straightCurve, 0.5)).toEqual({ x: 150, y: 0 });
  });

  it("points along the curve at its end, for drawing the arrowhead", () => {
    expect(getBezierCurveAngleAtPoint(straightCurve, 0.99)).toBeCloseTo(0);
    const upwards = { ...straightCurve, control_point_two: { x: 0, y: -300 }, end_point: { x: 0, y: -300 } };
    expect(getBezierCurveAngleAtPoint(upwards, 0.99)).toBeCloseTo(-Math.PI / 2);
  });
});

describe("fitBounds", () => {
  // Where a point of the drawing ends up on the canvas
  const onCanvas = (fit: ReturnType<typeof fitBounds>, point: { x: number; y: number }) => ({
    x: (point.x - fit.origin.x) / fit.scale,
    y: (point.y - fit.origin.y) / fit.scale,
  });

  it("centres something small, drawn no larger than the largest size allowed", () => {
    const fit = fitBounds({ left: 0, top: 0, right: 100, bottom: 50 }, 300, 150, { margin: 10, smallestScale: 2 });
    expect(fit.scale).toBe(2);
    expect(onCanvas(fit, { x: 50, y: 25 })).toEqual({ x: 150, y: 75 });
  });

  it("shrinks something wide to fit between the side margins, centred from top to bottom", () => {
    const fit = fitBounds({ left: -200, top: 0, right: 800, bottom: 100 }, 300, 150, { margin: 50, smallestScale: 2 });
    expect(fit.scale).toBe(5);
    expect(onCanvas(fit, { x: -200, y: 0 })).toEqual({ x: 50, y: 65 });
    expect(onCanvas(fit, { x: 800, y: 100 })).toEqual({ x: 250, y: 85 });
  });

  it("shrinks something tall to fit between the top and bottom margins, centred from side to side", () => {
    const fit = fitBounds({ left: 0, top: 100, right: 100, bottom: 1100 }, 300, 150, { margin: 25, smallestScale: 2 });
    expect(fit.scale).toBe(10);
    expect(onCanvas(fit, { x: 0, y: 100 })).toEqual({ x: 145, y: 25 });
    expect(onCanvas(fit, { x: 100, y: 1100 })).toEqual({ x: 155, y: 125 });
  });
});
