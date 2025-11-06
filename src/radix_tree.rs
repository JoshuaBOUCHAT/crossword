use std::ops::{Index, IndexMut};

use crate::explorer::ExploreState;

#[derive(Debug)]
pub struct TreeNode {
    is_valide_word: bool,
    branches: [u32; 26],
}
impl Default for TreeNode {
    fn default() -> Self {
        Self {
            is_valide_word: false,
            branches: [u32::MAX; 26],
        }
    }
}

#[derive(Debug)]
pub struct RadixTree {
    nodes: Vec<TreeNode>,
}

impl Index<usize> for RadixTree {
    type Output = TreeNode;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}
impl IndexMut<usize> for RadixTree {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
#[inline]
fn char_to_index(c: char) -> usize {
    (c as u8 - b'a') as usize
}
pub fn validate_char(c: char) -> Result<char, String> {
    match c {
        c_lower_case @ 'a'..='z' => Ok(c_lower_case),
        c_upper_case @ 'A'..='Z' => Ok(c_upper_case.to_ascii_lowercase()),
        c => {
            return Err(format!(
                "The char you gave contain invalide character: {}",
                c
            ));
        }
    }
}

impl RadixTree {
    fn new() -> Self {
        Self {
            nodes: vec![TreeNode::default()],
        }
    }
    pub fn try_from_iter<S: AsRef<str>>(
        word_iter: impl Iterator<Item = S>,
    ) -> Result<Self, String> {
        let mut tree = Self::new();
        for word in word_iter {
            tree.add_word(word.as_ref())?;
        }
        Ok(tree)
    }
    fn len(&self) -> usize {
        self.nodes.len()
    }
    fn insert_node(&mut self, node: TreeNode) -> usize {
        let new_tree_node_index = self.len();
        assert!(new_tree_node_index != u32::MAX as usize);
        self.nodes.push(node);
        new_tree_node_index
    }
    pub fn add_word(&mut self, word: &str) -> Result<(), String> {
        let mut actual = 0;
        for char in word.chars() {
            let valide_char = validate_char(char)?;
            let char_idx = char_to_index(valide_char);
            actual = if self[actual].branches[char_idx] == u32::MAX {
                let new_inserted_index = self.insert_node(TreeNode::default());
                self[actual].branches[char_idx] = new_inserted_index as u32;
                new_inserted_index
            } else {
                self[actual].branches[char_idx] as usize
            }
        }
        self[actual].is_valide_word = true;
        Ok(())
    }
    pub fn explore_char_unchecked(&self, index: usize, explore_char: char) -> Option<ExploreState> {
        let explore_char_index = char_to_index(explore_char);
        if self[index].branches[explore_char_index] == u32::MAX {
            return None;
        }
        let index = self[index].branches[explore_char_index];
        Some(ExploreState::new(
            self[index as usize].is_valide_word,
            index,
        ))
    }
}
