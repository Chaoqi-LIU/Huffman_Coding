use std::cmp::Ordering;
use std::cmp::PartialEq;


#[derive(PartialEq, PartialOrd)]
pub struct Frequency {
    frequency_ : i32,
    charactor_ : char
}

impl Frequency {
    pub fn new(freq : i32) -> Frequency {
        Frequency { frequency_: (freq), charactor_: ('\0') }
    }

    pub fn InitWithCharAndInt(c : char, freq : i32) -> Frequency {
        Frequency { frequency_: (freq), charactor_: (c) }
    }

    pub fn getCharactor(&self) -> char {
        self.charactor_
    }

    pub fn getFrequancy(&self) -> i32 {
        self.frequency_
    }

    fn eq(&self, other: &Self) -> bool {
        (self.frequency_ == other.frequency_) && (self.charactor_ == other.charactor_)
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.frequency_.partial_cmp(&other.frequency_)
    }
}