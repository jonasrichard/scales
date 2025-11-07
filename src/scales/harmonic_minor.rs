//! Modes of harmonic minor
use crate::{pitch::Accidental, scale::ScaleDegree};

pub const AEOLIAN_HARMONIC: [ScaleDegree; 7] = [
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
        modifier: Accidental::Natural,
    },
];

pub const LOCRIAN_NATURAL_6: [ScaleDegree; 7] = [
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
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 7,
        modifier: Accidental::Flat,
    },
];

pub const IONIAN_SHARP_5: [ScaleDegree; 7] = [
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
        modifier: Accidental::Sharp,
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

pub const DORIAN_SHARP_4: [ScaleDegree; 7] = [
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
        modifier: Accidental::Flat,
    },
];

pub const PHRYGIAN_DOMINANT: [ScaleDegree; 7] = [
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
        modifier: Accidental::Flat,
    },
    ScaleDegree {
        degree: 7,
        modifier: Accidental::Flat,
    },
];

pub const LYDIAN_SHARP_2: [ScaleDegree; 7] = [
    ScaleDegree {
        degree: 1,
        modifier: Accidental::Natural,
    },
    ScaleDegree {
        degree: 2,
        modifier: Accidental::Sharp,
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

// Altered scale and this is melodic minor mode
pub const SUPER_LOCRIAN: [ScaleDegree; 7] = [
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
        modifier: Accidental::Flat,
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
        modifier: Accidental::DoubleFlat,
    },
];
