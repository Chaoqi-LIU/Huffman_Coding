use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct HuffmanTree {
    pub nodes : Vec<TreeNode>,
    idx : usize,
}

impl HuffmanTree {
    pub fn new() -> HuffmanTree {
        HuffmanTree {
            nodes : vec![TreeNode { freq_ : 0, char_ : '\0', left_ : None, right_ : None }],
            idx : 0,
        }
    }

    pub fn insert(&self, frequency: u32, letter: String) -> usize {
        self.nodes.push(TreeNode {
            freq_ : frequency,
            char_ : letter,
            left_ : 0,
            right_ : 0
        });
        idx += 1;
        return idx;
    }

    pub fn remove_smallest(single_queue : Vec<TreeNode>, merged_queue : Vec<TreeNode>) {
        
    }

    pub fn build_tree(&self, frequancies : Vec<i32>) {
        
    }
}
