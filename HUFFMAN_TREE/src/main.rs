use crate::huffmantree::huffmantree::HuffmanTree;
mod huffmantree;

use crate::coder::coder::*;
mod coder;

use crate::bfile::bfile::*;
mod bfile;

use crate::ppm::ppm::ImagePPM;
mod ppm;

use crate::mapreduce::mapreduce::*;
mod mapreduce;

pub fn main() {



    let mut ippm = ImagePPM::new();
    ippm.read_from_file("output1.ppm");
    ippm.compress_to_file("output.dat", "huffmantree.crab");

    let mut ippm2 = ImagePPM::new();
    ippm2.depress_from_file("output.dat", "huffmantree.crab");
    ippm2.write_to_file("output2.ppm");

    
}