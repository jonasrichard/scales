use crate::pitch::{Accidental, Pitch, PitchClass};

#[derive(Clone, Copy, Debug)]
pub struct ScaleDegree {
    pub degree: u8,
    pub modifier: Accidental,
}

#[derive(Clone, Debug)]
pub struct Scale {
    pub root_note: Pitch,
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

/// Interval distances of scale degrees.
pub const SCALE_DEGREES: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];

impl Scale {
    pub fn new(root_note: &Pitch, degrees: &[ScaleDegree]) -> Self {
        Scale {
            root_note: *root_note,
            scale_degrees: degrees.to_vec(),
            pitches: Scale::build_scale(root_note, degrees),
        }
    }

    pub fn build_scale(root_note: &Pitch, degrees: &[ScaleDegree]) -> Vec<Pitch> {
        let mut result = vec![*root_note];
        let mut i = 1;
        let mut last_pitch = *root_note;
        let mut last_degree = SCALE_DEGREES[0] as i8;

        loop {
            let next_degree = degrees[i];
            let next_num_degree = SCALE_DEGREES[(next_degree.degree - 1) as usize] as i8;
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

//pub const DORIAN: [Interval; 7] = [
//    Interval::Unison,
//    Interval::Major2nd,
//    Interval::Minor3rd,
//    Interval::Perfect4th,
//    Interval::Perfect5th,
//    Interval::Major6th,
//    Interval::Minor7th,
//];

//pub const PHRYGIAN: [Interval; 7] = [
//    Interval::Unison,
//    Interval::Minor2nd,
//    Interval::Minor3rd,
//    Interval::Perfect4th,
//    Interval::Perfect5th,
//    Interval::Minor6th,
//    Interval::Minor7th,
//];
//
//// I still don't have a good modeling idea how to express the flat 4th here.
//pub const LYDIAN: [Interval; 7] = [
//    Interval::Unison,
//    Interval::Major2nd,
//    Interval::Major3rd,
//    Interval::Tritone,
//    Interval::Perfect5th,
//    Interval::Major6th,
//    Interval::Major7th,
//];
//
//pub const MIXOLYDIAN: [Interval; 7] = [
//    Interval::Unison,
//    Interval::Major2nd,
//    Interval::Major3rd,
//    Interval::Perfect4th,
//    Interval::Perfect5th,
//    Interval::Major6th,
//    Interval::Minor7th,
//];
//
//pub const AEOLIAN: [Interval; 7] = [
//    Interval::Unison,
//    Interval::Major2nd,
//    Interval::Minor3rd,
//    Interval::Perfect4th,
//    Interval::Perfect5th,
//    Interval::Minor6th,
//    Interval::Minor7th,
//];
//
//pub const LOCRIAN: [Interval; 7] = [
//    Interval::Unison,
//    Interval::Minor2nd,
//    Interval::Minor3rd,
//    Interval::Perfect4th,
//    Interval::Tritone,
//    Interval::Minor6th,
//    Interval::Minor7th,
//];

// TODO implement a feature like, one F lydian I want chord progression:
// I iv V III or something

// Here we need to learn const generic

pub fn compute_scale(root_note: &Pitch, scale: &[ScaleDegree; 7]) -> Vec<Pitch> {
    let mut new_scale = vec![];

    for (i, degree) in scale.iter().enumerate() {
        let distance = SCALE_DEGREES[i];

        if distance == 0 {
            new_scale.push(root_note.clone());
        }
    }

    new_scale
}

pub fn scale_from_string(scale: &str) -> Vec<Pitch> {
    let mut result = vec![];
    let scale_pitches: Vec<_> = scale.split_whitespace().collect();

    for sp in scale_pitches {
        let p = Pitch::parse(sp).unwrap();

        result.push(p);
    }

    result
}

pub fn display_scale(scale: &[Pitch]) {
    for pitch in scale {
        print!("{} ", pitch);
    }

    println!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_scale_test() {
        let a = Pitch::new(PitchClass::A);
        let a_ionian = compute_scale(&a, &IONIAN);

        assert_eq!(scale_from_string("A1 B1 C#1 D1 E1 F#1 G#1"), a_ionian);
    }
}
