use std::fs;
use std::fs::File;
use std::io::{Write, Error};
use std::cmp::PartialEq;

use crate::huffmantree::huffmantree::HuffmanTree;

use crate::bfile::bfile::*;

use crate::coder::coder::*;

#[derive(PartialEq, Debug, Clone)]
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

    pub fn get_red(&self) -> i32 { return self.red_; }

    pub fn set_red(&mut self, red : i32) { self.red_ = red; }

    pub fn get_green(&self) -> i32 { return self.green_; }

    pub fn set_green(&mut self, green : i32) { self.green_ = green; }

    pub fn get_blue(&self) -> i32 { return self.blue_; }

    pub fn set_blue(&mut self, blue : i32) { self.blue_ = blue; }

    // fn eq(&self, other: &Self) -> bool {
    //     (self.red_== other.red_) && (self.green_ == other.green_) && (self.blue_ == other.blue_)
    // }

    fn clone(&self) -> Self {
        Pixel {
            red_ : self.red_,
            green_ : self.green_,
            blue_ : self.blue_
        }
    }
}


pub struct ImagePPM {
    height_ : i32,
    width_ : i32,
    max_color_value_ : i32,
    pub pixels_ : Vec<Pixel>
}

impl ImagePPM {
    pub fn new() -> ImagePPM {
        ImagePPM {
            height_ : 0,
            width_ : 0,
            max_color_value_ : 0,
            pixels_ : Vec::new()
        }
    }

    pub fn read_from_file(&mut self, file : &str) {
        if let Ok(lines) = fs::read_to_string(file) {
            self.generate_ppm_with_string(lines);
        }
    }

    pub fn write_to_file(&self, file : &str) -> Result<(), Error> {
        let content = self.get_string();
        let mut output = File::create(file)?;
        return write!(output, "{}", content);
    }

    pub fn compress_to_file(&self, bfile : &str, tree : &str) -> Result<(), Error>  {
        let huffman_tree : HuffmanTree = self.generate_tree();
        match write_to_bfile(bfile, encode(&huffman_tree, self.get_string())) {
            Ok(_) => {
                println!("Successfully write to {bfile}");
            },
            Err(_) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "cannot write to bfile",
                ));
            }
        }
        match write_to_file(tree, huffmantree_to_stirng(&huffman_tree)) {
            Ok(_) => {
                println!("Successfully write to {tree}");
            }
            Err(_) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "cannot write to file",
                ));
            }
        }
        return Ok(());
    }

    pub fn compress_to_file_with_tree(&self, bfile : &str, tree : &str) -> Result<(), Error>  {
        let huffman_tree = read_huffmantree(tree);
        match huffman_tree {
            Ok(huffman_tree) => {
                match write_to_bfile(bfile, encode(&huffman_tree, self.get_string())) {
                    Ok(_) => {
                        println!("Successfully write to {bfile}");
                    },
                    Err(_) => {
                        return Err(Error::new(
                            std::io::ErrorKind::Other,
                            "cannot write to bfile",
                        ));
                    }
                }
            },
            Err(_) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "cannot read huffman tree",
                ));
            }
        }
        return Ok(());
    }

    pub fn depress_from_file(&mut self, bfile : &str, tree : &str) -> Result<(), Error> {
        let huffman_tree = read_huffmantree(tree);
        match huffman_tree {
            Ok(huffman_tree) => {
                let code = read_from_bfile(bfile);
                match code {
                    Ok(code) => {
                        let ppm_str = decode(&huffman_tree, code);
                        self.generate_ppm_with_string(ppm_str);
                        return Ok(());
                    },
                    Err(_) => {
                        return Err(Error::new(
                            std::io::ErrorKind::Other,
                            "cannot open bfile",
                        ));
                    }
                }
            },
            Err(_) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "cannot read huffman tree",
                ));
            }
        }
        
    }

    fn get_string(&self) -> String {
        let mut ret = "".to_string();
        ret += &"P3 \n";
        ret += &(self.width_.to_string() + &" ");
        ret += &(self.height_.to_string() + &"\n");
        ret += &(self.max_color_value_.to_string() + &"\n");
        for row in 0..self.height_ {
            for col in 0..self.width_ {
                let cur : &Pixel = &self.pixels_[(row * self.width_ + col) as usize];
                ret += &(cur.get_red().to_string() + &"\n");
                ret += &(cur.get_green().to_string() + &"\n");
                ret += &(cur.get_blue().to_string() + &"\n");
            }
        }
        return ret;
    }

    pub fn generate_tree(&self) -> HuffmanTree {
        return build_tree_from_text(self.get_string());
    }

    fn generate_ppm_with_string(&mut self, lines : String) {
        let mut cnt = 0;
        let mut pixel = Pixel::new();
        for line in lines.lines() {
            if cnt == 0 { cnt += 1; }  // P3
            else if cnt == 1 {  // width height
                let comps : Vec<&str> = line.split(" ").collect();
                self.width_ = comps[0].parse().unwrap();
                self.height_ = comps[1].parse().unwrap();
                cnt += 1;
            } else if cnt == 2 {
                self.max_color_value_ = line.parse().unwrap();
                cnt += 1;
            } else if cnt == 3 {  // read red_
                pixel.set_red(line.parse().unwrap());
                cnt += 1;
            } else if cnt == 4 {  // read green_
                pixel.set_green(line.parse().unwrap());
                cnt += 1;
            } else if cnt == 5 {  // read blud_
                pixel.set_blue(line.parse().unwrap());
                self.pixels_.push(pixel.clone());
                cnt = 3;
            }
        }
    }
}