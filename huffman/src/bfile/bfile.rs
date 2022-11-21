use crate::huffmantree::huffmantree::HuffmanTree;

use std::fs;
use std::fs::File;
use std::io::{Write, Error};

use std::io::BufWriter;
use std::io::BufReader;

pub use bitbit::writer::BitWriter;
pub use bitbit::reader::BitReader;
pub use bitbit::reader::MSB;

use std::process::exit;

pub fn file_error() {
    println!("\ncheck file path or content style\n");
    exit(0);
}

pub fn read_from_file(file : &str) -> Result<String, Error> {
    return fs::read_to_string(file);
}

pub fn read_from_bfile(bfile : & str) -> Result<String, Error> {
    let mut res = "".to_string();
    let file = File::open(bfile);
    match file {
        Ok(_) => {
            let buf_reader = BufReader::new(file.unwrap());
            let mut bit_reader : BitReader<_, MSB> = BitReader::new(buf_reader);
            let mut bit = bit_reader.read_bit();
            while (&bit).is_ok() {
                if bit.unwrap() { res.push_str("1"); } 
                else { res.push_str("0"); }
                bit = bit_reader.read_bit();
            }
            return Ok(res);
        },
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "cannot open file",
            ));
        }
    }
}

pub fn write_to_file(file : &str, content : String) -> Result<(), Error> {
    let mut output = File::create(file)?;
    return write!(output, "{}", content);
}

pub fn write_to_bfile(bfile : &str, content : String) -> Result<(), Error> {
    let bfile = File::create(bfile);
    let mut buf_writer = BufWriter::new(bfile.unwrap());
    let mut bit_writer = BitWriter::new(&mut buf_writer);

    for bit in content.chars() {
        if bit == '\0' { continue; }
        bit_writer.write_bit(bit == '1').ok();
    }
    return buf_writer.flush();
}


pub fn build_tree_from_file(file : &str) -> Result<HuffmanTree, Error> {
    let mut huffman_tree = HuffmanTree::new();
    let text = read_from_file(file);
    match text {
        Ok(_) => {
            huffman_tree.build_tree_from_string(text.unwrap());
            return Ok(huffman_tree);
        },
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "cannot open file",
            ));
        }
    }
    
}

pub fn build_tree_from_text(text : String) -> HuffmanTree {
    let mut huffman_tree = HuffmanTree::new();
    huffman_tree.build_tree_from_string(text);
    return huffman_tree;
}

pub fn read_huffmantree(file : &str) -> Result<HuffmanTree, Error> {
    let mut huffman_tree = HuffmanTree::new();
    let mut idx : usize = 0;
    let result = fs::read_to_string(file);
    match result {
        Ok(_) => {
            let mut reader = result.unwrap();
            reader.pop();
            for line in reader.split("🦀") {
                let comps : Vec<&str> = line.split("|").collect();
                idx = huffman_tree.insert(comps[2].to_string().parse::<i32>().unwrap(), comps[3].to_string());
                huffman_tree.set_left(idx, comps[0].to_string().parse::<usize>().unwrap());
                huffman_tree.set_right(idx, comps[1].to_string().parse::<usize>().unwrap());
            }
            huffman_tree.set_root(idx);
            huffman_tree.build_map(huffman_tree.get_root(), &mut vec![]);
            return Ok(huffman_tree);
        }
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "cannot open file",
            ));
        }
    };
}

pub fn huffmantree_to_stirng(huffman_tree : &HuffmanTree) -> String {
    let mut content : String = "".to_string();
    for node in &huffman_tree.get_nodes()[1..(huffman_tree.get_idx() + 1)] {
        content += &(node.left_.to_string() + &"|");
        content += &(node.right_.to_string() + &"|");
        content += &(node.freq_.get_frequancy().to_string() + &"|");
        content += &(node.freq_.get_charactor().to_string() + &"🦀");
    }
    return content;
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
    return write!(output, "{}", huffmantree_to_stirng(huffman_tree));
}
