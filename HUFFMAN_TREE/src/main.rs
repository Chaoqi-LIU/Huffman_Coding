// use crate::huffmantree::huffmantree::HuffmanTree;
use crate::huffmantree::frequency::Frequency;
mod huffmantree;








fn main() {
    let x = Frequency::new(3);
    let y = Frequency::InitWithCharAndInt('a', 1);
    if x < y {
        print!("<")
    } else {
        print!(">")
    }
}
