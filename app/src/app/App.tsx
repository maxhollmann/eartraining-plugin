import { useState } from "react";
import { intervalsByFifth } from "../config/intervals";
import { IntervalCircle } from "../features/interval-circle/components/IntervalCircle";
import { useInterval } from "../hooks/useInterval";
import "./App.css";

function App() {
  const [activeSemitones, setActiveSemitones] = useState<number[]>([0, 5, 1]);

  useInterval(1500, () => {
    setActiveSemitones(
      intervalsByFifth.map((interval) => interval.semiTones).filter(() => Math.random() > 0.5)
    );
  });

  return (
    <div className="container">
      <IntervalCircle intervals={intervalsByFifth} activeSemitones={activeSemitones} />
    </div>
  );
}

export default App;
