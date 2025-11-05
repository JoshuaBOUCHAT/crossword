use std::ops::{Index, IndexMut};

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
            let valide_char = match char {
                c_lower_case @ 'a'..='z' => c_lower_case,
                c_upper_case @ 'A'..='Z' => c_upper_case.to_ascii_lowercase(),
                c => {
                    return Err(format!(
                        "The word you tried to insert:({}) contain invalide character: {}",
                        word, c
                    ));
                }
            };
            let char_idx = (valide_char as u8 - b'a') as usize;
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
}

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
