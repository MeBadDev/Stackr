use super::{BOARD_WIDTH, BOARD_HEIGHT};
use super::piece::{Piece, PieceType};

/// Represents a cell in the Tetris board
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Filled(PieceType), // Stores the piece type for color information
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

/// Represents the Tetris game board
pub struct Board {
    grid: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    /// Creates a new empty board
    pub fn new() -> Self {
        Board {
            grid: [[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    /// Gets the cell at the specified coordinates
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        if row < BOARD_HEIGHT && col < BOARD_WIDTH {
            Some(&self.grid[row][col])
        } else {
            None
        }
    }

    /// Sets the cell at the specified coordinates
    pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell) -> bool {
        if row < BOARD_HEIGHT && col < BOARD_WIDTH {
            self.grid[row][col] = cell;
            true
        } else {
            false
        }
    }

    /// Checks if a piece can be placed at the specified position
    pub fn can_place(&self, piece: &Piece) -> bool {
        for &(row, col) in &piece.get_blocks() {
            // Out of bounds check
            if row >= BOARD_HEIGHT || col >= BOARD_WIDTH {
                return false;
            }
            
            // Collision check
            if let Some(Cell::Filled(_)) = self.get_cell(row, col) {
                return false;
            }
        }
        true
    }

    /// Places a piece on the board permanently
    pub fn place_piece(&mut self, piece: &Piece) -> bool {
        if !self.can_place(piece) {
            return false;
        }

        for &(row, col) in &piece.get_blocks() {
            self.grid[row][col] = Cell::Filled(piece.piece_type);
        }
        true
    }

    /// Clears completed lines and returns the number of lines cleared
    pub fn clear_lines(&mut self) -> usize {
        let mut lines_cleared = 0;
        
        // Check each row, starting from the bottom
        for row in (0..BOARD_HEIGHT).rev() {
            if self.is_line_complete(row) {
                self.remove_line(row);
                lines_cleared += 1;
            }
        }
        
        lines_cleared
    }

    /// Checks if a line is complete (all cells filled)
    fn is_line_complete(&self, row: usize) -> bool {
        if row >= BOARD_HEIGHT {
            return false;
        }
        
        for col in 0..BOARD_WIDTH {
            if let Cell::Empty = self.grid[row][col] {
                return false;
            }
        }
        true
    }

    /// Removes a line and shifts all lines above down
    fn remove_line(&mut self, row: usize) {
        if row >= BOARD_HEIGHT {
            return;
        }
        
        // Shift all rows above down by one
        for r in (1..=row).rev() {
            self.grid[r] = self.grid[r - 1];
        }
        
        // Clear the top row
        self.grid[0] = [Cell::Empty; BOARD_WIDTH];
    }

    /// Checks if the board has collisions at the spawn point
    pub fn is_top_blocked(&self) -> bool {
        for col in 0..BOARD_WIDTH {
            if let Cell::Filled(_) = self.grid[0][col] {
                return true;
            }
        }
        false
    }

    /// Clears the entire board
    pub fn clear(&mut self) {
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                self.grid[row][col] = Cell::Empty;
            }
        }
    }

    /// Checks if the board is completely empty (Perfect Clear)
    pub fn is_perfect_clear(&self) -> bool {
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                if let Cell::Filled(_) = self.grid[row][col] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perfect_clear() {
        // Create an empty board
        let mut board = Board::new();
        
        // An empty board is a perfect clear
        assert!(board.is_perfect_clear());
        
        // Add a single block
        board.set_cell(10, 5, Cell::Filled(PieceType::T));
        
        // No longer a perfect clear
        assert!(!board.is_perfect_clear());
        
        // Clear the board
        board.clear();
        
        // Should be a perfect clear again
        assert!(board.is_perfect_clear());
    }
}