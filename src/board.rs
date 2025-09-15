/*!
Types to create and solve puzzles

This module defines how code can interact with a picross puzzle.
*/

use crate::binmat::BinMat;
use crate::board::error::BoardError;
use crate::utils::in_range;

const BOARD_MIN: usize = 25;

/**
The state of a puzzle tile.

The tiles' state is used for indication/visual purposes only, it is not
tied to check if the puzzle is solved or not.
Only the tile's value is used to check if the picross is solved.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileState {
    /// A tile's initial state
    Normal = 0,
    /// A tile marked as not part of the solution
    Marked = 1,
    /// A tile picked as part of the solution
    Picked = 2,
}

/**
The picross puzzle itself
*/
#[derive(Debug, Clone)]
pub struct Board {
    /// The width of each row
    width: u8,
    /// The total amount of tiles
    total: usize,
    /// Contains the state of each tile
    tile_states: Vec<TileState>,
    /// Contains the bool value of each tile
    /// If all values are false, then the puzzle is solved
    tile_values: Vec<bool>,
}

// private
impl Board {
    /// Checks if given coords are in range
    fn valid_coord(&self, x: u8, y: u8) -> bool {
        in_range(&x, 0, self.width)
            && in_range(&(y as usize), 0, self.total / (self.width as usize))
    }
}

// general implementation
impl Board {
    /**
    Returns a new empty `Board`

    It may return an error if the amount is too small (<25)
    */
    pub fn new(w: u8, h: u8) -> Result<Self, BoardError> {
        let total: usize = (w * h) as usize;
        if total < BOARD_MIN {
            return Err(BoardError::BoardIsTooSmall);
        }

        Ok(Board {
            width: w,
            total,
            tile_states: vec![TileState::Normal; total],
            tile_values: vec![false; total],
        })
    }

    /**
    Returns a new `Board` initialized from a binary matrix

    Returns an error if the matrix is too small.
    */
    pub fn from_mat(mat: &BinMat) -> Result<Self, BoardError> {
        let total: usize = (mat.get_width() * mat.get_height()) as usize;
        if total < BOARD_MIN {
            return Err(BoardError::BoardIsTooSmall);
        }

        Ok(Board {
            width: mat.get_width(),
            total,
            tile_states: vec![TileState::Normal; total],
            tile_values: mat.get_mat(),
        })
    }

    /// returns the board's width
    pub fn get_width(&self) -> u8 {
        self.width
    }

    /// calculates and returns the board's height
    pub fn get_height(&self) -> u8 {
        (self.total / self.width as usize) as u8
    }

    /// Returns a clone of the tile state vector
    pub fn get_states(&self) -> Vec<TileState> {
        self.tile_states.clone()
    }

    /// Returns a clone of the one tile's state
    pub fn get_state(&self, x: u8, y: u8) -> Result<TileState, BoardError> {
        if self.valid_coord(x, y) {
            Ok(self.tile_states[(x + y * self.width) as usize])
        } else {
            Err(BoardError::OutOfBounds {
                max: self.total,
                val: (x + y * self.width) as usize,
            })
        }
    }

    /// This function returns `true` if every value are false
    pub fn is_solved(&self) -> bool {
        !self.tile_values.iter().any(|&x| x)
    }
}

// changing state
impl Board {
    /// Sets the selected tile's state to the given one.
    /// Also toggles the tile's value if it can change the state
    fn set_state(&mut self, x: u8, y: u8, new_state: TileState) -> Result<(), BoardError> {
        if !self.valid_coord(x, y) {
            return Err(BoardError::OutOfBounds {
                max: self.total,
                val: (x + y * self.width) as usize,
            });
        }
        let id: usize = (x + y * self.width) as usize;

        let state = self.tile_states[id];
        match (state, new_state) {
            (TileState::Normal, TileState::Picked) | (TileState::Picked, TileState::Normal) => {
                self.tile_values[id] ^= true;
            }
            _ => {}
        }
        self.tile_states[id] = new_state;
        Ok(())
    }

    /// Toggle mark a tile
    pub fn mark_tile(&mut self, x: u8, y: u8) -> Result<(), BoardError> {
        self.set_state(x, y, TileState::Marked)
    }

    /// Toggle pick a tile
    pub fn pick_tile(&mut self, x: u8, y: u8) -> Result<(), BoardError> {
        self.set_state(x, y, TileState::Picked)
    }
}

mod error {
    #[derive(Debug, PartialEq, Eq)]
    pub enum BoardError {
        BoardIsTooSmall,
        OutOfBounds { max: usize, val: usize },
    }

    impl std::fmt::Display for BoardError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                BoardError::BoardIsTooSmall => {
                    write!(
                        f,
                        "The requested board size is too low, must be at least 5 by 5"
                    )
                }
                BoardError::OutOfBounds { max, val } => write!(
                    f,
                    "Attempted access at #{val} which is greater than max = {max}"
                ),
            }
        }
    }

    impl std::error::Error for BoardError {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let board = Board::new(5, 5);
        let noboard = Board::new(0, 0);

        // are they created correctly?
        assert!(!board.is_err());
        assert!(noboard.is_err());
        assert_eq!(noboard.err(), Some(BoardError::BoardIsTooSmall));
    }

    #[test]
    fn test_create_from_binmat() {
        let mut mat = BinMat::new(5, 5);

        // preparing mat
        mat.toggle_val(0, 0).expect("Not the subject of this test");

        let board = Board::from_mat(&mat);

        // is it created correctly?
        assert!(board.is_ok());

        // is the value correctly initialized?
        assert_eq!(board.unwrap().tile_values[0], true);
    }

    #[test]
    fn test_init_capacity() {
        let (w, h) = (5, 5);
        let board = Board::new(w, h).unwrap();

        // do board's vectors have the right capacity
        let (s_capa, v_capa) = (board.tile_states.capacity(), board.tile_values.capacity());
        let goal: usize = board.total;

        assert!(s_capa >= goal);
        assert!(v_capa >= goal);
    }

    #[test]
    fn test_vec_index() {
        const W: u8 = 6;
        const H: u8 = 6;

        let board = Board::new(W, H).unwrap();

        assert!(3 < board.width);
        assert!(4 < (board.total / board.width as usize));
    }

    #[test]
    fn test_bounds_checking() {
        const W: u8 = 6;
        const H: u8 = 6;

        let mut board = Board::new(W, H).unwrap();

        // should fail because we are out of bounds
        assert!(board.set_state(10, 10, TileState::Picked).is_err());
        assert!(board.get_state(10, 10).is_err());
    }

    #[test]
    fn test_changing_tilestate() {
        const W: u8 = 6;
        const H: u8 = 6;

        let mut board = Board::new(W, H).unwrap();

        // Marking
        assert!(board.mark_tile(3, 3).is_ok());
        assert_eq!(board.get_state(3, 3), Ok(TileState::Marked));

        // Picking a normal tile
        assert!(board.pick_tile(3, 4).is_ok());
        assert_eq!(board.get_state(3, 4), Ok(TileState::Picked));
    }

    #[test]
    fn test_solving() {
        const W: u8 = 5;
        const H: u8 = 5;

        let mut board = Board::new(W, H).unwrap();

        // is it already in a solved state? (blank)
        assert!(board.is_solved());

        // setup
        board.tile_values[4] ^= true;
        board.tile_values[9] ^= true;
        board.tile_values[15] ^= true;

        // is it set up correctly?
        assert!(!board.is_solved());

        // solving
        assert!(board.pick_tile(4, 0).is_ok());
        assert!(board.pick_tile(4, 1).is_ok());
        assert!(board.pick_tile(0, 3).is_ok());

        // is it solved?
        assert!(board.is_solved());
    }
}
