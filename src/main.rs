use pitch::Pitch;
use scale::display_scale;

pub mod beat;
pub mod chord;
pub mod note;
pub mod pitch;
pub mod scale;

fn main() {
    let c_dorian = scale::compute_scale(Pitch::new(pitch::PitchClass::C), &scale::DORIAN);

    display_scale(&c_dorian);
}
