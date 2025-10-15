use pitch::{Pitch, PitchClass};

use crate::scale::Scale;

pub mod beat;
pub mod chord;
pub mod interval;
pub mod note;
pub mod pitch;
pub mod scale;

fn main() {
    println!(
        "{}",
        Scale::new(&Pitch::new(PitchClass::E), "ionian", &scale::IONIAN)
    );

    println!(
        "{}",
        Scale::new(&Pitch::new(PitchClass::E), "dorian", &scale::DORIAN)
    );
    println!(
        "{}",
        Scale::new(&Pitch::new(PitchClass::E), "phrygian", &scale::PHRYGIAN)
    );
    println!(
        "{}",
        Scale::new(&Pitch::new(PitchClass::E), "lydian", &scale::LYDIAN)
    );
    println!(
        "{}",
        Scale::new(&Pitch::new(PitchClass::E), "mixolydian", &scale::MIXOLYDIAN)
    );
    println!(
        "{}",
        Scale::new(&Pitch::new(PitchClass::E), "aeolian", &scale::AEOLIAN)
    );
    println!(
        "{}",
        Scale::new(&Pitch::new(PitchClass::E), "locrian", &scale::LOCRIAN)
    );
}
