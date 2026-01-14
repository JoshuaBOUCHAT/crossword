use crate::radix_tree::{CharIndex, RadixTree};
#[derive(Default, Debug, Clone, Copy)]
pub struct ExploreState {
    is_valid_word: bool,
    index: u32,
}
impl ExploreState {
    pub fn new(is_valid_word: bool, index: u32) -> Self {
        Self {
            is_valid_word,
            index,
        }
    }
}
pub struct WordExplorer<'a> {
    inner_word: String,
    tree: &'a RadixTree,
    word_state: ExploreState,
}
pub enum ExplorerResult {
    ValidWord,
    PartialWord,
    Reset,
}

impl<'a> WordExplorer<'a> {
    pub fn new(radix_tree: &'a RadixTree) -> Self {
        Self {
            inner_word: String::new(),
            tree: radix_tree,
            word_state: ExploreState::default(),
        }
    }
    pub fn explore_char(&mut self, c: char) -> ExplorerResult {
        let Some(char_idx) = CharIndex::new(c) else {
            self.flush();
            return ExplorerResult::Reset;
        };

        let Some(new_state) = self.tree.explore(self.word_state.index as usize, char_idx) else {
            self.flush();
            return ExplorerResult::Reset;
        };

        self.word_state = new_state;
        self.inner_word.push(char_idx.as_char());

        if self.word_state.is_valid_word {
            ExplorerResult::ValidWord
        } else {
            ExplorerResult::PartialWord
        }
    }
    pub fn get_word<'b>(&'b self) -> &'b str {
        &self.inner_word
    }
    pub fn flush(&mut self) {
        self.word_state = ExploreState::default();
        self.inner_word.clear();
    }
}
