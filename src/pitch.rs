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

impl Pitch {
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

    pub fn same_pitch(&self, other: &Self) -> bool {
        self.same_tone(other) && self.octave == other.octave
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

