use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::ops;

#[derive(PartialEq, PartialOrd)]
pub struct Frequency {
    pub frequency_ : i32,
    pub charactor_ : char
}

impl Frequency {
    pub fn new(freq : i32) -> Frequency {
        Frequency { frequency_: (freq), charactor_: ('\0') }
    }

    pub fn init_with_char_and_int(c : char, freq : i32) -> Frequency {
        Frequency { frequency_: (freq), charactor_: (c) }
    }

    pub fn get_charactor(&self) -> char {
        self.charactor_
    }

    pub fn get_frequancy(&self) -> i32 {
        self.frequency_
    }

    fn eq(&self, other: &Self) -> bool {
        (self.frequency_ == other.frequency_) && (self.charactor_ == other.charactor_)
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.frequency_.partial_cmp(&other.frequency_)
    }

    // fn add(&self, _rhs: &Self) -> Frequency {
           
    // }
}