//! Gameboard controller.

use piston::input::GenericEvent;

use crate::game_state::GameState;

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// the current GameState
    pub game_state: GameState,
    /// Selected cell.
    pub selected_cell: Option<[usize; 2]>,
    /// Stores last mouse cursor position.
    cursor_pos: [f64; 2],
}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(game_state: GameState) -> GameboardController {
        GameboardController {
            game_state: game_state,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    /// Loads the default save to continue on startup
    pub fn load_default_save(&mut self) {
        match GameState::load_from_filename("saves/current_save.json") {
            Ok(game_state) => {
                self.game_state = game_state;
            }
            Err(err) => {
                println!("Error loading save: {}", err);
            }
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        use piston::input::{Button, Key, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];

            // Check boundaries
            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                let cell_x = (x / size * 9.0) as usize;
                let cell_y = (y / size * 9.0) as usize;
                self.selected_cell = Some([cell_x, cell_y]);
            }
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if let Some(cell) = self.selected_cell {
                let mut changed = true;
                // Set cell value.
                match key {
                    Key::D0 => self.game_state.gameboard.set(cell, 0),
                    Key::D1 => self.game_state.gameboard.set(cell, 1),
                    Key::D2 => self.game_state.gameboard.set(cell, 2),
                    Key::D3 => self.game_state.gameboard.set(cell, 3),
                    Key::D4 => self.game_state.gameboard.set(cell, 4),
                    Key::D5 => self.game_state.gameboard.set(cell, 5),
                    Key::D6 => self.game_state.gameboard.set(cell, 6),
                    Key::D7 => self.game_state.gameboard.set(cell, 7),
                    Key::D8 => self.game_state.gameboard.set(cell, 8),
                    Key::D9 => self.game_state.gameboard.set(cell, 9),
                    _ => {
                        changed = false;
                    }
                }
                match key {
                    Key::S => {
                        self.game_state
                            .save_to_filename("saves/current_save.json")
                            .expect("Error");
                    }
                    _ => {}
                }
                if changed {
                    self.game_state
                        .winning_state
                        .update(&self.game_state.gameboard, cell);
                    if self.game_state.winning_state.game_won {
                        panic!("Game won!!!!");
                    }
                }
            }
        }
    }
}
