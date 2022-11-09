use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

use std::fs;
use std::fs::File;

use crate::huffmantree::frequency::Frequency;
use crate::huffmantree::treenode::TreeNode;

use super::frequency;

#[derive(Serialize, Deserialize, Debug)]
pub struct HuffmanTree {
    pub nodes : Vec<TreeNode>,
    idx : usize,
    root_ : usize,
    bits_map_ : HashMap<char, Vec<bool>>,
}

impl HuffmanTree {
    pub fn new() -> HuffmanTree {
        HuffmanTree {
            nodes : vec![TreeNode { freq_ : Frequency::new(0), left_ : 0, right_ : 0 }],
            idx : 0,
            root_ : 0,
            bits_map_ : HashMap::new(),
        }
    }

    pub fn insert(&mut self, frequency: i32, letter: char) -> usize {
        self.nodes.push(TreeNode {
            freq_ : Frequency::init_with_char_and_int(letter, frequency),
            left_ : 0,
            right_ : 0
        });
        self.idx += 1;
        return self.idx;
    }

    pub fn remove_smallest(&self, single_queue : &mut Vec<usize>, merged_queue : &mut Vec<usize>) -> usize {
        let mut to_return : usize = 0;
        if single_queue.len() == 0 {
            to_return = merged_queue[0]; merged_queue.remove(0);
        } else if merged_queue.len() == 0 {
            to_return = single_queue[0]; single_queue.remove(0);
        } else if self.nodes[single_queue[0]].freq_ < self.nodes[merged_queue[0]].freq_ {
            to_return = single_queue[0]; single_queue.remove(0);
        } else {
            to_return = merged_queue[0]; single_queue.remove(0);
        }
        return to_return;
    }
    
    pub fn build_tree(&mut self, frequancies :&Vec<Frequency>) {
        let mut single_queue : Vec<usize> = vec![];
        let mut merged_queue : Vec<usize> = vec![];

        for i in frequancies {
            single_queue.push(self.insert(i.get_frequancy(), i.get_charactor()));
        }

        while single_queue.len() + merged_queue.len() >= 2 {
            let left = self.remove_smallest(&mut single_queue, &mut merged_queue);
            let right = self.remove_smallest(&mut single_queue, &mut merged_queue);
            let cur = self.insert(self.nodes[left].freq_.get_frequancy() + self.nodes[right].freq_.get_frequancy(), '\0');
            self.nodes[cur].left_ = left;
            self.nodes[cur].right_ = right;
            merged_queue.push(cur);
        }
        
        if single_queue.len() == 1 {
            self.root_ = single_queue[0];
        }
        else {
            self.root_ = merged_queue[0];
        }
    }

    pub fn build_map(&mut self, current : usize, path : &mut Vec<bool>) {
        if self.nodes[current].left_ == 0 && self.nodes[current].right_ == 0 {
            self.bits_map_.insert(self.nodes[current].freq_.get_charactor(), path.clone());
            return;
        }

        path.push(false);
        self.build_map(self.nodes[current].left_, path);
        path.pop();

        path.push(true);
        self.build_map(self.nodes[current].right_, path);
        path.pop();
    }

    pub fn build_tree_from_text(&mut self, file : &str) {
        let text = fs::read_to_string(file).unwrap();
        let mut map : HashMap<char, i32> = HashMap::new();
        for c in text.chars() {
            if map.contains_key(&c) {
                map.insert(c, map[&c] + 1);
            } else {
                map.insert(c, 1);
            }
        }
        
        let mut frequencies = Vec::new();
        for pair in map {
            frequencies.push( Frequency::init_with_char_and_int(pair.0, pair.1) );
        }

        frequencies.sort_by(|a, b| a.frequency_.partial_cmp(&b.frequency_).unwrap());

        self.build_tree(&frequencies);
        self.build_map(self.root_, &mut Vec::new());
    }

}
