use pitch::{Pitch, PitchClass};
use scale::display_scale;

use crate::scale::Scale;

pub mod beat;
pub mod chord;
pub mod interval;
pub mod note;
pub mod pitch;
pub mod scale;

fn main() {
    display_scale(&Scale::new(&Pitch::new(PitchClass::C), &scale::IONIAN).pitches);
    display_scale(&Scale::new(&Pitch::new(PitchClass::E), &scale::IONIAN).pitches);
}
