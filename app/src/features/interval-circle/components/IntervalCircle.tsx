import { Interval } from "../../../config/intervals";
import { deg } from "../../../utils/geometry";
import { WedgeCircle, WedgeCircleWedge } from "./WedgeCircle";

type IntervalCircleProps = {
  intervals: Interval[];
  activeSemitones?: number[];
};
export const IntervalCircle = ({ intervals, activeSemitones = [] }: IntervalCircleProps) => {
  const wedges: WedgeCircleWedge[] = intervals.map((interval) => ({
    color: interval.color,
    active: activeSemitones.includes(interval.semiTones),
    label: interval.symbol,
    key: interval.name,
  }));

  return (
    <div className="container">
      <svg width="100%" height="100%" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
        <rect x="0" y="0" width="100" height="100" fill="var(--color-bg)" />

        <WedgeCircle wedges={wedges} gapAngle={deg(2)} />
      </svg>
    </div>
  );
};
