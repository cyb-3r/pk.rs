/*!
Types and systems to check puzzle constraints

A clue is an int `n` which indicate that there is `n` consecutive tiles
that are part of the solution.
*/

use crate::binmat::BinMat;

/**
Clue vector type alias

A `ClueSet` is an array of clues related to one row or column of the puzzle.
Clues next to each other indicates that there is a certain number of tiles between
the two groups that are not part of the solution.

Example:
```
use pkrs::clues::ClueSet;

let set: ClueSet = vec![1, 3, 2];

// here, we can see that there is 1 valid tile
// and a group of 3 valid tiles that are separated
// by at least 1 non valid tile.
```
*/
pub type ClueSet = Vec<u8>;

// TODO: simplify this function
fn clueset_new<'a>(data: impl Iterator<Item = &'a bool>) -> ClueSet {
    let mut clues: ClueSet = Vec::new();
    let mut count: u8 = 0;

    for val in data {
        match val {
            true => count += 1,
            false => {
                if count > 0 {
                    clues.push(count);
                    count = 0;
                }
            }
        }
    }

    if count > 0 {
        clues.push(count);
    }

    if clues.len() == 0 {
        vec![0]
    } else {
        clues
    }
}

/**
Manages set of clues related to a puzzle
*/
#[derive(Debug, Clone)]
pub struct PuzzleClues {
    /// Clue set for each row
    row_clues: Vec<ClueSet>,
    /// Clue set for each columns
    col_clues: Vec<ClueSet>,
}

impl PuzzleClues {
    /// Returns clues from from a binary matrix
    pub fn from_mat(mat: &BinMat) -> Self {
        let (w, h) = (mat.get_width(), mat.get_height());

        let cs: PuzzleClues = PuzzleClues {
            row_clues: (0..w)
                .map(move |row| clueset_new(mat.row_iter(row).unwrap()))
                .collect(),

            col_clues: (0..h)
                .map(move |col| clueset_new(mat.col_iter(col).unwrap()))
                .collect(),
        };

        cs
    }

    /// Returns a reference to the clues of a row
    pub fn get_row(&self, id: usize) -> Option<&Vec<u8>> {
        self.row_clues.get(id)
    }

    /// Returns a reference to the clues of a col
    pub fn get_col(&self, id: usize) -> Option<&Vec<u8>> {
        self.col_clues.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        const SIZE: u8 = 5;
        let clues = PuzzleClues::from_mat(&BinMat::new(SIZE, SIZE));

        assert_eq!(clues.row_clues.len(), SIZE as usize);
        assert_eq!(clues.col_clues.len(), SIZE as usize);
    }

    #[test]
    fn test_clues_accuracy() {
        let clues = PuzzleClues::from_mat(&{
            let mut mat: BinMat = BinMat::new(5, 5);

            // preparing mat
            let _ = mat.toggle_val(0, 0);
            let _ = mat.toggle_val(1, 0);
            let _ = mat.toggle_val(4, 0);
            mat
        });

        // row
        assert_eq!(clues.get_row(0), Some(&vec![2u8, 1u8]));
        // should fail
        assert_eq!(clues.get_row(10), None);

        // col
        assert_eq!(clues.get_col(0), Some(&vec![1u8]));
        // should fail
        assert_eq!(clues.get_col(11), None);
    }
}
