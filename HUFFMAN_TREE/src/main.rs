use huffmantree::huffmantree::HuffmanTree;
mod huffmantree;

pub fn main() {
    let mut ht = HuffmanTree::new();
    ht.build_tree_from_text("test.txt");
}