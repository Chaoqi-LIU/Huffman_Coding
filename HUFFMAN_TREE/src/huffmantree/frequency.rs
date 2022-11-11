use std::cmp::Ordering;
use std::cmp::PartialEq;


#[derive(PartialEq, PartialOrd, Debug)]
pub struct Frequency {
    pub frequency_ : i32,
    pub charactor_ : String
}

impl Frequency {
    pub fn new(freq : i32) -> Frequency {
        Frequency { frequency_: (freq), charactor_: (String::from("")) }
    }

    pub fn init_with_char_and_int(c : String, freq : i32) -> Frequency {
        Frequency { frequency_: (freq), charactor_: (c) }
    }

    pub fn get_charactor(&self) -> String {
        self.charactor_.clone()
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
}