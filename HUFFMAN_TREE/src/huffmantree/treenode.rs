use std::sync::Arc;

use crate::huffmantree::frequency::Frequency;

pub struct TreeNode {
    pub freq_ : Frequency,
    pub left_ : Option<Box<TreeNode>>,
    pub right_ : Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        TreeNode { freq_ : Frequency::new(0), left_ : None, right_ : None, }
    }

    pub fn init_with_freq(the_freq : Frequency) -> TreeNode {
        TreeNode { freq_ : the_freq, left_ : None, right_ : None, }
    }

    pub fn init_with_int(frequency : i32) -> TreeNode {
        TreeNode { freq_ : Frequency::init_with_char_and_int('\0', frequency), left_ : None, right_ : None, }       
    }

    pub fn copy(&self, other : TreeNode) {
        self.freq_ = other.freq_;
        self.left_ = other.left_;
        self.right_ = other.right_;
    }
}