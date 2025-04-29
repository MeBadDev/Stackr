use super::board::Board;
use super::piece::{Piece, Rotation, PieceType};

/// Implements the Super Rotation System (SRS)
/// This handles wall kicks and rotation tests
pub struct RotationSystem;

impl RotationSystem {
    /// Attempts to rotate a piece clockwise on the board
    /// Returns the new piece if successful, or None if not possible
    pub fn rotate_clockwise(piece: &Piece, board: &Board) -> Option<Piece> {
        let mut rotated_piece = piece.clone();
        rotated_piece.rotate_clockwise();
        
        // Try each kick offset in sequence
        let kick_offsets = Self::get_kick_offsets(piece.piece_type, piece.rotation, rotated_piece.rotation);
        
        for &(row_offset, col_offset) in kick_offsets.iter() {
            let mut kicked_piece = rotated_piece.clone();
            kicked_piece.row += row_offset;
            kicked_piece.col += col_offset;
            
            // If this position works, return it
            if board.can_place(&kicked_piece) {
                return Some(kicked_piece);
            }
        }
        
        // No valid rotation found
        None
    }
    
    /// Attempts to rotate a piece counter-clockwise on the board
    /// Returns the new piece if successful, or None if not possible
    pub fn rotate_counterclockwise(piece: &Piece, board: &Board) -> Option<Piece> {
        let mut rotated_piece = piece.clone();
        rotated_piece.rotate_counterclockwise();
        
        // Try each kick offset in sequence
        let kick_offsets = Self::get_kick_offsets(piece.piece_type, piece.rotation, rotated_piece.rotation);
        
        for &(row_offset, col_offset) in kick_offsets.iter() {
            let mut kicked_piece = rotated_piece.clone();
            kicked_piece.row += row_offset;
            kicked_piece.col += col_offset;
            
            // If this position works, return it
            if board.can_place(&kicked_piece) {
                return Some(kicked_piece);
            }
        }
        
        // No valid rotation found
        None
    }
    
    /// Gets the kick offsets for a rotation according to SRS
    fn get_kick_offsets(piece_type: PieceType, from: Rotation, to: Rotation) -> &'static [(i32, i32)] {
        // The Super Rotation System (SRS) kick offsets
        if piece_type == PieceType::I {
            // I-piece has special kick data
            match (from, to) {
                (Rotation::North, Rotation::East) => &[(0, 0), (-1, 0), (2, 0), (-1, -2), (2, 1)],
                (Rotation::East, Rotation::North) => &[(0, 0), (1, 0), (-2, 0), (1, 2), (-2, -1)],
                (Rotation::East, Rotation::South) => &[(0, 0), (2, 0), (-1, 0), (2, -1), (-1, 2)],
                (Rotation::South, Rotation::East) => &[(0, 0), (-2, 0), (1, 0), (-2, 1), (1, -2)],
                (Rotation::South, Rotation::West) => &[(0, 0), (1, 0), (-2, 0), (1, 2), (-2, -1)],
                (Rotation::West, Rotation::South) => &[(0, 0), (-1, 0), (2, 0), (-1, -2), (2, 1)],
                (Rotation::West, Rotation::North) => &[(0, 0), (-2, 0), (1, 0), (-2, 1), (1, -2)],
                (Rotation::North, Rotation::West) => &[(0, 0), (2, 0), (-1, 0), (2, -1), (-1, 2)],
                _ => &[(0, 0)], // Should never happen with valid rotations
            }
        } else if piece_type == PieceType::O {
            // O-piece doesn't rotate
            &[(0, 0)]
        } else {
            // Standard kicks for J, L, S, T, Z pieces
            match (from, to) {
                (Rotation::North, Rotation::East) => &[(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                (Rotation::East, Rotation::North) => &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                (Rotation::East, Rotation::South) => &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                (Rotation::South, Rotation::East) => &[(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                (Rotation::South, Rotation::West) => &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                (Rotation::West, Rotation::South) => &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                (Rotation::West, Rotation::North) => &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                (Rotation::North, Rotation::West) => &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                _ => &[(0, 0)], // Should never happen with valid rotations
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{BOARD_WIDTH, BOARD_HEIGHT};
    use super::super::board::Cell;

    // Helper function to create a board with specific cells filled
    fn create_board_with_blocks(filled_cells: &[(usize, usize)]) -> Board {
        let mut board = Board::new();
        for &(row, col) in filled_cells {
            board.set_cell(row, col, Cell::Filled(PieceType::I));
        }
        board
    }
    
    #[test]
    fn test_basic_rotation_without_obstacles() {
        let board = Board::new();
        let piece = Piece::new(PieceType::T, 5, 5); // T piece in the middle of the board
        
        // Test clockwise rotation
        let rotated_cw = RotationSystem::rotate_clockwise(&piece, &board).unwrap();
        assert_eq!(rotated_cw.rotation, Rotation::East);
        
        // Test counter-clockwise rotation
        let rotated_ccw = RotationSystem::rotate_counterclockwise(&piece, &board).unwrap();
        assert_eq!(rotated_ccw.rotation, Rotation::West);
    }
    
    #[test]
    fn test_o_piece_rotation() {
        let board = Board::new();
        let o_piece = Piece::new(PieceType::O, 5, 5);
        
        // O pieces should maintain position but change rotation state
        let rotated_o = RotationSystem::rotate_clockwise(&o_piece, &board).unwrap();
        assert_eq!(rotated_o.row, o_piece.row);
        assert_eq!(rotated_o.col, o_piece.col);
        
        // Compare the actual blocks - they should be the same since O doesn't change shape
        let original_blocks = o_piece.get_blocks();
        let rotated_blocks = rotated_o.get_blocks();
        assert_eq!(original_blocks.len(), rotated_blocks.len());
        
        // Sort blocks to ensure order doesn't matter
        let mut original_sorted = original_blocks.clone();
        let mut rotated_sorted = rotated_blocks.clone();
        original_sorted.sort();
        rotated_sorted.sort();
        
        assert_eq!(original_sorted, rotated_sorted);
    }
    
    #[test]
    fn test_i_piece_wall_kick() {
        // Create a board with obstacles to force wall kick
        let board = create_board_with_blocks(&[(5, 7), (6, 7), (7, 7), (8, 7)]);
        
        // I-piece next to the obstacle
        let i_piece = Piece::new(PieceType::I, 6, 5);
        
        // Rotate clockwise - should perform a wall kick
        let rotated = RotationSystem::rotate_clockwise(&i_piece, &board);
        assert!(rotated.is_some(), "Rotation should succeed with a wall kick");
        
        // Verify the piece was rotated to the expected orientation
        let rotated = rotated.unwrap();
        assert_eq!(rotated.rotation, Rotation::East, "Piece should be rotated to East");
        
        // Verify all blocks are valid positions
        for &(row, col) in &rotated.get_blocks() {
            assert!(row < BOARD_HEIGHT && col < BOARD_WIDTH, 
                   "All blocks should be within board bounds after wall kick");
            assert!(col != 7, "No block should overlap with the obstacle column");
        }
    }
    
    #[test]
    fn test_wall_kick_near_wall() {
        let board = Board::new();
        
        // T-piece right against the left wall
        let t_piece = Piece::new(PieceType::T, 5, 0);
        
        // Rotation should kick away from wall
        let rotated = RotationSystem::rotate_clockwise(&t_piece, &board);
        assert!(rotated.is_some(), "Rotation should succeed with a wall kick");
        
        // The standard SRS kicks for T piece from North to East should move it to the right
        let rotated_piece = rotated.unwrap();
        
        // Verify that after rotation, all blocks are within bounds
        for &(row, col) in &rotated_piece.get_blocks() {
            assert!(col < BOARD_WIDTH, "Block should be within horizontal bounds");
            assert!(row < BOARD_HEIGHT, "Block should be within vertical bounds");
        }
    }
    
    #[test]
    fn test_rotation_blocked_completely() {
        // Create a board with obstacles that should prevent any rotation
        let mut board = Board::new();
        
        // We need to really block every possible rotation with wall kicks
        // Fill a larger area around the piece
        for row in 3..8 {
            for col in 3..8 {
                // Leave just the center position empty for the T piece
                if row == 5 && col == 5 {
                    continue;
                }
                board.set_cell(row, col, Cell::Filled(PieceType::I));
            }
        }
        
        // T-piece surrounded by blocks with no rotation possibility
        let t_piece = Piece::new(PieceType::T, 5, 5);
        
        // Both rotation attempts should fail
        let rotated_cw = RotationSystem::rotate_clockwise(&t_piece, &board);
        let rotated_ccw = RotationSystem::rotate_counterclockwise(&t_piece, &board);
        
        assert!(rotated_cw.is_none(), "Clockwise rotation should fail when completely blocked");
        assert!(rotated_ccw.is_none(), "Counter-clockwise rotation should fail when completely blocked");
    }
    
    #[test]
    fn test_rotation_at_board_edge() {
        let board = Board::new();
        
        // Test pieces at various edges
        
        // Bottom edge
        let bottom_piece = Piece::new(PieceType::T, BOARD_HEIGHT as i32 - 2, 5);
        let rotated = RotationSystem::rotate_clockwise(&bottom_piece, &board);
        assert!(rotated.is_some());
        
        // Right edge
        let right_piece = Piece::new(PieceType::J, 5, BOARD_WIDTH as i32 - 2);
        let rotated = RotationSystem::rotate_clockwise(&right_piece, &board);
        assert!(rotated.is_some());
        
        // Corner case
        let corner_piece = Piece::new(PieceType::L, BOARD_HEIGHT as i32 - 2, BOARD_WIDTH as i32 - 2);
        let rotated = RotationSystem::rotate_clockwise(&corner_piece, &board);
        // This might succeed or fail depending on the kick offsets
        if let Some(kicked_piece) = rotated {
            // Make sure if it succeeded, the piece is still on the board
            for &(row, col) in &kicked_piece.get_blocks() {
                assert!(row < BOARD_HEIGHT);
                assert!(col < BOARD_WIDTH);
            }
        }
    }
    
    #[test]
    fn test_tspin_setup() {
        // Create a board with a T-spin setup
        // A classic T-spin setup has blocks in the 3 corners around the T
        let mut board = Board::new();
        // Place blocks for a T-spin setup
        board.set_cell(10, 4, Cell::Filled(PieceType::I));
        board.set_cell(10, 6, Cell::Filled(PieceType::I));
        board.set_cell(12, 4, Cell::Filled(PieceType::I));
        board.set_cell(12, 6, Cell::Filled(PieceType::I));
        
        // T-piece in position for T-spin
        let t_piece = Piece::new(PieceType::T, 11, 5);
        
        // Rotation should succeed (basic T-spin)
        let rotated = RotationSystem::rotate_clockwise(&t_piece, &board);
        assert!(rotated.is_some());
        
        // T-spin rotated should be in correct position
        let rotated_t = rotated.unwrap();
        assert_eq!(rotated_t.rotation, Rotation::East);
    }
    
    #[test]
    fn test_consecutive_rotations() {
        let board = Board::new();
        let piece = Piece::new(PieceType::T, 5, 5);
        
        // Do 4 clockwise rotations - should end up in the original rotation
        let mut current = piece.clone();
        for _ in 0..4 {
            let rotated = RotationSystem::rotate_clockwise(&current, &board).unwrap();
            current = rotated;
        }
        
        assert_eq!(current.rotation, Rotation::North);
        
        // Do 4 counter-clockwise rotations - should also end up in the original rotation
        let mut current = piece.clone();
        for _ in 0..4 {
            let rotated = RotationSystem::rotate_counterclockwise(&current, &board).unwrap();
            current = rotated;
        }
        
        assert_eq!(current.rotation, Rotation::North);
    }
    
    #[test]
    fn test_all_piece_types_rotation() {
        let board = Board::new();
        
        // Test rotation for each piece type
        let piece_types = [
            PieceType::I, 
            PieceType::O, 
            PieceType::T, 
            PieceType::S, 
            PieceType::Z,
            PieceType::J, 
            PieceType::L
        ];
        
        for &piece_type in &piece_types {
            let piece = Piece::new(piece_type, 5, 5);
            
            // All pieces should be able to rotate clockwise without obstacles
            let rotated_cw = RotationSystem::rotate_clockwise(&piece, &board);
            assert!(rotated_cw.is_some());
            
            // All pieces should be able to rotate counter-clockwise without obstacles
            let rotated_ccw = RotationSystem::rotate_counterclockwise(&piece, &board);
            assert!(rotated_ccw.is_some());
        }
    }
    
    #[test]
    fn test_i_piece_special_kicks() {
        // I-piece has special kick data - test it specifically
        let board = Board::new();
        let i_piece = Piece::new(PieceType::I, 5, 5);
        
        // Complete a full rotation cycle and check each intermediate rotation
        let east_piece = RotationSystem::rotate_clockwise(&i_piece, &board).unwrap();
        assert_eq!(east_piece.rotation, Rotation::East);
        
        let south_piece = RotationSystem::rotate_clockwise(&east_piece, &board).unwrap();
        assert_eq!(south_piece.rotation, Rotation::South);
        
        let west_piece = RotationSystem::rotate_clockwise(&south_piece, &board).unwrap();
        assert_eq!(west_piece.rotation, Rotation::West);
        
        let north_again = RotationSystem::rotate_clockwise(&west_piece, &board).unwrap();
        assert_eq!(north_again.rotation, Rotation::North);
    }
}