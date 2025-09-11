//! Binary matrix type

use crate::binmat::error::BinMatError;
use crate::utils::in_range;
use std::fs;

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

// getters
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

// general implementation
impl BinMat {
    /// Creates a new binary matrix
    pub fn new(w: u8, h: u8) -> Self {
        BinMat {
            width: w,
            height: h,
            values: vec![false; (w * h) as usize],
        }
    }

    /// Returns a binary matrix with initialized values
    fn new_internal(w: u8, h: u8, values: Vec<bool>) -> Self {
        let mut values = values;
        let size = (w * h) as usize;

        match values.len().cmp(&size) {
            std::cmp::Ordering::Less => values.append(&mut vec![false; size - values.len()]),
            std::cmp::Ordering::Greater => values.truncate(size),
            std::cmp::Ordering::Equal => {}
        };

        BinMat {
            width: w,
            height: h,
            values: values,
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

    /// Extracts an iterator going over one of the matrix's rows
    /// May return an error if the given row is out of range.
    pub fn row_iter(&self, row: u8) -> Result<impl Iterator<Item = &bool>, BinMatError> {
        if in_range(&row, 0, self.height) {
            Ok((0..self.width).map(move |col| &self.values[(row * self.width + col) as usize]))
        } else {
            Err(BinMatError::OutOfBounds)
        }
    }

    /// Extracts an iterator going over one of the matrix's columns
    /// May return an error if the given col is out of range.
    pub fn col_iter(&self, col: u8) -> Result<impl Iterator<Item = &bool>, BinMatError> {
        if in_range(&col, 0, self.width) {
            Ok((0..self.height).map(move |row| &self.values[(row * self.width + col) as usize]))
        } else {
            Err(BinMatError::OutOfBounds)
        }
    }
}

// file interaction
impl BinMat {
    fn from_string(content: String) -> Result<Self, BinMatError> {
        let (dimensions, data) = content
            .split_once(':')
            .ok_or(BinMatError::InvalidFileFormat)?;

        let dims: Vec<u8> = dimensions
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();

        match dims.as_slice() {
            [w, h] => Ok(BinMat::new_internal(
                *w,
                *h,
                data.chars().map(|c| c == '1').collect::<Vec<bool>>(),
            )),
            _ => Err(BinMatError::InvalidFileFormat),
        }
    }
    /**
    Loads a binary matrix from a file

    The expected file format should be the following: `w,h:[0|1]`
    alternatively `[0-9]{,2},[0-9]{,2}:[01]+` if you get regex.

    For instance this is valid: `2,2:0110`
    */
    pub fn from_file(path_str: &str) -> Result<Self, BinMatError> {
        let content = fs::read_to_string(path_str).map_err(|_| BinMatError::FileNotFound)?;
        Self::from_string(content)
    }

    /// Formats a binary matrix to store it into a file
    fn format(&self) -> String {
        format!(
            "{},{}:{}",
            self.width,
            self.height,
            self.values
                .iter()
                .map(|&b| if b { '1' } else { '0' })
                .collect::<String>()
        )
    }

    /**
    Writes a binary matrix to a file, creates the file if it doesn't exist
    */
    pub fn to_file(&self, path: &str) -> Result<(), BinMatError> {
        std::fs::write(path, self.format()).map_err(|_| BinMatError::FileWriteError)
    }
}

impl std::fmt::Display for BinMat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.format())
    }
}

mod error {
    #[derive(Debug, PartialEq, Eq)]
    pub enum BinMatError {
        OutOfBounds,
        InvalidFileFormat,
        FileNotFound,
        FileWriteError,
    }

    impl std::fmt::Display for BinMatError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            match self {
                BinMatError::OutOfBounds => {
                    write!(f, "Coordinates are out of bounds")
                }
                BinMatError::InvalidFileFormat => {
                    write!(f, "Binary matrix format is invalid")
                }
                BinMatError::FileNotFound => {
                    write!(f, "Binary matrix file was not found")
                }
                BinMatError::FileWriteError => {
                    write!(f, "Binary matrix could not be written to file")
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
    fn test_extract() {
        const SIZE: u8 = 3;
        let mut mat = BinMat::new(SIZE, SIZE);

        // preparing some data
        mat.values[4] ^= true;
        mat.values[5] ^= true;

        // is the extracted ref valid?

        // row
        let extract = mat.row_iter(1);
        assert!(extract.is_ok());
        let extract: Vec<&bool> = extract.unwrap().collect();
        assert_eq!(extract, vec![&false, &true, &true]);

        // col
        let extract = mat.col_iter(1);
        assert!(extract.is_ok());
        let extract: Vec<&bool> = extract.unwrap().collect();
        assert_eq!(extract, vec![&false, &true, &false]);

        // the following extract should fail
        let extract = mat.row_iter(3);
        assert!(extract.is_err());
    }

    #[test]
    fn test_file() {
        // do we get the original pattern from loading a mat from that pattern?
        const PATTERN: &str = "3,3:101111000";
        assert_eq!(
            BinMat::from_string(String::from(PATTERN))
                .expect("An error occured")
                .format(),
            String::from(PATTERN)
        );

        // should return an error if format constraint is not satisfied
        assert!(BinMat::from_string(String::from("huihihihi")).is_err());
        // pretty funny but should not be done in practice (I hope so)
        assert!(BinMat::from_string(String::from("0,0:")).is_ok());
        // does it add missing values?
        assert_eq!(
            BinMat::from_string(String::from("2,2:011"))
                .expect("oops")
                .values
                .len(),
            4usize
        );
        // does it truncate excess values?
        assert_eq!(
            BinMat::from_string(String::from("2,2:01111"))
                .expect("oops")
                .values
                .len(),
            4usize
        );
    }
}
