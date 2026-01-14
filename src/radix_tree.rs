use std::ops::{Index, IndexMut};

use crate::explorer::ExploreState;

const NO_NODE: u32 = u32::MAX;
#[derive(Debug)]
pub struct TreeNode {
    is_valid_word: bool,
    branches: [u32; 26],
}
impl Default for TreeNode {
    fn default() -> Self {
        Self {
            is_valid_word: false,
            branches: [NO_NODE; 26],
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct RadixTree {
    nodes: Vec<TreeNode>,
}

impl Index<usize> for RadixTree {
    type Output = TreeNode;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.nodes.get_unchecked(index) }
    }
}
impl IndexMut<usize> for RadixTree {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.nodes.get_unchecked_mut(index) }
    }
}
/// Newtype garantissant un index valide (0-25) pour les branches du radix tree.
/// La validation se fait à la construction, éliminant le besoin de vérifications ultérieures.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct CharIndex(u8);

impl CharIndex {
    /// Crée un CharIndex à partir d'un caractère alphabétique.
    /// Retourne None si le caractère n'est pas une lettre ASCII.
    #[inline]
    pub fn new(c: char) -> Option<Self> {
        match c {
            'a'..='z' => Some(Self(c as u8 - b'a')),
            'A'..='Z' => Some(Self(c as u8 - b'A')),
            _ => None,
        }
    }

    /// Retourne l'index (0-25) pour accéder aux branches.
    #[inline]
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }

    /// Retourne le caractère minuscule correspondant.
    #[inline]
    pub fn as_char(self) -> char {
        (self.0 + b'a') as char
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
        assert!(new_tree_node_index != NO_NODE as usize);
        self.nodes.push(node);
        new_tree_node_index
    }
    pub fn add_word(&mut self, word: &str) -> Result<(), String> {
        let mut actual = 0;
        for c in word.chars() {
            let char_idx = CharIndex::new(c)
                .ok_or_else(|| format!("The char you gave contains invalid character: {}", c))?;
            let idx = char_idx.as_usize();
            actual = if self[actual].branches[idx] == NO_NODE {
                let new_inserted_index = self.insert_node(TreeNode::default());
                self[actual].branches[idx] = new_inserted_index as u32;
                new_inserted_index
            } else {
                self[actual].branches[idx] as usize
            }
        }
        self[actual].is_valid_word = true;
        Ok(())
    }

    /// Explore une branche du radix tree avec un CharIndex garanti valide.
    /// Retourne None si la branche n'existe pas.
    ///
    #[inline(always)]
    pub fn explore(&self, node_index: usize, char_idx: CharIndex) -> Option<ExploreState> {
        let branch_idx = char_idx.as_usize();
        let next_node = self[node_index].branches[branch_idx];
        if next_node == NO_NODE {
            return None;
        }
        Some(ExploreState::new(
            self[next_node as usize].is_valid_word,
            next_node,
        ))
    }
}
