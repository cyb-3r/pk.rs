//! Binary matrix type

use crate::binmat::error::BinMatError;
use crate::utils::*;

/**
A matrix of bool values

Carries `w` * `h` bool values, `w` being the amount of columns and `h` the amount of rows.

Its purpose is to initialize puzzles and their clues. That means this structure typically
contains the solution pattern. That allows you to read the solution safely without
causing side effects on the puzzle board or the clues.

*This implementation can change in the future to use bits instead of bools to save memory.*
*/
pub struct BinMat {
    /// The number of columns of a matrix
    width: u8,
    /// The number of rows of a matrix
    height: u8,
    /// The values of a matrix
    values: Vec<bool>,
}

impl BinMat {
    /// Returns the matrix's width
    pub fn get_width(&self) -> u8 {
        self.width
    }

    /// Returns the matrix's height
    pub fn get_height(&self) -> u8 {
        self.height
    }

    /// Returns a clone of the matrix's values
    pub fn get_mat(&self) -> Vec<bool> {
        self.values.clone()
    }
}

impl BinMat {
    /// Creates a new binary matrix
    pub fn new(w: u8, h: u8) -> Self {
        BinMat {
            width: w,
            height: h,
            values: vec![false; (w * h) as usize],
        }
    }

    /// Inverse a value located at the x and y coordinates of the matrix
    /// May return an error if the given coordinates are out of range.
    pub fn toggle_val(&mut self, x: u8, y: u8) -> Result<(), BinMatError> {
        if !in_range(&x, 0, self.width) || !in_range(&y, 0, self.height) {
            return Err(BinMatError::OutOfBounds);
        }

        self.values[(x + y * self.width) as usize] ^= true;

        Ok(())
    }

    /// Returns a value located at the x and y coordinates of the matrix
    /// May return an error if the given coordinates are out of range.
    pub fn get_val(&self, x: u8, y: u8) -> Result<bool, BinMatError> {
        if !in_range(&x, 0, self.width) || !in_range(&y, 0, self.height) {
            return Err(BinMatError::OutOfBounds);
        }

        Ok(self.values[(x + y * self.width) as usize])
    }

    /// Extracts a reference of one of the matrix's rows
    /// May return an error if the given row is out of range.
    pub fn extract_row(&self, row: u8) -> Result<&[bool], BinMatError> {
        if in_range(&row, 0, self.height) {
            let id = (row * self.width) as usize;
            Ok(&self.values[id..id + self.width as usize])
        } else {
            Err(BinMatError::OutOfBounds)
        }
    }
}

mod error {
    #[derive(Debug, PartialEq, Eq)]
    pub enum BinMatError {
        OutOfBounds,
    }

    impl std::fmt::Display for BinMatError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            match self {
                BinMatError::OutOfBounds => {
                    write!(f, "Coordinates are out of bounds")
                }
            }
        }
    }

    impl std::error::Error for BinMatError {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        const SIZE: u8 = 3;
        let mat = BinMat::new(SIZE, SIZE);

        // is the vector properly initialized?
        assert_eq!(mat.values, vec![false; (SIZE * SIZE) as usize]);
    }

    #[test]
    fn test_get() {
        const SIZE: u8 = 3;
        let mat = BinMat::new(SIZE, SIZE);

        // coordinates are out of range, this should fail
        assert!(mat.get_val(2, 3).is_err());

        // however, this one should succeed
        assert!(mat.get_val(2, 2).is_ok());
        assert_eq!(mat.get_val(2, 2).unwrap(), false);
    }

    #[test]
    fn test_set() {
        const SIZE: u8 = 3;
        let mut mat = BinMat::new(SIZE, SIZE);

        // coordinates are out of range, this should fail
        assert!(mat.toggle_val(2, 3).is_err());

        // however, this one should succeed
        assert!(mat.toggle_val(2, 2).is_ok());

        // has the value been correctly updated?
        assert_eq!(mat.get_val(2, 2).unwrap(), true);
    }

    #[test]
    fn test_extract_row() {
        const SIZE: u8 = 3;
        let mat = BinMat::new(SIZE, SIZE);

        // is the extracted ref valid?
        let extract = mat.extract_row(2);
        assert!(extract.is_ok());
        assert_eq!(extract.unwrap(), &[false; SIZE as usize]);

        // the next extract should fail
        let extract = mat.extract_row(3);
        assert!(extract.is_err());
    }
}
