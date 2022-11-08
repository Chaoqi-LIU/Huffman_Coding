use std::cmp::Ordering;
use std::cmp::PartialEq;


#[derive(PartialEq, PartialOrd)]
pub struct Frequency {
    frequency_ : f32,
    charactor_ : char
}

impl Frequency {
    pub fn getCharactor(&self) -> char {
        self.charactor_
    }

    pub fn getFrequancy(&self) -> f32 {
        self.frequency_
    }
}

// impl PartialEq for Frequency {
//     fn eq(&self, other: &Self) -> bool {
//         (self.frequency_ == other.frequency_) && (self.charactor_ == other.charactor_)
//     }
// }

// impl PartialOrd for Frequency {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.frequency_.partial_cmp(&other.frequency_)
//     }
// }