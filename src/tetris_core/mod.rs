// Tetris Core - A modern Tetris engine
// This module provides all the components needed to build a Tetris game

mod board;
mod piece;
mod game;
mod rotation;
mod randomizer;

// Re-export the main components
pub use board::{Board, Cell};
pub use piece::PieceType;
pub use game::Game;

// Constants for the game
pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 22;  // Including 2 hidden rows at the top
pub const VISIBLE_HEIGHT: usize = 20;