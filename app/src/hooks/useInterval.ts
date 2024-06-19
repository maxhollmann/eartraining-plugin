import { useEffect } from "react";

export const useInterval = (duration: number, fn: () => any, deps: any[] = []) => {
  useEffect(() => {
    const interval = setInterval(fn, duration);
    return () => clearInterval(interval);
  }, deps);
};
