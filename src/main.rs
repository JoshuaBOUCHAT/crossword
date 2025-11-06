use crate::{
    explorer::{ExplorerResult, WordExplorer},
    radix_tree::RadixTree,
};

mod explorer;
pub mod radix_tree;

const LIST: &str = include_str!("../list.txt");

fn main() {
    let tree = RadixTree::try_from_iter(LIST.split('\n')).expect("can't build the tree");
    let mut word_explorer = WordExplorer::new(tree);
    let tested_word = "lapins";
    for c in tested_word.chars() {
        if let ExplorerResult::ValideWord = word_explorer.explore_char(c) {
            println!("{}", word_explorer.get_word())
        }
    }
}
