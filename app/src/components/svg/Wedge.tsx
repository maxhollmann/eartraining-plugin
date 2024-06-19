import { Point, pointOnCircle } from "../../utils/geometry";
import { svgPoint } from "../../utils/svg";

export type WedgeProps = {
  center: Point;
  startAngle: number;
  endAngle: number;
  outerRadius: number;
  innerRadius?: number;
  label?: string;
} & Omit<React.SVGProps<SVGPathElement>, "d">;
export const Wedge = ({
  center: c,
  startAngle,
  endAngle,
  outerRadius,
  innerRadius = 0,
  label,
  ...pathProps
}: WedgeProps) => {
  const inner1 = pointOnCircle({ c, r: innerRadius, a: startAngle });
  const outer1 = pointOnCircle({ c, r: outerRadius, a: startAngle });
  const outer2 = pointOnCircle({ c, r: outerRadius, a: endAngle });
  const inner2 = pointOnCircle({ c, r: innerRadius, a: endAngle });

  // A rx ry x-axis-rotation large-arc-flag sweep-flag x y
  const path = `
    M ${svgPoint(inner1)}
    L ${svgPoint(outer1)}
    A ${outerRadius} ${outerRadius} 0 0 1 ${svgPoint(outer2)}
    L ${svgPoint(inner2)}
    A ${innerRadius} ${innerRadius} 0 0 0 ${svgPoint(inner1)}
    z
  `;

  return <path d={path} {...pathProps} />;
};
