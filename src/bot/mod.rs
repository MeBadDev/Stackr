mod evaluator;
mod move_finder;

use super::tetris_core::{Game, GameState};
use move_finder::MoveFinder;
use evaluator::BoardEvaluator;

/// The main bot that plays Tetris
pub struct TetrisBot {
    evaluator: BoardEvaluator,
    move_finder: MoveFinder,
}

impl TetrisBot {
    /// Create a new Tetris bot
    pub fn new() -> Self {
        TetrisBot {
            evaluator: BoardEvaluator::new(),
            move_finder: MoveFinder::new(),
        }
    }

    /// Find and execute the best move for the current game state
    pub fn make_move(&self, game: &mut Game) -> bool {
        // Get all possible moves for the current piece
        let possible_moves = self.move_finder.find_possible_moves(game);
        
        if possible_moves.is_empty() {
            return false; // No moves available
        }
        
        // Evaluate each move and find the best one
        let mut best_move = &possible_moves[0];
        let mut best_score = f64::NEG_INFINITY;
        
        for possible_move in &possible_moves {
            // Clone the game to simulate the move
            let mut game_clone = game.clone();
            
            // Apply the move to the clone
            self.move_finder.apply_move(&mut game_clone, possible_move);
            
            // Evaluate the resulting board
            let score = self.evaluator.evaluate(&game_clone);
            
            // Update best move if this is better
            if score > best_score {
                best_score = score;
                best_move = possible_move;
            }
        }
        
        // Apply the best move to the actual game
        self.move_finder.apply_move(game, best_move);
        
        true
    }
    
    /// Play the game automatically until game over
    pub fn play_game(&self, game: &mut Game) {
        while game.state == GameState::Playing {
            if !self.make_move(game) {
                break; // No more moves possible
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bot_can_make_move() {
        let bot = TetrisBot::new();
        let mut game = Game::new();
        
        assert!(bot.make_move(&mut game));
    }
}