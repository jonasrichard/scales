use crate::interval::Interval;

const MINOR: [Interval; 3] = [Interval::Unison, Interval::Minor3rd, Interval::Perfect5th];
const MAJOR: [Interval; 3] = [Interval::Unison, Interval::Major3rd, Interval::Perfect5th];
// Tritone is not expressing that this is a flat 5th (because it can be a sharp 4th as well)
const DIMINISED: [Interval; 3] = [Interval::Unison, Interval::Minor3rd, Interval::Tritone];

// This is Cmin7
const MINOR7: [Interval; 4] = [
    Interval::Unison,
    Interval::Minor3rd,
    Interval::Perfect5th,
    Interval::Minor7th,
];

// This is Cmaj7
const MAJOR7: [Interval; 4] = [
    Interval::Unison,
    Interval::Major3rd,
    Interval::Perfect5th,
    Interval::Major7th,
];

// This is C7
const DOMINANT7: [Interval; 4] = [
    Interval::Unison,
    Interval::Major3rd,
    Interval::Perfect5th,
    Interval::Minor7th,
];

// Cmin7b5
const HALF_DIMINISED: [Interval; 4] = [
    Interval::Unison,
    Interval::Minor3rd,
    Interval::Tritone,
    Interval::Minor7th,
];

// Cdim7
const DIMINISED7: [Interval; 4] = [
    Interval::Unison,
    Interval::Minor3rd,
    Interval::Tritone,
    // Double flat 7
    Interval::Major6th,
];

// Csus2
const SUSPENDED2: [Interval; 3] = [Interval::Unison, Interval::Major2nd, Interval::Perfect5th];

// Csus4
const SUSPENDED4: [Interval; 3] = [Interval::Unison, Interval::Perfect4th, Interval::Perfect5th];

// TODO augmented and suspended chords
//
// https://www.all-guitar-chords.com/
//
// Chord can have major quality or minor quality
