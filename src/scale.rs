use std::fmt::Display;

use crate::pitch::{Accidental, Pitch};

#[derive(Clone, Copy, Debug)]
pub struct ScaleDegree {
    pub degree: u8,
    pub modifier: Accidental,
}

#[derive(Clone, Debug)]
pub struct Scale {
    pub root_note: Pitch,
    pub name: String,
    pub scale_degrees: Vec<ScaleDegree>,
    pub pitches: Vec<Pitch>,
}

pub const IONIAN: [ScaleDegree; 7] = [
    ScaleDegree {
        degree: 1,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 2,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 3,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 4,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 5,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 6,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 7,
        modifier: Accidental::Natural,
    },
];

pub const DORIAN: [ScaleDegree; 7] = [
    ScaleDegree {
        degree: 1,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 2,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 3,
        modifier: Accidental::Flat,
    },
    ScaleDegree {
        degree: 4,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 5,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 6,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 7,
        modifier: Accidental::Flat,
    },
];

pub const PHRYGIAN: [ScaleDegree; 7] = [
    ScaleDegree {
        degree: 1,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 2,
        modifier: Accidental::Flat,
    },
    ScaleDegree {
        degree: 3,
        modifier: Accidental::Flat,
    },
    ScaleDegree {
        degree: 4,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 5,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 6,
        modifier: Accidental::Flat,
    },
    ScaleDegree {
        degree: 7,
        modifier: Accidental::Flat,
    },
];

pub const LYDIAN: [ScaleDegree; 7] = [
    ScaleDegree {
        degree: 1,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 2,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 3,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 4,
        modifier: Accidental::Sharp,
    },
    ScaleDegree {
        degree: 5,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 6,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 7,
        modifier: Accidental::Natural,
    },
];

pub const MIXOLYDIAN: [ScaleDegree; 7] = [
    ScaleDegree {
        degree: 1,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 2,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 3,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 4,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 5,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 6,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 7,
        modifier: Accidental::Flat,
    },
];

pub const AEOLIAN: [ScaleDegree; 7] = [
    ScaleDegree {
        degree: 1,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 2,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 3,
        modifier: Accidental::Flat,
    },
    ScaleDegree {
        degree: 4,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 5,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 6,
        modifier: Accidental::Flat,
    },
    ScaleDegree {
        degree: 7,
        modifier: Accidental::Flat,
    },
];

pub const LOCRIAN: [ScaleDegree; 7] = [
    ScaleDegree {
        degree: 1,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 2,
        modifier: Accidental::Flat,
    },
    ScaleDegree {
        degree: 3,
        modifier: Accidental::Flat,
    },
    ScaleDegree {
        degree: 4,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 5,
        modifier: Accidental::Flat,
    },
    ScaleDegree {
        degree: 6,
        modifier: Accidental::Flat,
    },
    ScaleDegree {
        degree: 7,
        modifier: Accidental::Flat,
    },
];

/// Interval distances of scale degrees.
pub const SCALE_NATURAL_DEGREES: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];

impl Scale {
    pub fn new(root_note: &Pitch, name: &str, degrees: &[ScaleDegree]) -> Self {
        Scale {
            root_note: *root_note,
            name: name.to_string(),
            scale_degrees: degrees.to_vec(),
            pitches: Scale::build_scale(root_note, degrees),
        }
    }

    pub fn to_numeric_degrees(degrees: &[ScaleDegree]) -> Vec<i8> {
        let mut result = vec![0];

        for i in 1..degrees.len() {
            let nd = SCALE_NATURAL_DEGREES[i] as i8 + degrees[i].modifier as i8;

            result.push(nd);
        }

        result
    }

    pub fn build_scale(root_note: &Pitch, degrees: &[ScaleDegree]) -> Vec<Pitch> {
        let mut result = vec![*root_note];
        let mut i = 1;
        let mut last_pitch = *root_note;
        let mut last_degree = 0i8;
        let numeric_degrees = Scale::to_numeric_degrees(degrees);

        loop {
            let next_degree = degrees[i];
            let next_num_degree = numeric_degrees[(next_degree.degree - 1) as usize];
            let half_notes = next_num_degree - last_degree;
            let next_pitch = last_pitch.build_next(half_notes);

            last_pitch = next_pitch;
            last_degree = next_num_degree;

            result.push(next_pitch);

            i += 1;

            if i == degrees.len() {
                break result;
            }
        }
    }
}

impl Display for ScaleDegree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.degree, self.modifier))
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_fmt(format_args!("{} {} - ", self.root_note, self.name));

        for pitch in &self.pitches {
            let _ = f.write_fmt(format_args!("{} ", pitch));
        }

        let _ = f.write_str(" - ");

        for degree in &self.scale_degrees {
            let _ = f.write_fmt(format_args!("{} ", degree));
        }

        Ok(())
    }
}

// TODO implement a feature like, one F lydian I want chord progression:
// I iv V III or something

// Here we need to learn const generic

//pub fn compute_scale(root_note: &Pitch, scale: &[ScaleDegree; 7]) -> Vec<Pitch> {
//    let mut new_scale = vec![];
//
//    for (i, degree) in scale.iter().enumerate() {
//        let distance = SCALE_DEGREES[i];
//
//        if distance == 0 {
//            new_scale.push(root_note.clone());
//        }
//    }
//
//    new_scale
//}

pub fn scale_from_string(scale: &str) -> Vec<Pitch> {
    let mut result = vec![];
    let scale_pitches: Vec<_> = scale.split_whitespace().collect();

    for sp in scale_pitches {
        let p = Pitch::parse(sp).unwrap();

        result.push(p);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pitch::PitchClass;

    #[test]
    fn compute_scale_test() {
        let a = Pitch::new(PitchClass::A);
        let a_ionian = Scale::new(&a, "ionian", &IONIAN);

        assert_eq!(
            scale_from_string("A1 B1 C#1 D1 E1 F#1 G#1"),
            a_ionian.pitches
        );
    }
}
