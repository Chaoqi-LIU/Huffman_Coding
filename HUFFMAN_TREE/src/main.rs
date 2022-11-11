use crate::huffmantree::huffmantree::HuffmanTree;
mod huffmantree;

use crate::coder::coder::*;
mod coder;

use crate::bfile::bfile::*;
mod bfile;

pub fn main() {

    let mut ht = HuffmanTree::new();
    ht.build_tree_from_text("test.txt");
    write_huffmantree("output.txt", &ht);

    // let res = encode(&ht, "12\n5".to_string());
    // println!("encoded as {}", res);
    // let text = decode(&ht, res.clone());
    // println!("decoded as {}", text.clone());

    // write_to_file("output.txt", text.clone());

    let mut ht_new = HuffmanTree::new();
    read_huffmantree("output.txt", &mut ht_new);
    write_huffmantree("output2.txt", &ht_new);




    // let x1 = ht.get_nodes();
    // let x2 = ht_new.get_nodes();
    // for node in x1 {
    //     println!("char: {}, freq: {}", node.freq_.get_charactor(), node.freq_.get_frequancy());
    // }
    // println!();
    // for node in x2 {
    //     println!("char: {}, freq: {}", node.freq_.get_charactor(), node.freq_.get_frequancy());
    // }








    // println!("original tree");
    // for mp in ht.get_bits_map() {
    //     println!("========");
    //     println!("char: {}", mp.0);
    //     for d in mp.1 {
    //         if *d {
    //             print!("1 ");
    //         } else {
    //             print!("0 ");
    //         }
    //     }
    //     println!();
    // }

    // println!("new tree");
    // for mp in ht_new.get_bits_map() {
    //     println!("========");
    //     println!("char: {}", mp.0);
    //     for d in mp.1 {
    //         if *d {
    //             print!("1 ");
    //         } else {
    //             print!("0 ");
    //         }
    //     }
    //     println!();
    // }
    
}