export type Interval = {
  name: string;
  symbol: string;
  color: string;
  semiTones: number;
};

export const intervals = {
  unison: { semiTones: 0, name: "Unison", symbol: "1", color: "#554edb" },
  minorSecond: { semiTones: 1, name: "Minor Second", symbol: "b2", color: "#a3e563" },
  majorSecond: { semiTones: 2, name: "Major Second", symbol: "2", color: "#e656e7" },
  minorThird: { semiTones: 3, name: "Minor Third", symbol: "b3", color: "#57e6a2" },
  majorThird: { semiTones: 4, name: "Major Third", symbol: "3", color: "#e0504c" },
  perfectFourth: { semiTones: 5, name: "Perfect Fourth", symbol: "4", color: "#609de4" },
  tritone: { semiTones: 6, name: "Tritone", symbol: "#4", color: "#e4e75c" },
  perfectFifth: { semiTones: 7, name: "Perfect Fifth", symbol: "5", color: "#9c54d8" },
  minorSixth: { semiTones: 8, name: "Minor Sixth", symbol: "b6", color: "#50e660" },
  majorSixth: { semiTones: 9, name: "Major Sixth", symbol: "6", color: "#e55196" },
  minorSeventh: { semiTones: 10, name: "Minor Seventh", symbol: "b7", color: "#58e6e3" },
  majorSeventh: { semiTones: 11, name: "Major Seventh", symbol: "7", color: "#e39e60" },
};

export const intervalsByFifth = [
  intervals.unison,
  intervals.perfectFifth,
  intervals.majorSecond,
  intervals.majorSixth,
  intervals.majorThird,
  intervals.majorSeventh,
  intervals.tritone,
  intervals.minorSecond,
  intervals.minorSixth,
  intervals.minorThird,
  intervals.minorSeventh,
  intervals.perfectFourth,
];
