use crate::radix_tree::{RadixTree, validate_char};
#[derive(Default, Debug, Clone, Copy)]
pub struct ExploreState {
    is_valide_word: bool,
    index: u32,
}
impl ExploreState {
    pub fn new(is_valide_word: bool, index: u32) -> Self {
        Self {
            is_valide_word,
            index,
        }
    }
}
pub struct WordExplorer {
    inner_word: String,
    tree: RadixTree,
    word_state: ExploreState,
}
pub enum ExplorerResult {
    ValideWord,
    PartialWord,
    Reset,
}

impl WordExplorer {
    pub fn new(radix_tree: RadixTree) -> Self {
        Self {
            inner_word: String::new(),
            tree: radix_tree,
            word_state: ExploreState::default(),
        }
    }
    pub fn explore_char(&mut self, c: char) -> ExplorerResult {
        let valide_char = validate_char(c).expect(&format!(
            "Char {c} passed by parameter is inivalide only letter are accepted"
        ));
        let maybe_new_state = self
            .tree
            .explore_char_unchecked(self.word_state.index as usize, valide_char);

        if let Some(new_state) = maybe_new_state {
            self.word_state = new_state;
            self.inner_word.push(valide_char);
        } else {
            self.flush();
            return ExplorerResult::Reset;
        }

        if self.word_state.is_valide_word {
            ExplorerResult::ValideWord
        } else {
            ExplorerResult::PartialWord
        }
    }
    pub fn get_word<'a>(&'a self) -> &'a str {
        &self.inner_word
    }
    pub fn flush(&mut self) {
        self.word_state = ExploreState::default();
        self.inner_word.clear();
    }
}
