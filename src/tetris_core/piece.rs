use super::rotation::RotationSystem;

/// Represents the different types of Tetris pieces
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    I, // I-piece (cyan)
    O, // O-piece (yellow)
    T, // T-piece (purple)
    S, // S-piece (green)
    Z, // Z-piece (red)
    J, // J-piece (blue)
    L, // L-piece (orange)
}

/// Represents a piece direction/orientation
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rotation {
    North = 0,
    East = 1, 
    South = 2,
    West = 3,
}

impl Rotation {
    /// Rotates clockwise
    pub fn rotate_cw(self) -> Self {
        match self {
            Rotation::North => Rotation::East,
            Rotation::East => Rotation::South,
            Rotation::South => Rotation::West,
            Rotation::West => Rotation::North,
        }
    }
    
    /// Rotates counter-clockwise
    pub fn rotate_ccw(self) -> Self {
        match self {
            Rotation::North => Rotation::West,
            Rotation::East => Rotation::North,
            Rotation::South => Rotation::East,
            Rotation::West => Rotation::South,
        }
    }

    /// Converts rotation to index (0-3)
    pub fn to_index(self) -> usize {
        self as usize
    }
}

/// Represents a Tetris piece with position and rotation
pub struct Piece {
    pub piece_type: PieceType,
    pub row: i32,        // Using i32 for positions to allow negative values during rotations
    pub col: i32,
    pub rotation: Rotation,
}

impl Piece {
    /// Creates a new piece of the specified type at the given position
    pub fn new(piece_type: PieceType, row: i32, col: i32) -> Self {
        Piece {
            piece_type,
            row,
            col,
            rotation: Rotation::North,
        }
    }
    
    /// Get all block coordinates for this piece in its current position and rotation
    pub fn get_blocks(&self) -> Vec<(usize, usize)> {
        let offsets = self.get_block_offsets();
        
        let blocks = offsets.iter()
            .filter_map(|&(row_offset, col_offset)| {
                let row = self.row + row_offset;
                let col = self.col + col_offset;
                
                // Convert to usize, but only if non-negative
                if row >= 0 && col >= 0 {
                    Some((row as usize, col as usize))
                } else {
                    None
                }
            })
            .collect();
            
        blocks
    }
    
    /// Get the block offsets for this piece in its current rotation
    fn get_block_offsets(&self) -> [(i32, i32); 4] {
        // These offsets follow the standard SRS (Super Rotation System) used in guideline Tetris
        match (self.piece_type, self.rotation) {
            // I-piece - using standard guideline offsets
            (PieceType::I, Rotation::North) => [(0, -1), (0, 0), (0, 1), (0, 2)],
            (PieceType::I, Rotation::East) => [(-1, 1), (0, 1), (1, 1), (2, 1)],
            (PieceType::I, Rotation::South) => [(1, -1), (1, 0), (1, 1), (1, 2)],
            (PieceType::I, Rotation::West) => [(-1, 0), (0, 0), (1, 0), (2, 0)],
            
            // O-piece - uses single position for all rotations per guideline
            (PieceType::O, _) => [(0, 0), (0, 1), (1, 0), (1, 1)],
            
            // T-piece - standard guideline positions
            (PieceType::T, Rotation::North) => [(0, 0), (0, -1), (0, 1), (1, 0)],
            (PieceType::T, Rotation::East) => [(0, 0), (-1, 0), (1, 0), (0, 1)],
            (PieceType::T, Rotation::South) => [(0, 0), (0, -1), (0, 1), (-1, 0)],
            (PieceType::T, Rotation::West) => [(0, 0), (-1, 0), (1, 0), (0, -1)],
            
            // S-piece - standard guideline positions
            (PieceType::S, Rotation::North) => [(0, 0), (0, -1), (1, 0), (1, 1)],
            (PieceType::S, Rotation::East) => [(0, 0), (1, 0), (0, 1), (-1, 1)],
            (PieceType::S, Rotation::South) => [(0, 0), (0, 1), (-1, 0), (-1, -1)],
            (PieceType::S, Rotation::West) => [(0, 0), (-1, 0), (0, -1), (1, -1)],
            
            // Z-piece - standard guideline positions
            (PieceType::Z, Rotation::North) => [(0, 0), (0, 1), (1, 0), (1, -1)],
            (PieceType::Z, Rotation::East) => [(0, 0), (-1, 0), (0, -1), (1, -1)],
            (PieceType::Z, Rotation::South) => [(0, 0), (0, -1), (-1, 0), (-1, 1)],
            (PieceType::Z, Rotation::West) => [(0, 0), (1, 0), (0, 1), (-1, 1)],
            
            // J-piece - standard guideline positions
            (PieceType::J, Rotation::North) => [(0, 0), (0, -1), (0, 1), (-1, 1)],
            (PieceType::J, Rotation::East) => [(0, 0), (-1, 0), (1, 0), (-1, -1)],
            (PieceType::J, Rotation::South) => [(0, 0), (0, 1), (0, -1), (1, -1)],
            (PieceType::J, Rotation::West) => [(0, 0), (1, 0), (-1, 0), (1, 1)],
            
            // L-piece - standard guideline positions
            (PieceType::L, Rotation::North) => [(0, 0), (0, -1), (0, 1), (1, 1)],
            (PieceType::L, Rotation::East) => [(0, 0), (-1, 0), (1, 0), (1, -1)],
            (PieceType::L, Rotation::South) => [(0, 0), (0, 1), (0, -1), (-1, -1)],
            (PieceType::L, Rotation::West) => [(0, 0), (1, 0), (-1, 0), (-1, 1)],
        }
    }
    
    /// Move the piece left
    pub fn move_left(&mut self) {
        self.col -= 1;
    }
    
    /// Move the piece right
    pub fn move_right(&mut self) {
        self.col += 1;
    }
    
    /// Move the piece down
    pub fn move_down(&mut self) {
        self.row += 1;
    }
    
    /// Rotate the piece clockwise
    pub fn rotate_clockwise(&mut self) {
        self.rotation = self.rotation.rotate_cw();
    }
    
    /// Rotate the piece counter-clockwise
    pub fn rotate_counterclockwise(&mut self) {
        self.rotation = self.rotation.rotate_ccw();
    }
    
    /// Creates a clone of this piece with a clockwise rotation
    pub fn with_clockwise_rotation(&self) -> Self {
        let mut new_piece = self.clone();
        new_piece.rotate_clockwise();
        new_piece
    }
    
    /// Creates a clone of this piece with a counter-clockwise rotation
    pub fn with_counterclockwise_rotation(&self) -> Self {
        let mut new_piece = self.clone();
        new_piece.rotate_counterclockwise();
        new_piece
    }
    
    /// Creates a clone of this piece moved left
    pub fn with_left_move(&self) -> Self {
        let mut new_piece = self.clone();
        new_piece.move_left();
        new_piece
    }
    
    /// Creates a clone of this piece moved right
    pub fn with_right_move(&self) -> Self {
        let mut new_piece = self.clone();
        new_piece.move_right();
        new_piece
    }
    
    /// Creates a clone of this piece moved down
    pub fn with_down_move(&self) -> Self {
        let mut new_piece = self.clone();
        new_piece.move_down();
        new_piece
    }
}

impl Clone for Piece {
    fn clone(&self) -> Self {
        Piece {
            piece_type: self.piece_type,
            row: self.row,
            col: self.col,
            rotation: self.rotation,
        }
    }
}