use std::collections::HashMap;

use pitch::{Pitch, PitchClass};

use crate::scale::{Scale, ScaleType};

pub mod beat;
pub mod chord;
pub mod interval;
pub mod modes;
pub mod note;
pub mod pitch;
pub mod scale;

fn init_modes(scales: &mut HashMap<String, ScaleType>) {
    let pairs = vec![
        ("ionian", modes::IONIAN.to_vec()),
        ("dorian", modes::DORIAN.to_vec()),
        ("phrygian", modes::PHRYGIAN.to_vec()),
        ("lydian", modes::LYDIAN.to_vec()),
        ("mixolydian", modes::MIXOLYDIAN.to_vec()),
        ("aeolian", modes::AEOLIAN.to_vec()),
        ("locrian", modes::LOCRIAN.to_vec()),
    ];

    for (name, degrees) in pairs {
        scales.insert(
            name.to_string(),
            ScaleType {
                name: name.to_string(),
                scale_degrees: degrees,
            },
        );
    }
}

fn main() {
    let modes = vec![
        "ionian",
        "dorian",
        "phrygian",
        "lydian",
        "mixolydian",
        "aeolian",
        "locrian",
    ];
    let mut scales = HashMap::new();

    init_modes(&mut scales);

    for mode in modes {
        println!(
            "{}",
            Scale::from_pitch_string("E2", scales.get(mode).unwrap())
        );
    }
}
