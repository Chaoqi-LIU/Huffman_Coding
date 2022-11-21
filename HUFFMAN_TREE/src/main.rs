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

use std::env;
use std::process::exit;


pub fn invalid_argument() {
    println!("\n\n ======== invalid argument ========\n see '-- help' for more information \n ==================================\n\n");
    exit(0);
}

pub fn help() {
    println!();
    println!();
    println!("Usage: cargo run -- [options]");
    println!();
    println!("Options: ");
    println!("  -- help                                                 print this help message");
    println!("  -- encode input_file -o output_file huffman_tree        encode the content in 'input_file'[.txt], write encoded content to 'output_file'[.dat], and save the huffman tree to 'huffman_tree'[.crab] ");
    println!("  -- encode input_file huffman_tree -o output_file        encode the content in 'input_file'[.txt] with 'huffman_tree'[.crab] provided, write encoded content to 'output_file'[.dat] ");
    println!("  -- decode input_file huffman_tree -o output_file        decode the content in 'input_file'[.txt] with 'huffman_tree'[.crab] provided, write decoded content to 'output_file'[.txt] ");
    println!("  -- compress input_file -o output_file huffman_tree      compress the image in 'input_file'[.ppm], write compressed image to 'output_file'[.dat] and save the huffman tree to 'huffman_tree'[.crab] ");
    println!("  -- compress input_file huffman_tree -o output_file      compress the image in 'input_file'[.ppm] with 'huffman_tree'[.crab] provided, write compressed image to 'output_file'[.dat] ");
    println!("  -- depress intput_file huffman_tree -o output_file      depress the image in 'input_file'[.dat] with 'huffman_tree'[.crab] provided, write depressed image to 'output_file'[.ppm] ");
    println!("  -- print huffman_tree                                   print the 'huffman_tree'[.crab] ");
    println!();
    println!();
    exit(0);
}

pub fn cannot_open_file() {
    println!("\ncannnot open the file, check file path");
}

pub fn cannot_read_file() {
    println!("\ncannot read the file, check content style");
}

pub fn get_o_pos(args : & Vec<String>) -> i32 {
    let mut o_pos : i32 = -1;
    let mut o_cnt : i32 = 0;
    for i in 0..args.len() {
        if args[i].as_str() == "-o" {
            o_pos = i as i32;
            o_cnt += 1;
        }
    }
    if o_pos == -1 || o_cnt != 1 { invalid_argument(); }
    return o_pos;
}

pub fn main() {

    let args : Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "help" { help(); }

    if args.len() != 6 && !(args[1] == "print" && args.len() == 3) { invalid_argument(); }

    // print arguments
    let mut i = 0;
    println!("\nargs:");
    for arg in &args { println!("{i}, {arg}"); i += 1 }
    println!();

    let o_pos = get_o_pos(&args);

    match args[1].as_str() {
        // -- encode input_file -o output_file huffman_tree
        // -- encode input_file huffman_tree -o output_file
        "encode" => match o_pos{
            3 => {

            },
            4 => {
                
            },
            _ => invalid_argument()
        },
        // -- decode input_file huffman_tree -o output_file
        "decode" => match o_pos {
            4 => {
                let huffman_tree = read_huffmantree(&args[3]);
                match &huffman_tree {
                    Ok(huffman_tree) => {
                        let content = read_from_file(&args[2]);
                        let encoded = encode(&huffman_tree, content);
                        match write_to_file(&args[5], encoded) {
                            Ok(()) => {
                                
                            },
                            Err(_) => {
                                invalid_argument();
                            }
                        }
                    },
                    _ => cannot_read_file()
                }
            },
            _ => invalid_argument()
        },
        // -- compress input_file -o output_file huffman_tree
        // -- compress input_file huffman_tree -o output_file
        "compress" => match o_pos {
            3 => {
                let mut image = ImagePPM::new();
                image.read_from_file(&args[2]);
                match image.compress_to_file(&args[4], &args[5]) {
                    Ok(()) => {
                        
                    },
                    Err(_) => {
                        invalid_argument();
                    }
                }
            },
            4 => {
                let mut image = ImagePPM::new();
                image.read_from_file(&args[2]);
                match image.compress_to_file_with_tree(&args[5], &args[3]) {
                    Ok(()) => {
                        
                    },
                    Err(_) => {
                        invalid_argument();
                    }
                }
            },
            _ => invalid_argument()
        },
        // -- depress intput_file huffman_tree -o output_file
        "depress" => match o_pos { 
            4 => {
                let mut image = ImagePPM::new();
                image.depress_from_file(&args[2], &args[3]);
                match image.write_to_file(&args[5]) {
                    Ok(()) => {
                        
                    },
                    Err(_) => {
                        invalid_argument();
                    }
                }
            }
            _ => invalid_argument()
        },
        "print" => {
            let mut huffman_tree = read_huffmantree(&args[2]);
            match &mut huffman_tree {
                Ok(huffman_tree) => println!("{}", huffman_tree.print_tree()),
                _ => cannot_read_file()
            }
        }
        _ => invalid_argument()
    };


    
    
    // match args[1].as_str() {
    //     "encode" => match args[3].as_str() {
            
    //         _ => invalid_argument()
    //     },
    //     "decode" => match args[2].as_str() {
    //         _ => invalid_argument()
    //     },
    //     "compress" => match args[2].as_str() {
    //         _ => invalid_argument()
    //     },
    //     "depress" => match args[2].as_str() {
    //         _ => invalid_argument()
    //     },
    //     _ => invalid_argument()
    // };

}


// pub fn main() {

//     // let mut ippm = ImagePPM::new();
//     // ippm.read_from_file("output1.ppm");
//     // ippm.compress_to_file("output.dat", "huffmantree.crab");

//     // let mut ippm2 = ImagePPM::new();
//     // ippm2.depress_from_file("output.dat", "huffmantree.crab");
//     // ippm2.write_to_file("output2.ppm");

// }