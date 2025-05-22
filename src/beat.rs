use crate::note::Note;

pub struct Staff {
    pub measures: Vec<Measure>,
}

pub struct Measure {
    pub time_signature: TimeSignature,
    pub notes: Vec<Note>,
}

pub struct TimeSignature {
    /// Number of beats
    pub numerator: u8,
    pub denominator: u8,
}
