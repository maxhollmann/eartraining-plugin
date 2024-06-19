export const deg = (angle: number) => (angle * Math.PI) / 180;

export const FULL_CIRCLE = Math.PI * 2;

export type Point = [number, number];

export const pointOnCircle = ({ c, r, a }: { c: Point; r: number; a: number }): Point => {
  const [cX, cY] = c;
  const pX = cX + r * Math.cos(a);
  const pY = cY + r * Math.sin(a);
  return [pX, pY];
};
