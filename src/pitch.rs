use std::fmt::Display;

use crate::scale::Interval;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PitchClass {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Accidental {
    DoubleFlat = -2,
    Flat = -1,
    Natural = 0,
    Sharp = 1,
    DoubleSharp = 2,
}

#[derive(Debug, PartialEq)]
pub struct Pitch {
    pub class: PitchClass,
    pub accidental: Option<Accidental>,
    pub octave: u8,
}

impl PitchClass {
    pub fn next(&self) -> Self {
        match self {
            PitchClass::C => PitchClass::D,
            PitchClass::D => PitchClass::E,
            PitchClass::E => PitchClass::F,
            PitchClass::F => PitchClass::G,
            PitchClass::G => PitchClass::A,
            PitchClass::A => PitchClass::B,
            PitchClass::B => PitchClass::C,
        }
    }
}

impl Pitch {
    pub fn new(class: PitchClass) -> Self {
        Pitch {
            class,
            accidental: None,
            octave: 1,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let mut chars = s.chars().peekable();

        let class = match chars.next()? {
            'C' => PitchClass::C,
            'D' => PitchClass::D,
            'E' => PitchClass::E,
            'F' => PitchClass::F,
            'G' => PitchClass::G,
            'A' => PitchClass::A,
            'B' => PitchClass::B,
            _ => return None,
        };

        let accidental = match chars.peek() {
            Some('𝄫') => Some(Accidental::DoubleFlat),
            Some('b') => Some(Accidental::Flat),
            Some('#') => Some(Accidental::Sharp),
            Some('x') => Some(Accidental::DoubleSharp),
            Some('n') => Some(Accidental::Natural),
            _ => None,
        };

        if accidental.is_some() {
            chars.next(); // consume the accidental
        }

        let octave = chars.collect::<String>().parse().ok()?;

        Some(Pitch {
            class,
            accidental,
            octave,
        })
    }

    pub fn to_numeric_scale_degree(&self) -> u8 {
        let num: u8 = self.class as u8;
        let acc: u8 = self.accidental.map_or(0u8, |a| a as u8);

        num + acc
    }

    // Here we need to know that we tend to make a flat or sharp pitch note.
    pub fn add_interval(&self, interval: &Interval) -> Self {
        let mut raised_octave = self.octave;
        let mut raised = self.to_numeric_scale_degree() + *interval as u8;

        if raised > Interval::Major7th as u8 {
            raised_octave += 1;
            raised -= 12;
        }

        Into::<Interval>::into(raised).to_pitch(raised_octave)
    }

    pub fn same_pitch(&self, other: &Self) -> bool {
        self.same_tone(other) && self.octave == other.octave
    }

    /// Find the next alternative of the pitch in the sharp direction, if it
    /// overflows start from the flat.
    pub fn alternative_pitch(&self) -> Self {
        if self.accidental.is_none() || self.accidental.unwrap() == Accidental::Natural {
            return *self;
        }

        match self.accidental.unwrap() {
            Accidental::DoubleFlat => todo!(),
            Accidental::Flat => todo!(),
            Accidental::Natural => todo!(),
            Accidental::Sharp => {
                let new_pitch_class = self.class.next();
            }
            Accidental::DoubleSharp => todo!(),
        }
    }

    fn from(value: u8, is_sharp: bool) -> Self {
        let mut pitch = Pitch {
            class: PitchClass::C,
            accidental: None,
            octave: 0,
        };

        // C1    B#0
        // C#1   Db1
        // D1    Cx1, Ebb1
        // D#1   Eb1, Fbb1
        // E1    Dx1, Fb1
        // F1    E#1, Gbb1
        // F#1   Gb1
        // G1    Fx1, Abb1
        // G#1   Ab1
        // A1    Gx1, Bbb1
        // A#1   Bb1, Cbb1
        // B1    Ax1, Cb1
        //
        // Bb1 needs to find in the 2nd list, so basically we need to create
        // equivalence groups

        let pitch_str = match value {
            0 => "C0",
            1 => {
                if is_sharp {
                    "C#0"
                } else {
                    "Db0"
                }
            }
            2 => {
                pitch.class = PitchClass::D;
            }
            3 => {
                if is_sharp {
                    pitch.class = PitchClass::D;
                    pitch.accidental = Some(Accidental::Sharp);
                }
            }
            4 => {
                if is_sharp {
                    pitch.class = PitchClass::E;
                } else {
                    pitch.class = PitchClass::F;
                    pitch.accidental = Some(Accidental::Flat);
                }
            }
            5 => {
                if is_sharp {
                    pitch.class = PitchClass::E;
                    pitch.accidental = Some(Accidental::Sharp);
                } else {
                    pitch.class = PitchClass::F;
                }
            }
            6 => {
                if is_sharp {
                    pitch.class = PitchClass::F;
                    pitch.accidental = Some(Accidental::Sharp);
                } else {
                    pitch.class = PitchClass::G;
                    pitch.accidental = Some(Accidental::Flat);
                }
            }
            7 => PitchClass::G,
            8 => {
                if is_sharp {
                    Pitch::parse("G#0")
                } else {
                    Pitch::parse("Ab0")
                }
            }

            9 => PitchClass::A,
            11 => PitchClass::B,
            _ => unreachable!(),
        };

        Pitch::parse(pitch_str)
    }

    fn same_tone(&self, other: &Self) -> bool {
        let mut t1 = self.class as i8;

        if let Some(a) = self.accidental {
            t1 -= a as i8;
        }

        let mut t2 = other.class as i8;

        if let Some(a) = other.accidental {
            t2 -= a as i8;
        }

        t1 == t2
    }
}

impl Display for PitchClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PitchClass::C => f.write_str("C"),
            PitchClass::D => f.write_str("D"),
            PitchClass::E => f.write_str("E"),
            PitchClass::F => f.write_str("F"),
            PitchClass::G => f.write_str("G"),
            PitchClass::A => f.write_str("A"),
            PitchClass::B => f.write_str("B"),
        }
    }
}

impl Display for Accidental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accidental::DoubleFlat => f.write_str("𝄫"),
            Accidental::Flat => f.write_str("b"),
            Accidental::Natural => f.write_str(""),
            Accidental::Sharp => f.write_str("#"),
            Accidental::DoubleSharp => f.write_str("𝄪"),
        }
    }
}

impl Display for Pitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}",
            self.class,
            self.accidental.unwrap_or(Accidental::Natural)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_from_str() {
        assert_eq!(
            Pitch::parse("C4"),
            Some(Pitch {
                class: PitchClass::C,
                accidental: None,
                octave: 4
            })
        );
        assert_eq!(
            Pitch::parse("D#5"),
            Some(Pitch {
                class: PitchClass::D,
                accidental: Some(Accidental::Sharp),
                octave: 5
            })
        );
        assert_eq!(
            Pitch::parse("E𝄫3"),
            Some(Pitch {
                class: PitchClass::E,
                accidental: Some(Accidental::DoubleFlat),
                octave: 3
            })
        );
        assert_eq!(
            Pitch::parse("Bb3"),
            Some(Pitch {
                class: PitchClass::B,
                accidental: Some(Accidental::Flat),
                octave: 3
            })
        );
        assert_eq!(
            Pitch::parse("F#2"),
            Some(Pitch {
                class: PitchClass::F,
                accidental: Some(Accidental::Sharp),
                octave: 2
            })
        );
        assert_eq!(
            Pitch::parse("Gx6"),
            Some(Pitch {
                class: PitchClass::G,
                accidental: Some(Accidental::DoubleSharp),
                octave: 6
            })
        );
        assert_eq!(
            Pitch::parse("A7"),
            Some(Pitch {
                class: PitchClass::A,
                accidental: None,
                octave: 7
            })
        );
        assert_eq!(Pitch::parse("D#"), None);
    }
}
