use serde::{ Deserialize, Serialize };

use super::frequency::Frequency;

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeNode {
  pub freq_: Frequency,
  pub left_: usize,
  pub right_: usize,
}