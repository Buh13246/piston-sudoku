//! Winning State
/// Saves for each winning condition if they are already satisfied.
use crate::gameboard::Gameboard;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
/// Represents the winning State
pub struct WinningState {
    /// Winning State for sections
    sections_winning_state: [bool; 9],
    /// Winning State for rows
    rows_winning_state: [bool; 9],
    /// Winning State for colums
    columns_winning_state: [bool; 9],
    /// Game is won
    pub game_won: bool,
}

impl WinningState {
    /// Creates a new WinningState
    pub fn new() -> WinningState {
        WinningState {
            sections_winning_state: [false; 9],
            rows_winning_state: [false; 9],
            columns_winning_state: [false; 9],
            game_won: false,
        }
    }

    /// Updates the winning state based on Gameboard an new cell
    pub fn update(&mut self, gameboard: &Gameboard, cell: [usize; 2]) {
        let section_check: bool = self.check_section(gameboard, cell);
        let row_check: bool = self.check_row(gameboard, cell);
        let column_check: bool = self.check_column(gameboard, cell);

        let idx_section_x = cell[0] / 3;
        let idx_section_y = cell[1] / 3;
        let idx_section = idx_section_y * 3 + idx_section_x;
        let idx_row = cell[1];
        let idx_column = cell[0];

        self.sections_winning_state[idx_section] = section_check;
        self.rows_winning_state[idx_row] = row_check;
        self.columns_winning_state[idx_column] = column_check;

        if section_check && row_check && column_check {
            for i in 0..9 {
                if !(self.sections_winning_state[i]
                    && self.rows_winning_state[i]
                    && self.columns_winning_state[i])
                {
                    return;
                }
            }
            self.game_won = true;
        }
    }

    /// Checks if the containing Section contains all numbers.
    fn check_section(&mut self, gameboard: &Gameboard, cell: [usize; 2]) -> bool {
        let section_start_x = cell[0] - (cell[0] % 3);
        let section_start_y = cell[1] - (cell[1] % 3);

        let mut counting_sort_array: [u8; 10] = [0; 10];

        for dx in 0..3 {
            for dy in 0..3 {
                let val = gameboard
                    .get([section_start_x + dx, section_start_y + dy])
                    .unwrap();
                counting_sort_array[val as usize] += 1;
            }
        }

        for i in 1..10 {
            if counting_sort_array[i] != 1 {
                return false;
            }
        }

        return true;
    }

    /// Checks if the containing row contains all numbers.
    fn check_row(&mut self, gameboard: &Gameboard, cell: [usize; 2]) -> bool {
        let y = cell[1];

        let mut counting_sort_array: [u8; 10] = [0; 10];

        for x in 0..9 {
            let val = gameboard.get([x, y]).unwrap();
            counting_sort_array[val as usize] += 1;
        }

        for i in 1..10 {
            if counting_sort_array[i] != 1 {
                return false;
            }
        }

        return true;
    }

    /// Checks if the containing row contains all numbers.
    fn check_column(&mut self, gameboard: &Gameboard, cell: [usize; 2]) -> bool {
        let x = cell[0];

        let mut counting_sort_array: [u8; 10] = [0; 10];

        for y in 0..9 {
            let val = gameboard.get([x, y]).unwrap();
            counting_sort_array[val as usize] += 1;
        }

        for i in 1..10 {
            if counting_sort_array[i] != 1 {
                return false;
            }
        }

        return true;
    }
}
