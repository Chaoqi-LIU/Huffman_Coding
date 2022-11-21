use crate::huffmantree::huffmantree::HuffmanTree;

pub fn encode(huffman_tree : &HuffmanTree, text : String) -> String {
    let mut output : String = String::from("");
    let map = huffman_tree.get_bits_map();
    
    for c in text.chars() {
        if !map.contains_key(&c.to_string()) {
            panic!("fail to encode");
        }
        for bit in &map[&c.to_string()] {
            if *bit {
                output += "1";
            } else {
                output += "0";
            }
        }
    }
    let mut diff = (8 - (output.len() % 8)) % 8;
    for _i in 0..diff {
        output = "0".to_string()+ &output;
    }
    for _i in 0..8 {
        output = (diff & 1).to_string() + &output; 
        diff >>= 1;
    }
    return output;
}

pub fn decode(huffman_tree : &HuffmanTree, code : String) -> String {
    let mut output : String = String::from("");
    let nodes = huffman_tree.get_nodes();
    let mut cur = huffman_tree.get_root();

    let mut cnt : i32 = 0;
    let mut vec : Vec<&str> = code.as_str().split("").collect();
    vec.remove(0);
    for i in 0..8 {
        if vec[0] == "1" { cnt += 1 << (7 - i); }
        vec.remove(0); 
    }
    for _i in 0..cnt { vec.remove(0); }
    vec.pop();
    for bit in vec {
        if bit == "0" { cur = huffman_tree.get_left(cur); } 
        else { cur = huffman_tree.get_right(cur); }

        if huffman_tree.get_left(cur) == 0 && huffman_tree.get_right(cur) == 0 {
            output += &nodes[cur].freq_.get_charactor();
            cur = huffman_tree.get_root();
        }
    }
    return output;
}