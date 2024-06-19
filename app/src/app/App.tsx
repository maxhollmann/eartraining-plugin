import { useEffect, useState } from "react";
import { intervalsByFifth } from "../config/intervals";
import { IntervalCircle } from "../features/interval-circle/components/IntervalCircle";
import "./App.css";

function App() {
  const [activeSemitones, setActiveSemitones] = useState<number[]>([0, 5, 1]);

  useEffect(() => {
    const interval = setInterval(() => {
      const semitones = [];
      for (let i = 0; i < 12; i++) {
        if (Math.random() > 0.5)
          semitones.push(i);
      }
      setActiveSemitones(semitones);
    }, 1500);
    return () => clearInterval(interval);
  }, [])

  return (
    <div className="container">
      <IntervalCircle intervals={intervalsByFifth} activeSemitones={activeSemitones} />
    </div>
  );
}

export default App;
