mod tetris_core;

use tetris_core::{Game, Cell};

fn main() {
    println!("Tetris Core Engine Demo");
    println!("=======================");
    
    // Create a new game instance
    let mut game = Game::new();
    
    // Demo of some game mechanics
    println!("Starting new game...");
    
    // Hard drop the first piece
    game.hard_drop();
    print_board_state(&game);
    
    // Preview next pieces
    let next_pieces = game.peek_next_pieces(3);
    println!("Next pieces: {:?}", next_pieces);
    
    // Demonstrate piece movement
    for _ in 0..3 {
        game.move_left();
    }
    println!("After moving left 3 times:");
    print_board_state(&game);
    
    // Rotation
    game.rotate_clockwise();
    println!("After clockwise rotation:");
    print_board_state(&game);
    
    // Hard drop and continue
    game.hard_drop();
    println!("After hard drop:");
    print_board_state(&game);
    
    // Show hold piece functionality
    game.hold_piece();
    println!("After holding piece:");
    print_board_state(&game);
    
    // Test perfect clear functionality
    println!("\nTesting Perfect Clear functionality...");
    // First, reset the game to have a clean board
    game.reset();
    
    // Setup a scenario that will result in a perfect clear
    // We'll use a custom board setup and piece position
    setup_perfect_clear_scenario(&mut game);
    println!("Perfect clear scenario setup:");
    print_board_state(&game);
    
    // Now perform the perfect clear
    println!("Performing move for perfect clear...");
    game.hard_drop();
    
    // Check the result
    println!("After perfect clear move:");
    print_board_state(&game);
    println!("Score after perfect clear: {}", game.score_system.score);
    
    // Run a short simulation
    println!("\nRunning short simulation of 10 drops...");
    for i in 1..=10 {
        // Randomly move and rotate
        if i % 3 == 0 {
            game.move_left();
        } else if i % 3 == 1 {
            game.move_right();
        }
        
        if i % 2 == 0 {
            game.rotate_clockwise();
        }
        
        // Drop the piece
        game.hard_drop();
        
        // Show status
        println!("Drop {}: Score: {}, Level: {}, Lines: {}", 
            i, 
            game.score_system.score, 
            game.score_system.level, 
            game.score_system.lines_cleared
        );
    }
    
    println!("\nFinal board state:");
    print_board_state(&game);
    
    println!("\nTetris Core Engine demo completed!");
}

// Helper function to print the board state
fn print_board_state(game: &Game) {
    println!("\nBoard:");
    
    // Create a temporary board copy to show current piece position
    let mut display_board = game.board.clone();
    
    // Place the current piece on the display board if it exists
    if let Some(ref piece) = game.current_piece {
        for &(row, col) in &piece.get_blocks() {
            if row < tetris_core::BOARD_HEIGHT && col < tetris_core::BOARD_WIDTH {
                display_board.set_cell(row, col, Cell::Filled(piece.piece_type));
            }
        }
    }
    
    // Print the visible part of the board
    for row in 0..tetris_core::VISIBLE_HEIGHT {
        print!("│");
        for col in 0..tetris_core::BOARD_WIDTH {
            match display_board.get_cell(row, col) {
                Some(Cell::Empty) => print!(" "),
                Some(Cell::Filled(_)) => print!("█"),
                None => print!("?"),
            }
        }
        println!("│");
    }
    println!("└{}┘", "─".repeat(tetris_core::BOARD_WIDTH));
}

// Implement Clone for Board to make the printing function work
impl Clone for tetris_core::Board {
    fn clone(&self) -> Self {
        let mut new_board = tetris_core::Board::new();
        
        for row in 0..tetris_core::BOARD_HEIGHT {
            for col in 0..tetris_core::BOARD_WIDTH {
                if let Some(cell) = self.get_cell(row, col) {
                    new_board.set_cell(row, col, *cell);
                }
            }
        }
        
        new_board
    }
}

// Helper function to set up a scenario for a Perfect Clear demonstration
fn setup_perfect_clear_scenario(game: &mut Game) {
    // Clear the board first
    game.board.clear();
    
    // Set up a board where one piece can clear the last two rows
    // Fill the bottom two rows except for 4 specific cells that an I piece can fill
    for row in (tetris_core::BOARD_HEIGHT - 2)..tetris_core::BOARD_HEIGHT {
        for col in 0..tetris_core::BOARD_WIDTH {
            // Leave gaps for our final I piece to clear everything
            if !(row == tetris_core::BOARD_HEIGHT - 1 && (col == 3 || col == 4 || col == 5 || col == 6)) {
                game.board.set_cell(row, col, Cell::Filled(tetris_core::PieceType::O));
            }
        }
    }
    
    // Create a new game with our prepared board
    // We'll need to manipulate the current piece to get an I piece in the right position
    game.reset();
    
    // Clear the board again but keep our setup
    let mut temp_board = tetris_core::Board::new();
    for row in (tetris_core::BOARD_HEIGHT - 2)..tetris_core::BOARD_HEIGHT {
        for col in 0..tetris_core::BOARD_WIDTH {
            if !(row == tetris_core::BOARD_HEIGHT - 1 && (col == 3 || col == 4 || col == 5 || col == 6)) {
                temp_board.set_cell(row, col, Cell::Filled(tetris_core::PieceType::O));
            }
        }
    }
    
    // Set the board
    game.board = temp_board;
    
    // Now we'll keep resetting until we get an I piece
    while game.current_piece.as_ref().map_or(true, |p| p.piece_type != tetris_core::PieceType::I) {
        game.reset();
        // Restore our board setup after reset
        for row in (tetris_core::BOARD_HEIGHT - 2)..tetris_core::BOARD_HEIGHT {
            for col in 0..tetris_core::BOARD_WIDTH {
                if !(row == tetris_core::BOARD_HEIGHT - 1 && (col == 3 || col == 4 || col == 5 || col == 6)) {
                    game.board.set_cell(row, col, Cell::Filled(tetris_core::PieceType::O));
                }
            }
        }
    }
    
    // Position the I piece above the gap
    // We'll use the public methods instead of accessing private members
    // First move to the correct horizontal position (approx)
    for _ in 0..5 {
        game.move_right();
    }
    
    // Try to rotate the piece to horizontal orientation
    game.rotate_clockwise();
    
    // The I piece should now be positioned to complete the perfect clear
}
