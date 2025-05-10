use crate::pitch::Pitch;

#[derive(Debug)]
pub enum DurationName {
    Whole = 1,
    Half = 2,
    Quarter = 4,
    Eigth = 8,
    Sixteenth = 16,
    ThirtySecond = 32,
    SixtyFourth = 64,
}

#[derive(Debug)]
pub struct Duration {
    pub name: DurationName,
    /// 0 means no dots, 1 dot, 2 dots, etc.
    pub dots: u8,
}

#[derive(Debug)]
pub enum Note {
    Tone(Pitch, Duration),
    Rest(Duration),
}
