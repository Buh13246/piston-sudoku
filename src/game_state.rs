//! Struct to save Game

use crate::gameboard::Gameboard;
use crate::winning_state::WinningState;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

/// Represenst the State of a Game (This can be saved)
#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    /// Stores the gameboard state.
    pub gameboard: Gameboard,
    /// Stores the winning state
    pub winning_state: WinningState,
}

impl GameState {
    /// Creates a new GameState
    pub fn new() -> GameState {
        GameState {
            gameboard: Gameboard::new(),
            winning_state: WinningState::new(),
        }
    }

    /// Load GameState from file
    pub fn load_from_filename(path: &str) -> Result<GameState, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut data: String = String::new();
        file.read_to_string(&mut data)?;
        let game_state: GameState = serde_json::from_str(&data)?;
        Ok(game_state)
    }

    /// Save GameState to file
    pub fn save_to_filename(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        let serialized = serde_json::to_string(&self).unwrap();

        file.write_all(&serialized.as_bytes())?;
        Ok(())
    }
}
