#[derive(Clone, Copy, Debug)]
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

impl From<u8> for Interval {
    fn from(value: u8) -> Self {
        match value {
            0 => Interval::Unison,
            1 => Interval::Minor2nd,
            2 => Interval::Major2nd,
            3 => Interval::Minor3rd,
            4 => Interval::Major3rd,
            5 => Interval::Perfect4th,
            6 => Interval::Tritone,
            7 => Interval::Perfect5th,
            8 => Interval::Minor6th,
            9 => Interval::Major6th,
            10 => Interval::Minor7th,
            11 => Interval::Major7th,
            12 => Interval::Octave,
            _ => {
                unreachable!()
            }
        }
    }
}
