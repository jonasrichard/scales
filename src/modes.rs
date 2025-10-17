use crate::{pitch::Accidental, scale::ScaleDegree};

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
