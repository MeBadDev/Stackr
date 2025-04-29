use crate::tetris_core::{Game, BOARD_WIDTH};

/// Represents a move that can be performed by the bot
#[derive(Clone, Debug)]
pub struct Move {
    /// Number of left movements to perform
    pub left_moves: u8,
    /// Number of right movements to perform
    pub right_moves: u8,
    /// Number of clockwise rotations to perform
    pub clockwise_rotations: u8,
    /// Number of counter-clockwise rotations to perform
    pub counterclockwise_rotations: u8,
    /// Whether to hard drop immediately
    pub hard_drop: bool,
    /// Whether to hold the piece
    pub hold: bool,
}

impl Move {
    /// Create a new move
    pub fn new(
        left_moves: u8,
        right_moves: u8,
        clockwise_rotations: u8,
        counterclockwise_rotations: u8,
        hard_drop: bool,
        hold: bool,
    ) -> Self {
        Move {
            left_moves,
            right_moves,
            clockwise_rotations,
            counterclockwise_rotations,
            hard_drop,
            hold,
        }
    }
}

/// Finds and applies possible moves for the Tetris bot
pub struct MoveFinder {
    max_moves_to_consider: usize,
}

impl MoveFinder {
    /// Create a new move finder with default settings
    pub fn new() -> Self {
        MoveFinder {
            max_moves_to_consider: 500, // Limit to avoid excessive computation
        }
    }
    
    /// Create a new move finder with custom max moves to consider
    pub fn with_max_moves(max_moves: usize) -> Self {
        MoveFinder {
            max_moves_to_consider: max_moves,
        }
    }
    
    /// Find all possible moves for the current piece
    pub fn find_possible_moves(&self, game: &Game) -> Vec<Move> {
        let mut moves = Vec::new();
        
        // Check if the current piece is valid
        if game.current_piece.is_none() {
            return moves;
        }
        
        // Consider holding the piece first
        if game.can_hold {
            moves.push(Move::new(0, 0, 0, 0, true, true));
        }
        
        // Consider rotations: 0, 1, 2, or 3 clockwise rotations
        for clockwise_rotations in 0..4 {
            // For each rotation, try every possible horizontal position
            for position in 0..BOARD_WIDTH {
                // Calculate left or right moves needed to reach this position
                let mut game_clone = game.clone();
                
                // Apply rotations
                for _ in 0..clockwise_rotations {
                    if !game_clone.rotate_clockwise() {
                        break;
                    }
                }
                
                // Get the horizontal position of the piece after rotation
                let current_position = if let Some(ref piece) = game_clone.current_piece {
                    piece.col as usize
                } else {
                    continue;
                };
                
                // Calculate and apply horizontal moves
                let (left_moves, right_moves) = if position < current_position {
                    ((current_position - position) as u8, 0)
                } else {
                    (0, (position - current_position) as u8)
                };
                
                // Create a move and add to possible moves
                let new_move = Move::new(
                    left_moves,
                    right_moves,
                    clockwise_rotations,
                    0,
                    true,
                    false
                );
                
                moves.push(new_move);
                
                // Limit the number of moves to avoid excessive computation
                if moves.len() >= self.max_moves_to_consider {
                    return moves;
                }
            }
        }
        
        // Also consider counter-clockwise rotations for more optimal moves
        for counterclockwise_rotations in 1..4 {
            // For each rotation, try every possible horizontal position
            for position in 0..BOARD_WIDTH {
                // Calculate left or right moves needed to reach this position
                let mut game_clone = game.clone();
                
                // Apply rotations
                for _ in 0..counterclockwise_rotations {
                    if !game_clone.rotate_counterclockwise() {
                        break;
                    }
                }
                
                // Get the horizontal position of the piece after rotation
                let current_position = if let Some(ref piece) = game_clone.current_piece {
                    piece.col as usize
                } else {
                    continue;
                };
                
                // Calculate and apply horizontal moves
                let (left_moves, right_moves) = if position < current_position {
                    ((current_position - position) as u8, 0)
                } else {
                    (0, (position - current_position) as u8)
                };
                
                // Create a move and add to possible moves
                let new_move = Move::new(
                    left_moves,
                    right_moves,
                    0,
                    counterclockwise_rotations,
                    true,
                    false
                );
                
                moves.push(new_move);
                
                // Limit the number of moves to avoid excessive computation
                if moves.len() >= self.max_moves_to_consider {
                    return moves;
                }
            }
        }
        
        moves
    }
    
    /// Apply a move to the game state
    pub fn apply_move(&self, game: &mut Game, move_to_apply: &Move) -> bool {
        // Apply hold if needed
        if move_to_apply.hold && game.can_hold {
            if !game.hold_piece() {
                return false;
            }
        }
        
        // Apply rotations
        for _ in 0..move_to_apply.clockwise_rotations {
            if !game.rotate_clockwise() {
                return false;
            }
        }
        
        for _ in 0..move_to_apply.counterclockwise_rotations {
            if !game.rotate_counterclockwise() {
                return false;
            }
        }
        
        // Apply horizontal movements
        for _ in 0..move_to_apply.left_moves {
            if !game.move_left() {
                return false;
            }
        }
        
        for _ in 0..move_to_apply.right_moves {
            if !game.move_right() {
                return false;
            }
        }
        
        // Hard drop if needed
        if move_to_apply.hard_drop {
            if !game.hard_drop() {
                return false;
            }
        }
        
        true
    }
    
    /// Test if a move is valid by simulating it
    pub fn is_valid_move(&self, game: &Game, move_to_test: &Move) -> bool {
        let mut game_clone = game.clone();
        self.apply_move(&mut game_clone, move_to_test)
    }
}