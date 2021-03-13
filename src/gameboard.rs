//! Game board logic
use serde::{Deserialize, Serialize};

/// Size of game board.
const SIZE: usize = 9;

#[derive(Serialize, Deserialize, Debug)]
/// Gameboard State struct
pub struct Gameboard {
    /// Representation of Cells
    /// `0` is an empty cell.
    pub cells: [[u8; SIZE]; SIZE],
}

impl Gameboard {
    /// Creates a new Gameboard
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
        }
    }

    /// Return the character
    pub fn char(&self, cell: [usize; 2]) -> Option<char> {
        Some(match self.cells[cell[0]][cell[1]] {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            0 => '0',
            _ => return None,
        })
    }

    /// Get the value of cell
    pub fn get(&self, cell: [usize; 2]) -> Option<u8> {
        Some(self.cells[cell[0]][cell[1]])
    }

    /// Sets the cell to val
    pub fn set(&mut self, cell: [usize; 2], val: u8) {
        self.cells[cell[0]][cell[1]] = val;
    }
}
