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

const IONIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Major2nd,
    Interval::Major3rd,
    Interval::Perfect4th,
    Interval::Perfect5th,
    Interval::Major6th,
    Interval::Major7th,
];

const DORIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Major2nd,
    Interval::Minor3rd,
    Interval::Perfect4th,
    Interval::Perfect5th,
    Interval::Major6th,
    Interval::Minor7th,
];

const PHRYGIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Minor2nd,
    Interval::Minor3rd,
    Interval::Perfect4th,
    Interval::Perfect5th,
    Interval::Minor6th,
    Interval::Minor7th,
];

const LYDIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Major2nd,
    Interval::Major3rd,
    Interval::Tritone,
    Interval::Perfect5th,
    Interval::Major6th,
    Interval::Major7th,
];

const MIXOLYDIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Major2nd,
    Interval::Major3rd,
    Interval::Perfect4th,
    Interval::Perfect5th,
    Interval::Major6th,
    Interval::Minor7th,
];

const AEOLIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Major2nd,
    Interval::Minor3rd,
    Interval::Perfect4th,
    Interval::Perfect5th,
    Interval::Minor6th,
    Interval::Minor7th,
];

const LOCRIAN: [Interval; 7] = [
    Interval::Unison,
    Interval::Minor2nd,
    Interval::Minor3rd,
    Interval::Perfect4th,
    Interval::Tritone,
    Interval::Minor6th,
    Interval::Minor7th,
];
