use std::collections::HashMap;
use crate::huffmantree::frequency::Frequency;

pub struct TreeNode {
    // todo
    x : f32
}

impl TreeNode {
    pub fn new() -> TreeNode {
        TreeNode { x: (0.0) }
    }
}

static _max_print_height: usize = 9;

pub struct HuffmanTree {
    root_ : Option<Box<TreeNode>>,
    bitsMap_ : std::collections::HashMap<char, Vec<bool>>,
}

impl HuffmanTree {
    pub fn new(frequancies : Vec<Frequency>) -> HuffmanTree {
        HuffmanTree {
            root_  : std::option::Option::Some(Box::new(TreeNode::new())),
            bitsMap_ : std::collections::HashMap::new(),
        }
    }

    // pub fn new(bfile : BinaryFileReader) -> HuffmanTree { 
    //     todo!();
    // } 

    // // copy constructor
    // pub fn new(other : HuffmanTree) -> HuffmanTree {
    //     Self {
    //         root_ : other.root_,
    //         bitsMap_ : other.bitsMap_,
    //     }
    // }
}