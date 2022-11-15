use std::collections::HashMap;

use crate::huffmantree::frequency::Frequency;
use crate::huffmantree::treenode::TreeNode;
use crate::bfile::bfile::*;
use crate::mappers::*;
use crate::map::*;
use crate::reduce::*;

#[derive(Debug)]
pub struct HuffmanTree {
    nodes : Vec<TreeNode>,
    idx : usize,
    root_ : usize,
    bits_map_ : HashMap<String, Vec<bool>>,
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

    pub fn insert(&mut self, frequency: i32, letter: String) -> usize {
        self.nodes.push(TreeNode {
            freq_ : Frequency::init_with_char_and_int(letter, frequency),
            left_ : 0,
            right_ : 0
        });
        self.idx += 1;
        return self.idx;
    }

    pub fn remove_smallest(&self, single_queue : &mut Vec<usize>, merged_queue : &mut Vec<usize>) -> usize {
        let to_return : usize;
        if single_queue.len() == 0 {
            to_return = merged_queue[0]; merged_queue.remove(0);
        } else if merged_queue.len() == 0 {
            to_return = single_queue[0]; single_queue.remove(0);
        } else if self.nodes[single_queue[0]].freq_ < self.nodes[merged_queue[0]].freq_ {
            to_return = single_queue[0]; single_queue.remove(0);
        } else {
            to_return = merged_queue[0]; merged_queue.remove(0);
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
            let cur = self.insert(self.nodes[left].freq_.get_frequancy() + self.nodes[right].freq_.get_frequancy(), 
                                            String::from(self.nodes[left].freq_.get_charactor() + &self.nodes[right].freq_.get_charactor()));
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
        self.build_map(self.get_left(current), path);
        path.pop();

        path.push(true);
        self.build_map(self.get_right(current), path);
        path.pop();
    }

    pub fn build_tree_from_text(&mut self, file : &str) {
        let text = read_from_file(file);
        self.build_tree_from_string(text);
    }

    pub fn build_tree_from_string(&mut self, text : String) {
        if text == "".to_string() { panic!("empty text"); }
        
        let num_chunks = (text.len() / 100) + 1;
        let receiver = multi_threaded_mapper_generic::<char>(text, num_chunks, char_count_mapper.clone());
        let map = thread_reducer(receiver);
        
        let mut frequencies = Vec::new();
        for pair in map.iter() {
            frequencies.push( Frequency::init_with_char_and_int(pair.0.to_string(), pair.1.clone()) );
        }

        frequencies.sort_by(|a, b| a.frequency_.partial_cmp(&b.frequency_).unwrap());
        
        self.build_tree(&frequencies);
        self.build_map(self.root_, &mut Vec::new());
    }

    pub fn print_tree(&mut self) -> String {
        if self.root_ == 0 {
            return "(empty tree)".to_string();
        }
        let mut ret : String = "".to_string();

        let height = self.get_height();
        let print_matrix_width = (4 << height) - 3;
        let print_matrix_height = 2 * height + 1;

        let mut output : Vec<String> = vec!["".to_string(); print_matrix_height as usize];
        for i in &mut output {
            for _j in 0..(print_matrix_width + 4) {
                i.push(' ');
            }
        }

        self.print_subtree(self.root_, &mut output, 0, 0, print_matrix_width);

        for i in output {
            ret += &i;
            ret += &"\n".to_string();
        }
        return ret;
    }

    fn print_subtree(&mut self, croot : usize, output : &mut Vec<String>, left : i32, top : i32, curr_width : i32) {
        let mut node_str : String = "".to_string();
        if self.get_left(croot) == 0 && self.get_right(croot) == 0 {
            let mut s = self.get_node_at(croot).freq_.get_charactor();
            if s == "\n" {
                s = "\\n".to_string();
            } else if s == " " {
                s = "_".to_string();
            } else if s == "\t" {
                s = "\\t".to_string();
            }
            node_str = s + ":" + self.get_node_at(croot).freq_.get_frequancy().to_string().as_str();
        } else {
            node_str = self.get_node_at(croot).freq_.get_frequancy().to_string();
        }
        
        let left_start_shift : i32 = 1 - (node_str.len() as i32 - 1) / 2;
        let mut i : i32 = 0;
        while i < node_str.len() as i32 && left + curr_width / 2 + i < output[top as usize].len() as i32 {
            output[top as usize].replace_range((left + curr_width / 2 + left_start_shift + i) as usize..(left + curr_width / 2 + left_start_shift + i + 1) as usize,
                               node_str.chars().nth(i as usize).unwrap().to_string().as_str());
            i += 1;
        }

        // Calculate / \ offset = 2 ^ height
        let branchoffset = (curr_width + 3) >> 3; // (1 << (node -> printData - 1));

        // Print left child
        let center = left + curr_width / 2;
        let leftcenter = left + (curr_width / 2 - 1) / 2;
        let rightcenter = left + curr_width / 2 + 2 + (curr_width / 2 - 1) / 2;
        
        if self.get_left(croot) != 0 {
            let branch_pos = center - branchoffset + 1;
            // draw left upper branch
            let mut pos = center + left_start_shift - 2;
            while pos > branch_pos {
                output[top as usize].replace_range(pos as usize..(pos + 1) as usize, "_"); 
                pos -= 1;
            }
            output[(top + 1) as usize].replace_range(branch_pos as usize..(branch_pos + 1) as usize, "/");
            pos = branch_pos - 1;
            while pos > leftcenter + 2 {
                output[(top + 1) as usize].replace_range(pos as usize..(pos + 1) as usize, "_"); 
                pos -= 1;
            }
            self.print_subtree(self.get_left(croot), output, left, top + 2, curr_width / 2 - 1);
        }

        if self.get_right(croot) != 0 {
            let branch_pos = center + branchoffset + 1;
            //draw right upper branch
            let mut pos = center + left_start_shift + node_str.len() as i32 + 1;
            while pos < branch_pos {
                output[top as usize].replace_range(pos as usize..(pos + 1) as usize, "_"); 
                pos += 1;
            }
            output[(top + 1) as usize].replace_range(branch_pos as usize..(branch_pos + 1) as usize, "\\");
            pos = branch_pos + 1;
            while pos < rightcenter {
                output[(top + 1) as usize].replace_range(pos as usize..(pos + 1) as usize, "_"); 
                pos += 1;
            }
            self.print_subtree(self.get_right(croot), output, left + curr_width / 2 + 1, top + 2, curr_width / 2 - 1);
            
        }
    }

    pub fn get_left(&self, idx :usize) -> usize { return self.nodes[idx].left_; }

    pub fn get_right(&self, idx :usize) -> usize { return self.nodes[idx].right_; }

    pub fn get_bits_map(&self) -> &HashMap<String, Vec<bool>> { return &self.bits_map_; }

    pub fn get_nodes(&self) -> &Vec<TreeNode> { return &self.nodes; }

    pub fn get_root(&self) -> usize { return self.root_; }

    pub fn set_root(&mut self, idx : usize) { self.root_ = idx; }

    pub fn set_left(&mut self, cur : usize, left : usize) { self.nodes[cur].left_ = left; }

    pub fn set_right(&mut self, cur : usize, right : usize) { self.nodes[cur].right_ = right; }

    pub fn get_idx(&self) -> usize { return self.idx; }

    pub fn get_node_at(&self, idx : usize) -> &TreeNode { return &self.nodes[idx]; }

    pub fn get_height(&self) -> i32 { return self.get_height_helper(self.root_); }

    fn get_height_helper(&self, current : usize) -> i32 {
        if current == 0 { return -1; }
        return 1 + std::cmp::max(self.get_height_helper(self.get_left(current)), 
                                 self.get_height_helper(self.get_right(current)));
    }
}