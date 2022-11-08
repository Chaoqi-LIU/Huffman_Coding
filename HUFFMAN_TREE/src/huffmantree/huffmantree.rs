use std::collections::HashMap;
use crate::huffmantree::frequency::Frequency;

pub struct TreeNode {
    freq : Frequency,
    left : Option<Box<TreeNode>>,
    right : Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        TreeNode { freq : Frequency::new(0), left : None, right : None, }
    }

    pub fn Init(theFreq : Frequency) -> TreeNode {
        TreeNode { freq : theFreq, left : None, right : None, }
    }

    pub fn InitWithInt(frequency : i32) -> TreeNode {
        TreeNode { freq : Frequency::InitWithCharAndInt('\0', frequency), left : None, right : None, }       
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

    pub fn CopyConstructor(other : HuffmanTree) -> HuffmanTree {
        HuffmanTree {
            root_ : other.root_,
            bitsMap_ : other.bitsMap_,
        }
    }

    // pub fn ReadFromBFile(bfile : BinaryFileReader) -> HuffmanTree { 
    //     todo!();
    // } 

    
}