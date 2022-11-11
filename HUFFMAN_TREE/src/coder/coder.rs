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
    return output;
}

pub fn decode(huffman_tree : &HuffmanTree, code : String) -> String {
    let mut output : String = String::from("");
    let nodes = huffman_tree.get_nodes();
    let mut cur = huffman_tree.get_root();
    for bit in code.chars() {
        if bit == '0' {
            cur = huffman_tree.get_left(cur);
        } else {
            cur = huffman_tree.get_right(cur);
        }

        // huffman_tree.get_left(cur) == 0 when fixed the tree
        if huffman_tree.get_left(cur) == 0 && huffman_tree.get_right(cur) == 0 {
            output += &nodes[cur].freq_.get_charactor();
            cur = huffman_tree.get_root();
        }
    }
    return output;
}