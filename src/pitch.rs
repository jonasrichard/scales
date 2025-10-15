use std::fmt::Display;

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pitch {
    pub class: PitchClass,
    pub accidental: Accidental,
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
            accidental: Accidental::Natural,
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
            accidental: accidental.unwrap_or(Accidental::Natural),
            octave,
        })
    }

    /// Check if list of pitches has already the pitch class in it.
    pub fn has_pitch_class(pitches: &[Pitch], class: PitchClass) -> bool {
        pitches.iter().any(|p| p.class == class)
    }

    /// Raise the pitch by choosing the next pitch class and depending on
    /// the number of half notes it computes the accidental and the new
    /// octave if the pitch overflows.
    pub fn build_next(&self, half_notes: i8) -> Self {
        let next_pitch_class = self.class.next();
        let numeric_degree = self.to_numeric_scale_degree();

        let mut raised_degree = numeric_degree + half_notes;

        let raised_octave = if raised_degree > 11 {
            raised_degree %= 12;

            self.octave + 1
        } else {
            self.octave
        };

        let next_base = next_pitch_class as u8;

        println!(
            "[build_next] original: {self:?} raised by {half_notes} halves: {numeric_degree} -> {raised_degree}"
        );

        let accidental = match raised_degree - next_base as i8 {
            -2 => Accidental::DoubleFlat,
            -1 => Accidental::Flat,
            0 => Accidental::Natural,
            1 => Accidental::Sharp,
            2 => Accidental::DoubleSharp,
            diff => {
                panic!("Too big difference: {diff}");
            }
        };

        Pitch {
            class: next_pitch_class,
            accidental,
            octave: raised_octave,
        }
    }

    pub fn to_numeric_scale_degree(&self) -> i8 {
        let num: i8 = self.class as i8;
        let acc: i8 = self.accidental as i8;

        num + acc
    }

    // Here we need to know that we tend to make a flat or sharp pitch note.
    //pub fn add_interval(&self, interval: &Interval) -> Self {
    //    let mut raised_octave = self.octave;
    //    let mut raised = self.to_numeric_scale_degree() + *interval as u8;

    //    if raised > Interval::Major7th as u8 {
    //        raised_octave += 1;
    //        raised -= 12;
    //    }

    //    Into::<Interval>::into(raised).to_pitch(raised_octave)
    //}

    pub fn same_pitch(&self, other: &Self) -> bool {
        self.same_tone(other) && self.octave == other.octave
    }

    fn from(value: u8, is_sharp: bool) -> Self {
        let mut pitch = Pitch {
            class: PitchClass::C,
            accidental: Accidental::Natural,
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

        //let pitch_str = match value {
        //    0 => "C0",
        //    1 => {
        //        if is_sharp {
        //            "C#0"
        //        } else {
        //            "Db0"
        //        }
        //    }
        //    2 => {
        //        pitch.class = PitchClass::D;
        //    }
        //    3 => {
        //        if is_sharp {
        //            pitch.class = PitchClass::D;
        //            pitch.accidental = Some(Accidental::Sharp);
        //        }
        //    }
        //    4 => {
        //        if is_sharp {
        //            pitch.class = PitchClass::E;
        //        } else {
        //            pitch.class = PitchClass::F;
        //            pitch.accidental = Some(Accidental::Flat);
        //        }
        //    }
        //    5 => {
        //        if is_sharp {
        //            pitch.class = PitchClass::E;
        //            pitch.accidental = Some(Accidental::Sharp);
        //        } else {
        //            pitch.class = PitchClass::F;
        //        }
        //    }
        //    6 => {
        //        if is_sharp {
        //            pitch.class = PitchClass::F;
        //            pitch.accidental = Some(Accidental::Sharp);
        //        } else {
        //            pitch.class = PitchClass::G;
        //            pitch.accidental = Some(Accidental::Flat);
        //        }
        //    }
        //    7 => PitchClass::G,
        //    8 => {
        //        if is_sharp {
        //            Pitch::parse("G#0")
        //        } else {
        //            Pitch::parse("Ab0")
        //        }
        //    }

        //    9 => PitchClass::A,
        //    11 => PitchClass::B,
        //    _ => unreachable!(),
        //};

        //Pitch::parse(pitch_str).expect("Cannot parse pitch")
        pitch
    }

    fn same_tone(&self, other: &Self) -> bool {
        let t1 = self.class as i8 - self.accidental as i8;
        let t2 = other.class as i8 - other.accidental as i8;

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
        f.write_fmt(format_args!("{}{}", self.class, self.accidental))
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
                accidental: Accidental::Natural,
                octave: 4
            })
        );
        assert_eq!(
            Pitch::parse("D#5"),
            Some(Pitch {
                class: PitchClass::D,
                accidental: Accidental::Sharp,
                octave: 5
            })
        );
        assert_eq!(
            Pitch::parse("E𝄫3"),
            Some(Pitch {
                class: PitchClass::E,
                accidental: Accidental::DoubleFlat,
                octave: 3
            })
        );
        assert_eq!(
            Pitch::parse("Bb3"),
            Some(Pitch {
                class: PitchClass::B,
                accidental: Accidental::Flat,
                octave: 3
            })
        );
        assert_eq!(
            Pitch::parse("F#2"),
            Some(Pitch {
                class: PitchClass::F,
                accidental: Accidental::Sharp,
                octave: 2
            })
        );
        assert_eq!(
            Pitch::parse("Gx6"),
            Some(Pitch {
                class: PitchClass::G,
                accidental: Accidental::DoubleSharp,
                octave: 6
            })
        );
        assert_eq!(
            Pitch::parse("A7"),
            Some(Pitch {
                class: PitchClass::A,
                accidental: Accidental::Natural,
                octave: 7
            })
        );
        assert_eq!(Pitch::parse("D#"), None);
    }

    #[test]
    fn test_raise_by_half_note() {
        let pairs = vec![
            ("C3", "Db3", 1),
            ("C#3", "D3", 1),
            ("Db3", "E𝄫3", 1),
            ("D3", "Eb3", 1),
            ("D#3", "E3", 1),
            ("Eb3", "Fb3", 1),
            ("E3", "F3", 1),
            ("Fb3", "G𝄫3", 1),
            ("F3", "Gb3", 1),
            ("F#3", "G3", 1),
        ];

        for (base, raised, halves) in pairs {
            assert_eq!(
                Pitch::parse(raised).unwrap(),
                Pitch::parse(base).unwrap().build_next(halves),
                "{base:?} raised by half to get {raised:?}"
            );
        }
    }
}
