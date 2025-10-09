use crate::pitch::{Accidental, Pitch, PitchClass};

#[derive(Clone, Copy, Debug)]
pub enum Interval {
    Unison = 0,
    Minor2nd = 1,
    Major2nd = 2,
    Minor3rd = 3,
    Major3rd = 4,
    Perfect4th = 5,
    Tritone = 6,
    Perfect5th = 7,
    Minor6th = 8,
    Major6th = 9,
    Minor7th = 10,
    Major7th = 11,
    Octave = 12,
}

pub const IONIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Major2nd,
    Interval::Major3rd,
    Interval::Perfect4th,
    Interval::Perfect5th,
    Interval::Major6th,
    Interval::Major7th,
];

pub const DORIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Major2nd,
    Interval::Minor3rd,
    Interval::Perfect4th,
    Interval::Perfect5th,
    Interval::Major6th,
    Interval::Minor7th,
];

pub const PHRYGIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Minor2nd,
    Interval::Minor3rd,
    Interval::Perfect4th,
    Interval::Perfect5th,
    Interval::Minor6th,
    Interval::Minor7th,
];

// I still don't have a good modeling idea how to express the flat 4th here.
pub const LYDIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Major2nd,
    Interval::Major3rd,
    Interval::Tritone,
    Interval::Perfect5th,
    Interval::Major6th,
    Interval::Major7th,
];

pub const MIXOLYDIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Major2nd,
    Interval::Major3rd,
    Interval::Perfect4th,
    Interval::Perfect5th,
    Interval::Major6th,
    Interval::Minor7th,
];

pub const AEOLIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Major2nd,
    Interval::Minor3rd,
    Interval::Perfect4th,
    Interval::Perfect5th,
    Interval::Minor6th,
    Interval::Minor7th,
];

pub const LOCRIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Minor2nd,
    Interval::Minor3rd,
    Interval::Perfect4th,
    Interval::Tritone,
    Interval::Minor6th,
    Interval::Minor7th,
];

// TODO implement a feature like, one F lydian I want chord progression:
// I iv V III or something

impl Interval {
    pub fn to_pitch(&self, octave: u8) -> Pitch {
        let mut pitch = Pitch {
            class: PitchClass::C,
            accidental: None,
            octave,
        };

        match *self {
            Interval::Unison => {}
            Interval::Minor2nd => {
                pitch.accidental = Some(Accidental::Sharp);
            }
            Interval::Major2nd => {
                pitch.class = PitchClass::D;
            }
            Interval::Minor3rd => {
                pitch.class = PitchClass::D;
                pitch.accidental = Some(Accidental::Sharp);
            }
            Interval::Major3rd => {
                pitch.class = PitchClass::E;
            }
            Interval::Perfect4th => {
                pitch.class = PitchClass::F;
            }
            Interval::Tritone => {
                pitch.class = PitchClass::F;
                pitch.accidental = Some(Accidental::Sharp);
            }
            Interval::Perfect5th => {
                pitch.class = PitchClass::G;
            }
            Interval::Minor6th => {
                pitch.class = PitchClass::G;
                pitch.accidental = Some(Accidental::Sharp);
            }
            Interval::Major6th => {
                pitch.class = PitchClass::A;
            }
            Interval::Minor7th => {
                pitch.class = PitchClass::A;
                pitch.accidental = Some(Accidental::Sharp);
            }
            Interval::Major7th => {
                pitch.class = PitchClass::B;
            }
            Interval::Octave => {
                pitch.octave += 1;
            }
        }

        pitch
    }
}

impl From<u8> for Interval {
    fn from(value: u8) -> Self {
        match value {
            0 => Interval::Unison,
            1 => Interval::Minor2nd,
            2 => Interval::Major2nd,
            3 => Interval::Minor3rd,
            4 => Interval::Major3rd,
            5 => Interval::Perfect4th,
            6 => Interval::Tritone,
            7 => Interval::Perfect5th,
            8 => Interval::Minor6th,
            9 => Interval::Major6th,
            10 => Interval::Minor7th,
            11 => Interval::Major7th,
            12 => Interval::Octave,
            _ => {
                unreachable!()
            }
        }
    }
}

// Here we need to learn const generic

pub fn compute_scale(root_note: Pitch, scale: &[Interval; 7]) -> Vec<Pitch> {
    let mut new_scale = vec![];

    for interval in scale {
        new_scale.push(root_note.add_interval(interval));
    }

    new_scale
}

pub fn display_scale(scale: &[Pitch]) {
    for pitch in scale {
        print!("{} ", pitch);
    }

    println!()
}
