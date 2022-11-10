use huffmantree::huffmantree::HuffmanTree;
mod huffmantree;

use encoder::encoder::*;
mod encoder;

pub fn main() {
    let mut ht = HuffmanTree::new();
    ht.build_tree_from_text("test.txt");
    // encode(ht, "s");
}