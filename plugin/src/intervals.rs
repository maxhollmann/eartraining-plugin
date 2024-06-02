use core::fmt;
use std::fmt::{Display, Formatter};

use nih_plug_iced::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    Unison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    Tritone,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
}

impl Interval {
    pub const ALL: [Interval; 12] = [
        Interval::Unison,
        Interval::MinorSecond,
        Interval::MajorSecond,
        Interval::MinorThird,
        Interval::MajorThird,
        Interval::PerfectFourth,
        Interval::Tritone,
        Interval::PerfectFifth,
        Interval::MinorSixth,
        Interval::MajorSixth,
        Interval::MinorSeventh,
        Interval::MajorSeventh,
    ];

    pub fn short_name(self) -> &'static str {
        match self {
            Interval::Unison => "1",
            Interval::MinorSecond => "b2",
            Interval::MajorSecond => "2",
            Interval::MinorThird => "b3",
            Interval::MajorThird => "3",
            Interval::PerfectFourth => "4",
            Interval::Tritone => "#4",
            Interval::PerfectFifth => "5",
            Interval::MinorSixth => "b6",
            Interval::MajorSixth => "6",
            Interval::MinorSeventh => "b7",
            Interval::MajorSeventh => "7",
        }
    }

    #[allow(dead_code)]
    pub fn long_name(self) -> &'static str {
        match self {
            Interval::Unison => "Unison",
            Interval::MinorSecond => "Minor Second",
            Interval::MajorSecond => "Major Second",
            Interval::MinorThird => "Minor Third",
            Interval::MajorThird => "Major Third",
            Interval::PerfectFourth => "Perfect Fourth",
            Interval::Tritone => "Tritone",
            Interval::PerfectFifth => "Perfect Fifth",
            Interval::MinorSixth => "Minor Sixth",
            Interval::MajorSixth => "Major Sixth",
            Interval::MinorSeventh => "Minor Seventh",
            Interval::MajorSeventh => "Major Seventh",
        }
    }

    pub fn color(self) -> Color {
        match self {
            Interval::Unison => Color::from_rgb8(0x55, 0x4e, 0xdb),
            Interval::MinorSecond => Color::from_rgb8(0xa3, 0xe5, 0x63),
            Interval::MajorSecond => Color::from_rgb8(0xe6, 0x56, 0xe7),
            Interval::MinorThird => Color::from_rgb8(0x57, 0xe6, 0xa2),
            Interval::MajorThird => Color::from_rgb8(0xe0, 0x50, 0x4c),
            Interval::PerfectFourth => Color::from_rgb8(0x60, 0x9d, 0xe4),
            Interval::Tritone => Color::from_rgb8(0xe4, 0xe7, 0x5c),
            Interval::PerfectFifth => Color::from_rgb8(0x9c, 0x54, 0xd8),
            Interval::MinorSixth => Color::from_rgb8(0x50, 0xe6, 0x60),
            Interval::MajorSixth => Color::from_rgb8(0xe5, 0x51, 0x96),
            Interval::MinorSeventh => Color::from_rgb8(0x58, 0xe6, 0xe3),
            Interval::MajorSeventh => Color::from_rgb8(0xe3, 0x9e, 0x60),
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.short_name())
    }
}
