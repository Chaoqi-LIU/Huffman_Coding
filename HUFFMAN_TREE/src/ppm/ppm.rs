use std::fs;
use std::fs::File;
use std::io::{Write, Error};

use crate::huffmantree::huffmantree::HuffmanTree;
mod huffmantree;

use crate::bfile::bfile::*;
mod bfile;

use coder::coder::*;
mod coder;

#[derive(PartialEq, Debug)]
pub struct Pixel {
    red_ : i32,
    green_ : i32,
    blue_ : i32
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel {
            red_ : 0,
            green_ : 0,
            blue_ : 0
        }
    }

    pub fn get_red(&self) -> i32 {
        return self.red_;
    }

    pub fn get_green(&self) -> i32 {
        return self.green_;
    }

    pub fn get_blue(&self) -> i32 {
        return self.blue_;
    }

    fn eq(&self, other: &Self) -> bool {
        (self.red_== other.red_) && (self.green_ == other.green_) && (self.blue_ == other.blue_);
    }
}

// 1. read ppm
// 2. write ppm
// 3. compress ppm
// 4. depress ppm

pub struct ImagePPM {
    height_ : i32,
    width_ : i32,
    max_color_value_ : i32,
    pixels_ : Vec<Pixel>
}

impl ImagePPM {
    pub fn new() -> ImagePPM {
        ImagePPM {
            height_ : 0,
            width_ : 0,
            max_color_value_ : 0,
            pixels_ : Vec<Pixel>::new()
        }
    }

    pub fn read_from_file(&mut self, file : &str) {
        let mut cnt = 0;
        if let Ok(lines) = read_lines(file) {
            for line in lines {
                if cnt == 0 { cnt += 1; continue; }
                else if cnt == 1 {
                    let comps = line.split(" ");
                    
                }
            }
        }
    }

    pub fn write_to_file(&self, file : &str) {
        todo()
    }

    pub fn compress() -> String {
        todo()
    }

    pub fn depress(&self, code : String) -> ImagePPM {
        todo()
    }

    fn get_string() -> String {
        let mut ret = "".to_string();
        ret += &"P3 \n";
        ret += &width_.to_string();
        ret += &" ";
        ret += &height_.to_string();
        
    }
}