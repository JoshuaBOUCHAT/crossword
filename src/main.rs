use crate::radix_tree::RadixTree;

pub mod radix_tree;

fn main() {
    let words = [
        "ma", "mai", "mais", "son", "ons", "maison", "maisons", "ai", "mal",
    ];
    let tree = RadixTree::try_from_iter(words.iter()).expect("tree buidling failed:");
    dbg!(tree);
}
