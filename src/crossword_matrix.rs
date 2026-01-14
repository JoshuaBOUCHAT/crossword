use core::fmt;
use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{explorer::WordExplorer, radix_tree::RadixTree};

#[derive(Debug)]
pub struct CrosswordMatrix {
    inner: Vec<char>,
    h_len: usize,
    v_len: usize,
}
#[derive(Debug, Clone)]
pub enum CrosswordMatrixError {
    InvalidChar,
    InvalidSize,
}

impl fmt::Display for CrosswordMatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrosswordMatrixError::InvalidChar => {
                write!(f, "Matrix contains non-alphabetic characters")
            }
            CrosswordMatrixError::InvalidSize => {
                write!(f, "Matrix dimensions do not match string length")
            }
        }
    }
}

impl std::error::Error for CrosswordMatrixError {}

impl CrosswordMatrix {
    fn validate_letters(s: &str) -> Result<(), CrosswordMatrixError> {
        if s.chars().all(|c| c.is_ascii_alphabetic()) {
            Ok(())
        } else {
            Err(CrosswordMatrixError::InvalidChar)
        }
    }

    pub fn from_linear(
        h_len: usize,
        v_len: usize,
        as_letters: impl AsRef<str>,
    ) -> Result<Self, CrosswordMatrixError> {
        let letters = as_letters.as_ref();

        let total_len = h_len * v_len;
        if total_len != letters.len() {
            return Err(CrosswordMatrixError::InvalidSize);
        }
        Self::validate_letters(letters)?;
        Ok(Self {
            h_len,
            v_len,

            inner: letters.chars().collect(),
        })
    }
    pub fn from_row(letterss: &[impl AsRef<str>]) -> Result<Self, CrosswordMatrixError> {
        let h_len = letterss[0].as_ref().len();
        let v_len = letterss.len();
        let total_len = v_len * h_len;
        let mut inner: Vec<char> = Vec::with_capacity(total_len);
        for letters in letterss.into_iter().map(|s| s.as_ref()) {
            if letters.len() != h_len {
                return Err(CrosswordMatrixError::InvalidSize);
            }
            Self::validate_letters(letters)?;
            inner.extend(letters.chars());
        }
        Ok(Self {
            h_len,
            v_len,

            inner,
        })
    }
}
impl Index<usize> for CrosswordMatrix {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}
impl IndexMut<usize> for CrosswordMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}
impl CrosswordMatrix {
    #[inline]
    fn handle_parcours(
        &self,
        explorer: &mut WordExplorer,
        result: &mut HashSet<String>,
        start: usize,
        stop: usize,
        step: usize,
        rev: bool,
    ) {
        use crate::explorer::ExplorerResult;
        explorer.flush();

        if rev {
            for idx in (start..stop).step_by(step).rev() {
                match explorer.explore_char(self[idx]) {
                    ExplorerResult::Reset => break,
                    ExplorerResult::ValidWord => {
                        result.insert(explorer.get_word().to_string());
                    }
                    ExplorerResult::PartialWord => {}
                }
            }
        } else {
            for idx in (start..stop).step_by(step) {
                match explorer.explore_char(self[idx]) {
                    ExplorerResult::Reset => break,
                    ExplorerResult::ValidWord => {
                        result.insert(explorer.get_word().to_string());
                    }
                    ExplorerResult::PartialWord => {}
                }
            }
        }
    }
    #[inline(always)]
    pub fn solve_row(
        &self,
        col: usize,
        row: usize,
        exp_ref: &mut WordExplorer,
        result: &mut HashSet<String>,
    ) {
        let h_len = self.h_len;
        let v_len = self.v_len;

        let index = row * h_len + col;

        //down
        self.handle_parcours(exp_ref, result, index, h_len * v_len, h_len, false);
        //right
        let rg_stop = (row + 1) * h_len;
        self.handle_parcours(exp_ref, result, index, rg_stop, 1, false);

        //left
        self.handle_parcours(exp_ref, result, row * h_len, index + 1, 1, true);

        //up
        self.handle_parcours(exp_ref, result, col, index + 1, h_len, true);

        //staire down left right
        let max_iter = (h_len - col - 1).min(v_len - row - 1);
        let stop = index + max_iter * (h_len + 1) + 1;

        self.handle_parcours(exp_ref, result, index, stop, h_len + 1, false);

        //staire up right left
        let max_iter = col.min(row);
        let start = index - (h_len + 1) * max_iter;
        self.handle_parcours(exp_ref, result, start, index + 1, h_len + 1, true);

        //stair down right left
        let max_iter = (v_len - 1 - row).min(col);
        let stop = index + max_iter * (h_len - 1) + 1;
        self.handle_parcours(exp_ref, result, index, stop, h_len - 1, false);

        //staire up left right
        let max_iter = (h_len - col).min(row);
        let start = index - (h_len - 1) * max_iter;
        self.handle_parcours(exp_ref, result, start, index + 1, h_len - 1, true);
    }

    pub fn solve(&self, tree: &RadixTree) -> HashSet<String> {
        let h_len = self.h_len;
        let v_len = self.v_len;

        // On divise les colonnes en chunks pour paralléliser proprement
        let n_threads = std::thread::available_parallelism()
            .map(|n| n.get() * 2)
            .unwrap_or(1);

        let col_chunks = chunked_range_by_count(0..h_len, n_threads).collect::<Vec<_>>();

        col_chunks
            .into_par_iter()
            .flat_map(|col_range| {
                let mut local_results = HashSet::new();
                let mut explorer = WordExplorer::new(tree);

                for col in col_range {
                    for row in 0..v_len {
                        self.solve_row(col, row, &mut explorer, &mut local_results);
                    }
                }

                local_results
            })
            .collect()
    }
}

fn chunked_range_by_count(
    range: std::ops::Range<usize>,
    n_chunks: usize,
) -> impl Iterator<Item = std::ops::Range<usize>> {
    let len = range.end - range.start;
    let base_size = len / n_chunks;
    let remainder = len % n_chunks;

    (0..n_chunks).map(move |i| {
        let start = range.start + i * base_size + remainder.min(i);
        let end = start + base_size + if i < remainder { 1 } else { 0 };
        start..end
    })
}
mod test {

    use crate::{crossword_matrix::CrosswordMatrix, radix_tree::RadixTree};

    #[test]
    fn test_soling() {
        const UP: &str = "SoooAoooMoooXooo";
        const DOWN: &str = "XoooMoooAoooSooo";
        const STAIRS_DOWN_LEFT_RIGHT: &str = "XooooMooooAooooS";
        const STAIRS_UP_RIGHT_LEFT: &str = "SooooAooooMooooX";
        const RIGHT: &str = "XMASoooooooooooo";
        const LEFT: &str = "SAMXoooooooooooo";
        const STAIRS_DOWN_RIGHT_LEFT: &str = "oooXooMooAooSooo";
        const STAIRS_UP_LEFT_RIGHT: &str = "oooSooAooMooXooo";
        let tree = RadixTree::try_from_iter(["xmas"].iter()).expect("can't build the tree");
        let tests = [
            UP,
            DOWN,
            STAIRS_DOWN_LEFT_RIGHT,
            STAIRS_UP_RIGHT_LEFT,
            RIGHT,
            LEFT,
            STAIRS_DOWN_RIGHT_LEFT,
            STAIRS_UP_LEFT_RIGHT,
        ];
        for game in tests {
            let matrix = CrosswordMatrix::from_linear(4, 4, game).unwrap();
            assert_eq!(matrix.solve(&tree).len(), 1);
        }
    }

    #[test]
    fn test_solving_reel() {
        const TEST: &str = "ooXooooSAMXooAooAoXMASoSoXoooo";
        let tree = RadixTree::try_from_iter(["xmas"].iter()).expect("can't build the tree");
        let matrix = CrosswordMatrix::from_linear(6, 5, TEST).unwrap();
        assert_eq!(matrix.solve(&tree).len(), 4);
    }
    #[test]
    fn solve_site_problem() {
        const TEST: &str = "MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX";
        let tree = RadixTree::try_from_iter(["xmas"].iter()).expect("can't build the tree");
        let matrix = CrosswordMatrix::from_linear(10, 10, TEST).unwrap();
        assert_eq!(matrix.solve(&tree).len(), 18);
    }
}
