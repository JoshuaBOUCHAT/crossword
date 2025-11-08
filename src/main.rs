use std::{collections::HashSet, time::Instant};

use crate::{crossword_matrix::CrosswordMatrix, radix_tree::RadixTree};
use rand::Rng;
mod crossword_matrix;
mod explorer;
pub mod radix_tree;

const LIST: &str = include_str!("../list.txt");
const N: usize = 31623;

fn main() {
    let tree = RadixTree::try_from_iter(LIST.split('\n')).expect("can't build the tree");
    let grid = random_lowwercase(N * N);
    let matrix = CrosswordMatrix::from_linear(N, N, grid).unwrap();
    let now = Instant::now();
    let result = matrix.solve(&tree);
    let needed_time = now.elapsed().as_secs_f32();
    /*for word in &result {
        print!("{word} ");
    }*/
    println!(
        "\n nombre total de mots: {} trouvé en seulement {}s",
        result.len(),
        needed_time
    )
}

fn random_lowwercase(len: usize) -> String {
    let mut rng = rand::rng();
    (0..len)
        .map(|_| (b'a' + rng.random_range(0..26)) as char)
        .collect()
}

const XMAS: &str = include_str!("../input.txt");

fn xmas_solving() {
    let tree = RadixTree::try_from_iter(["xmas"].iter()).expect("can't build the tree");
    let rows: Vec<&str> = XMAS.lines().collect();
    let matrix = CrosswordMatrix::from_row(rows.as_slice()).expect("Matrix initialisation failed");
    let result = matrix.solve(&tree);
    println!("Le nombre de mot trouvé: {}", result.len());
}
