

use super::frequency::Frequency;

#[derive(Debug)]
pub struct TreeNode {
  pub freq_: Frequency,
  pub left_: usize,
  pub right_: usize,
}