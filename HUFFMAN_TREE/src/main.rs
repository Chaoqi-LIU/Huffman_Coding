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

pub fn main() {





    // let mut ht = build_tree_from_text("test.txt");
    // let en =  encode(&ht, "there’s a way to simplify this process: 
    // we can create a shortcut to a path with
    // the use keyword once, and then use the 
    // shorter name everywhere else in the scope go fuck.".to_string());
    // println!("en : {}", &en);
    
    // let de = decode(&ht, en);
    // println!("de : {}", &de);
    

    let mut ippm = ImagePPM::new();
    ippm.read_from_file("output1.ppm");
    ippm.compress_to_file("output.dat", "huffmantree.txt");

    let mut ippm2 = ImagePPM::new();
    ippm2.depress_from_file("output.dat", "huffmantree.txt");
    ippm2.write_to_file("output2.ppm");
    // write_huffmantree("output.txt", &ht);

    // let res = encode(&ht, "12\n5".to_string());
    // println!("encoded as {}", res);
    // let text = decode(&ht, res.clone());
    // println!("decoded as {}", text.clone());

    // write_to_file("output.txt", text.clone());

    // let mut ht_new = HuffmanTree::new();
    // read_huffmantree("output.txt", &mut ht_new);
    // write_huffmantree("output2.txt", &ht_new);




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