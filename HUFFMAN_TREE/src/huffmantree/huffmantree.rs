use std::collections::HashMap;
use crate::huffmantree::frequency::Frequency;
use crate::huffmantree::treenode::TreeNode;

// extern crate queues;
// use queues::*;

static _max_print_height: usize = 9;

// #[derive(PartialEq, PartialOrd)]
pub struct HuffmanTree {
    root_ : Option<Box<TreeNode>>,
    bits_map_ : std::collections::HashMap<char, Vec<bool>>,
}

impl HuffmanTree {
    pub fn new(frequancies : Vec<Frequency>) -> HuffmanTree {
        HuffmanTree {
            root_  : std::option::Option::Some(Box::new(TreeNode::new())),
            bits_map_ : std::collections::HashMap::new(),
        }
    }

    // pub fn init_with_binary_file_reader(bfile : BinaryFileReader) -> HuffmanTree {
    //     todo!();
    // }

    pub fn copy_constructor(other : HuffmanTree) -> HuffmanTree {
        HuffmanTree {
            root_ : other.root_,
            bits_map_ : other.bits_map_,
        }
    }

    // pub fn decode_file(bfile : BinaryFileReader) -> String {
    //     todo!();
    // }

    // pub fn write_to_file(data : String, bfile : BinaryFileWriter) {
    //     todo!();
    // }

    // pub fn write_to_file_char(c : char, bfile : BinaryFileWriter) {
    //     todo!();
    // }

    // pub fn write_tree(bfile : BinaryFileWriter) {
    //     todo!();
    // }

    // pub fn print_in_order() {
    //     todo!();
    // }

    // pub fn print(out : ostream) {
    //     todo!();
    // }
    
    // pub fn copy_huffmantree(&self, other : HuffmanTree) {
    //     todo!()
    // }

    // pub fn copy_recursive_treenode(current : Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    //     if current.is_none() {
    //         return None;
    //     }

    //     let left = copy_recursive_treenode(Some<Box<TreeNode>::(current.unwrap().left_)>);

    //     Some(Box(TreeNode {
    //         freq_ : current.unwrap().freq_,
    //         left_ : copy_recursive_treenode(current.unwrap().left_).unwrap(),
    //         right_ : copy_recursive_treenode(current.unwrap().right_).unwrap()
    //     }));
    // }

    // pub fn copy_treenode(&self, )



    

    
    // pub fn read_tree(bfile : BinaryFileReader) -> Option<Box<TreeNode>> {
    //     todo!()
    // }

    pub fn build_map(current : Option<Box<TreeNode>>, path : Vec<bool>) {
        todo!()
    }

    pub fn print_inorder(current : Option<Box<TreeNode>>) {
        todo!()
    }

    pub fn remove_smallest(single_queue : &mut Vec<Option<Box<TreeNode>>>, 
                           merge_queue : &mut Vec<Option<Box<TreeNode>>>) -> Option<Box<TreeNode>> {
        let mut to_return : Option<Box<TreeNode>> = None;
        if single_queue.len() == 0 {
            // to_return = Some(Box::new(merge_queue[0].unwrap().clone()));
            to_return = TreeNode {
                freq_ : 
                
            }
            merge_queue.remove(0);
        } else if merge_queue.len() == 0 {
            to_return = single_queue[0];
            single_queue.remove(0);
            let x = single_queue[0].unwrap().freq_;
        } else if single_queue[0].unwrap().freq_ < merge_queue[0].unwrap().freq_ {
            to_return = single_queue[0];
            single_queue.remove(0);
        } else {
            to_return = merge_queue[0];
            merge_queue.remove(0);
        }
        return to_return;
    }
    

    // pub fn build_tree(&self, frequancies : Vec<Frequency>) {
    //     let mut single_queue : Vec<Option<Box<TreeNode>>> = vec![];
    //     let mut merge_queue : Vec<Option<Box<TreeNode>>> = vec![];
    //     for i in frequancies {
    //         single_queue.push(Some(Box::new(TreeNode::init_with_freq(i))));
    //     }

    //     while single_queue.len() + merge_queue.len() >= 2 {
    //         let left : TreeNode = *(remove_smallest(&single_queue, &merge_queue).unwrap());
    //         let right : TreeNode = *(remove_smallest(&single_queue, &merge_queue).unwrap());
    //         let cur : TreeNode = TreeNode::init_with_freq(left.freq_ + right.freq_);
    //         merge_queue.push(Some(Box::new(cur)));
    //     }

    //     if single_queue.len() == 1 {
    //         self.root_ = single_queue[0];
    //     }
    //     else  {
    //         self.root_ = merge_queue[0];
    //     }
    // }

    pub fn get_bits_for_char(&self, c : char) -> Vec<bool> {
        todo!()
    }

    // // // void decode(std::stringstream& ss, BinaryFileReader& bfile);
    // pub fn decode(&self, leave_blank , bfile : BinaryFileReader) {
    //     todo!()
    // }

    // pub fn write_tree(&self, current : Option<Box<TreeNode>>, bfile : BinaryFileWriter) {
    //     todo!()
    // }

    pub fn height(&self, sub_root : Option<Box<TreeNode>>) -> usize {
        todo!();
    }
    
}