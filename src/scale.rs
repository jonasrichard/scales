use std::fmt::Display;

use crate::pitch::{Accidental, Pitch};

#[derive(Clone, Copy, Debug)]
pub struct ScaleDegree {
    pub degree: u8,
    pub modifier: Accidental,
}

#[derive(Clone, Debug)]
pub struct ScaleType {
    pub name: String,
    pub scale_degrees: Vec<ScaleDegree>,
}

#[derive(Clone, Debug)]
pub struct Scale {
    pub root_note: Pitch,
    pub scale_type: ScaleType,
    pub pitches: Vec<Pitch>,
}

/// Interval distances of scale degrees.
pub const SCALE_NATURAL_DEGREES: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];

impl Scale {
    pub fn new(root_note: &Pitch, scale_type: &ScaleType) -> Self {
        Scale {
            root_note: *root_note,
            scale_type: scale_type.clone(),
            pitches: Scale::build_scale(root_note, &scale_type.scale_degrees),
        }
    }

    pub fn from_pitch_string(root: &str, scale_type: &ScaleType) -> Self {
        let p = Pitch::parse(root).unwrap();

        Scale::new(&p, scale_type)
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
        let _ = f.write_fmt(format_args!(
            "{} {} - ",
            self.root_note, self.scale_type.name
        ));

        for pitch in &self.pitches {
            let _ = f.write_fmt(format_args!("{} ", pitch));
        }

        let _ = f.write_str(" - ");

        for degree in &self.scale_type.scale_degrees {
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
    use crate::{modes, pitch::PitchClass};

    #[test]
    fn compute_scale_test() {
        let a = Pitch::new(PitchClass::A);
        let ionian = ScaleType {
            name: "ionian".to_string(),
            scale_degrees: modes::IONIAN.to_vec(),
        };
        let a_ionian = Scale::new(&a, &ionian);

        assert_eq!(
            scale_from_string("A1 B1 C#1 D1 E1 F#1 G#1"),
            a_ionian.pitches
        );
    }
}
