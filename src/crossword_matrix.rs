use core::fmt;
use std::ops::{Index, IndexMut};

use crate::explorer::WordExplorer;

#[derive(Debug)]
pub struct CrosswordMatrix {
    inner: Vec<char>,
    h_len: usize,
    v_len: usize,
    total_len: usize,
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
            total_len,
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
            total_len,
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
        result: &mut Vec<String>,
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
                    ExplorerResult::ValideWord => result.push(explorer.get_word().to_string()),
                    ExplorerResult::PartialWord => {}
                }
            }
        } else {
            for idx in (start..stop).step_by(step) {
                match explorer.explore_char(self[idx]) {
                    ExplorerResult::Reset => break,
                    ExplorerResult::ValideWord => result.push(explorer.get_word().to_string()),
                    ExplorerResult::PartialWord => {}
                }
            }
        }
    }

    pub fn solve(&self, explorer: &mut WordExplorer) -> Vec<String> {
        let mut result = vec![];
        let total_len = self.total_len;
        let h_len = self.h_len;

        for col in 0..self.h_len {
            for row in 0..self.v_len {
                let index = row * self.h_len + col;

                //down
                self.handle_parcours(explorer, &mut result, index, total_len, h_len, false);
                //right
                let rg_stop = (row + 1) * self.h_len;
                self.handle_parcours(explorer, &mut result, index, rg_stop, 1, false);

                //left
                self.handle_parcours(explorer, &mut result, row * h_len, index + 1, 1, true);

                //up
                self.handle_parcours(explorer, &mut result, col, index + 1, h_len, true);

                //staire down left right
                let max_iter = (h_len - col - 1).min(self.v_len - row - 1);
                let stop = index + max_iter * (self.h_len + 1) + 1;

                self.handle_parcours(explorer, &mut result, index, stop, h_len + 1, false);

                //staire up right left
                let max_iter = col.min(row);
                let start = index - (self.h_len + 1) * max_iter;
                self.handle_parcours(explorer, &mut result, start, index + 1, h_len + 1, true);

                //stair down right left
                let max_iter = (self.v_len - 1 - row).min(col);
                let stop = index + max_iter * (h_len - 1) + 1;
                self.handle_parcours(explorer, &mut result, index, stop, h_len - 1, false);

                //staire up left right
                let max_iter = (h_len - col).min(row);
                let start = index - (h_len - 1) * max_iter;
                self.handle_parcours(explorer, &mut result, start, index + 1, h_len - 1, true);
            }
        }

        result
    }
}

mod test {
    use crate::{crossword_matrix::CrosswordMatrix, explorer::WordExplorer, radix_tree::RadixTree};

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
        let mut explorer = WordExplorer::new(tree);
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
            assert_eq!(matrix.solve(&mut explorer).len(), 1);
        }
    }

    #[test]
    fn test_solving_reel() {
        const TEST: &str = "ooXooooSAMXooAooAoXMASoSoXoooo";
        let tree = RadixTree::try_from_iter(["xmas"].iter()).expect("can't build the tree");
        let mut explorer = WordExplorer::new(tree);
        let matrix = CrosswordMatrix::from_linear(6, 5, TEST).unwrap();
        assert_eq!(matrix.solve(&mut explorer).len(), 4);
    }
    #[test]
    fn solve_site_problem() {
        const TEST: &str = "MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX";
        let tree = RadixTree::try_from_iter(["xmas"].iter()).expect("can't build the tree");
        let mut explorer = WordExplorer::new(tree);
        let matrix = CrosswordMatrix::from_linear(10, 10, TEST).unwrap();
        assert_eq!(matrix.solve(&mut explorer).len(), 18);
    }
}
