import { describe, expect, it } from "vitest";
import {
  getBezierCurveAngleAtPoint, getPointOnBezierCurveAtDistance, roundToNearest,
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
