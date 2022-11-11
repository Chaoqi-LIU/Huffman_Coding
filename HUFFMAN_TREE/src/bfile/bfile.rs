use crate::huffmantree::huffmantree::HuffmanTree;

use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

pub fn read_from_file(file : &str) -> String {
    return fs::read_to_string(file).unwrap();
}

pub fn write_to_file(file : &str, content : String) -> Result<(), Error> {
    let mut output = File::create(file)?;
    return write!(output, "{}", content);
}

pub fn read_huffmantree(file : &str, huffman_tree : &mut HuffmanTree) -> Result<(), Error> {
    // let mut idx : usize = 0;
    // let file = File::open(file)?;
    // let reader = BufReader::new(file);
    // for line in reader.lines() {
    //     if let Ok(line) = line {
    //         let comps : Vec<&str> = line.split("|").collect();
    //         idx = huffman_tree.insert(comps[2].to_string().parse::<i32>().unwrap(), comps[3].to_string());
    //         huffman_tree.set_left(idx, comps[0].to_string().parse::<usize>().unwrap());
    //         huffman_tree.set_right(idx, comps[1].to_string().parse::<usize>().unwrap());
    //     }
    // }
    let mut idx : usize = 0;
    let mut reader = fs::read_to_string(file).unwrap();
    reader.pop();
    // println!("reader: {}", reader);
    for line in reader.split("🦀") {
        let comps : Vec<&str> = line.split("|").collect();
        idx = huffman_tree.insert(comps[2].to_string().parse::<i32>().unwrap(), comps[3].to_string());
        huffman_tree.set_left(idx, comps[0].to_string().parse::<usize>().unwrap());
        huffman_tree.set_right(idx, comps[1].to_string().parse::<usize>().unwrap());
    }
    huffman_tree.set_root(idx);
    huffman_tree.build_map(huffman_tree.get_root(), &mut vec![]);
    return Ok(());
}

/*
file_name
---------------------------------------------
left_node right_node freq char 
*/
pub fn write_huffmantree(file : &str, huffman_tree : &HuffmanTree) -> Result<(), Error> {
    if huffman_tree.get_root() == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "empty Huffman Tree",
        ));
    }
    let mut output = File::create(file)?;
    let mut result = Ok(());
    for node in &huffman_tree.get_nodes()[1..(huffman_tree.get_idx() + 1)] {
        result = write!(output, "{}|{}|{}|{}🦀", node.left_, node.right_, node.freq_.get_frequancy(), node.freq_.get_charactor());
    }
    return result;
}