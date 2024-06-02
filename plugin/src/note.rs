use core::fmt;
use std::fmt::{Display, Formatter};

use nih_plug::params::enums::Enum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl Enum for Note {
    fn from_index(index: usize) -> Self {
        Self::ALL[index]
    }

    fn to_index(self) -> usize {
        Self::ALL.iter().position(|&i| i == self).unwrap()
    }

    fn variants() -> &'static [&'static str] {
        &[
            "C",
            "C#",
            "D",
            "D#",
            "E",
            "F",
            "F#",
            "G",
            "G#",
            "A",
            "A#",
            "B",
        ]
    }

    fn ids() -> Option<&'static [&'static str]> { None }
}

impl Note {
    pub const ALL: [Note; 12] = [
        Note::C,
        Note::CSharp,
        Note::D,
        Note::DSharp,
        Note::E,
        Note::F,
        Note::FSharp,
        Note::G,
        Note::GSharp,
        Note::A,
        Note::ASharp,
        Note::B,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Note::C => "C",
            Note::CSharp => "C#",
            Note::D => "D",
            Note::DSharp => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => "F#",
            Note::G => "G",
            Note::GSharp => "G#",
            Note::A => "A",
            Note::ASharp => "A#",
            Note::B => "B",
        }
    }

    pub fn midi_index(self) -> u8 {
        match self {
            Note::C => 0,
            Note::CSharp => 1,
            Note::D => 2,
            Note::DSharp => 3,
            Note::E => 4,
            Note::F => 5,
            Note::FSharp => 6,
            Note::G => 7,
            Note::GSharp => 8,
            Note::A => 9,
            Note::ASharp => 10,
            Note::B => 11,
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
