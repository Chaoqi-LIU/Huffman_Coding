use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeNode {
  pub freq_: i32,
  pub char_: String,
  pub left_: Option<usize>,
  pub right_: Option<usize>,
}