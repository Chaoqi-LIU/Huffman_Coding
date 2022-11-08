// use crate::huffmantree::huffmantree::HuffmanTree;
use crate::huffmantree::frequency::Frequency;
mod huffmantree;



fn main() {
    println!("in main");
    let x = Frequency::init_with_char_and_int('a', 3);
    let y = Frequency::init_with_char_and_int('a', 3);
    println!("x's freq: {}", x.get_frequancy());
    if x == y {
        println!("yes")
    } else {
        println!("no")
    }
}
