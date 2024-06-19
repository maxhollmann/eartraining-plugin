import { Key, useEffect, useState } from "react";
import { Wedge } from "../../../components/svg/Wedge";
import { FULL_CIRCLE, Point, deg, pointOnCircle } from "../../../utils/geometry";
import { useSpring } from "@react-spring/web";

export type WedgeCircleWedge = {
  color: string;
  active: boolean;
  label?: string;
  key: Key;
};

export type WedgeCircleProps = {
  wedges: WedgeCircleWedge[];
  gapAngle: number;
  startAngle?: number;
};
export const WedgeCircle = ({ wedges, gapAngle, startAngle = deg(270) }: WedgeCircleProps) => {
  const num = wedges.length;
  const radPerWedge = FULL_CIRCLE / num;

  return (
    <>
      {wedges.map((wedge, i) => (
        <Segment
          key={wedge.key}
          startAngle={startAngle + (i - 0.5) * radPerWedge + gapAngle / 2}
          endAngle={startAngle + (i + 0.5) * radPerWedge - gapAngle / 2}
          color={wedge.color}
          active={wedge.active}
          label={wedge.label}
        />
      ))}
    </>
  );
};

type SegmentProps = {
  startAngle: number;
  endAngle: number;
  color: string;
  active: boolean;
  label?: string;
};
const Segment = ({ startAngle, endAngle, color, active, label }: SegmentProps) => {
  const c: Point = [50, 50];
  const [labelX, labelY] = pointOnCircle({ c, r: 33.5, a: (startAngle + endAngle) / 2 });

  const [styles, api] = useSpring(() => ({ expansion: 0 }))
  const [dummy, setDummy] = useState(0);

  useEffect(() => {
    if (active) {
      api.start({
        expansion: 1,
      })
    } else {
      api.start({
        expansion: 0,
      })
    }
  }, [active]);

  useEffect(() => {
    const interval = setInterval(() => {
      setDummy((prev) => (prev + 1) % 999);
    }, 10);
    return () => clearInterval(interval);
  }, [])

  return (
    <>
        <Wedge
          center={c}
          startAngle={startAngle}
          endAngle={endAngle}
          innerRadius={30}
          outerRadius={30 + 6.5 * styles.expansion.get()}
          fill={color}
          fillOpacity={0.5}
        />
      <Wedge
        center={c}
        startAngle={startAngle}
        endAngle={endAngle}
        innerRadius={30}
        outerRadius={31}
        fill={color}
        fillOpacity={1}
      />

      {label && (
        <text
          x={labelX}
          y={labelY}
          textAnchor="middle"
          dominantBaseline="central"
          fill="var(--color-fg)"
          fontSize="3"
        >
          {label}
        </text>
      )}
    </>
  );
};
