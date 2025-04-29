use std::time::{Duration, Instant};
use super::board::Board;
use super::piece::{Piece, PieceType};
use super::randomizer::{Randomizer, BagRandomizer};
use super::rotation::RotationSystem;
use super::{BOARD_WIDTH, BOARD_HEIGHT};

/// Represents the current state of the game
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
}

/// Represents the scoring system for the Tetris game
pub struct ScoreSystem {
    pub score: u32,
    pub level: u32,
    pub lines_cleared: u32,
}

impl ScoreSystem {
    pub fn new() -> Self {
        ScoreSystem {
            score: 0,
            level: 1,
            lines_cleared: 0,
        }
    }
    
    /// Add score based on the number of lines cleared
    pub fn add_score_for_lines(&mut self, lines: usize) {
        if lines == 0 {
            return;
        }
        
        // Score based on modern Tetris guidelines
        let line_multiplier = match lines {
            1 => 100,    // Single
            2 => 300,    // Double
            3 => 500,    // Triple
            4 => 800,    // Tetris
            _ => 0,      // Invalid
        };
        
        self.score += line_multiplier * self.level;
        self.lines_cleared += lines as u32;
        
        // Level up every 10 lines
        self.level = (self.lines_cleared / 10) + 1;
    }
    
    /// Add score based on lines cleared with T-spin bonus
    pub fn add_score_for_lines_with_tspin(&mut self, lines: usize, tspin_type: TSpinType) {
        if lines == 0 {
            // No lines cleared
            match tspin_type {
                TSpinType::Full => self.score += 400 * self.level, // T-spin no lines
                TSpinType::Mini => self.score += 100 * self.level, // Mini T-spin no lines
                TSpinType::None => {} // No bonus
            }
            return;
        }
        
        // Calculate score based on clear type and T-spin status
        let line_multiplier = match (lines, tspin_type) {
            // T-spin line clears
            (1, TSpinType::Full) => 800,    // T-spin Single
            (2, TSpinType::Full) => 1200,   // T-spin Double
            (3, TSpinType::Full) => 1600,   // T-spin Triple
            
            // Mini T-spin line clears
            (1, TSpinType::Mini) => 200,    // Mini T-spin Single
            (2, TSpinType::Mini) => 400,    // Mini T-spin Double
            
            // Regular line clears
            (1, TSpinType::None) => 100,    // Single
            (2, TSpinType::None) => 300,    // Double
            (3, TSpinType::None) => 500,    // Triple
            (4, TSpinType::None) => 800,    // Tetris
            
            // Fallback (shouldn't happen)
            (_, _) => 0,
        };
        
        self.score += line_multiplier * self.level;
        self.lines_cleared += lines as u32;
        
        // Level up every 10 lines
        self.level = (self.lines_cleared / 10) + 1;
    }
    
    /// Add score for a perfect clear (all lines cleared from the board)
    pub fn add_perfect_clear_bonus(&mut self, lines: usize) {
        // Perfect clear bonuses based on number of lines
        let bonus = match lines {
            1 => 800,     // PC Single
            2 => 1200,    // PC Double
            3 => 1800,    // PC Triple
            4 => 2000,    // PC Tetris
            _ => 0,
        };
        
        self.score += bonus * self.level;
    }
    
    /// Add score for a soft drop (manually moving down)
    pub fn add_soft_drop_score(&mut self, rows: u32) {
        self.score += rows;
    }
    
    /// Add score for a hard drop (instant drop)
    pub fn add_hard_drop_score(&mut self, rows: u32) {
        self.score += rows * 2;
    }
}

/// T-spin detection types
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TSpinType {
    None,
    Mini,
    Full
}

// Lock delay constants
const LOCK_DELAY: Duration = Duration::from_millis(500); // Standard 0.5s lock delay
const MAX_LOCK_RESETS: u8 = 15; // Maximum number of lock delay resets

/// The main game controller for Tetris
pub struct Game {
    pub board: Board,
    pub current_piece: Option<Piece>,
    pub held_piece: Option<PieceType>,
    pub can_hold: bool,
    pub state: GameState,
    pub score_system: ScoreSystem,
    randomizer: Box<dyn Randomizer>,
    time_since_last_drop: Duration,
    gravity_delay: Duration,
    // Lock delay fields
    lock_delay_timer: Duration,
    lock_delay_active: bool,
    lock_delay_resets: u8,
    last_successful_movement: Instant,
}

impl Game {
    /// Create a new Tetris game
    pub fn new() -> Self {
        let mut game = Game {
            board: Board::new(),
            current_piece: None,
            held_piece: None,
            can_hold: true,
            state: GameState::Playing,
            score_system: ScoreSystem::new(),
            randomizer: Box::new(BagRandomizer::new()),
            time_since_last_drop: Duration::ZERO,
            gravity_delay: Duration::from_millis(1000), // Initial gravity speed
            // Initialize lock delay fields
            lock_delay_timer: Duration::ZERO,
            lock_delay_active: false,
            lock_delay_resets: 0,
            last_successful_movement: Instant::now(),
        };
        
        // Spawn the first piece
        game.spawn_new_piece();
        
        game
    }
    
    /// Update the game state based on elapsed time
    pub fn update(&mut self, dt: Duration) -> bool {
        if self.state != GameState::Playing {
            return false;
        }
        
        // Apply gravity
        self.time_since_last_drop += dt;
        if self.time_since_last_drop >= self.gravity_delay {
            self.time_since_last_drop = Duration::ZERO;
            
            // Try to move piece down
            if let Some(ref current_piece) = self.current_piece {
                let moved_piece = current_piece.with_down_move();
                if self.board.can_place(&moved_piece) {
                    self.current_piece = Some(moved_piece);
                    // Reset lock delay when piece moves down successfully
                    self.lock_delay_active = false;
                    self.lock_delay_timer = Duration::ZERO;
                } else {
                    // Start lock delay if it's not active
                    if !self.lock_delay_active {
                        self.lock_delay_active = true;
                        self.lock_delay_timer = Duration::ZERO;
                        self.lock_delay_resets = 0;
                    }
                }
            }
        }
        
        // Process lock delay
        if self.lock_delay_active {
            self.lock_delay_timer += dt;
            if self.lock_delay_timer >= LOCK_DELAY {
                // Lock delay expired, lock the piece
                self.lock_piece();
                self.lock_delay_active = false;
                self.lock_delay_timer = Duration::ZERO;
            }
        }
        
        true
    }
    
    /// Attempt to reset lock delay when the player moves or rotates
    fn try_reset_lock_delay(&mut self) {
        if self.lock_delay_active && self.lock_delay_resets < MAX_LOCK_RESETS {
            self.lock_delay_timer = Duration::ZERO;
            self.lock_delay_resets += 1;
        }
    }
    
    /// Move the current piece left if possible
    pub fn move_left(&mut self) -> bool {
        if let Some(ref current_piece) = self.current_piece {
            let moved_piece = current_piece.with_left_move();
            if self.board.can_place(&moved_piece) {
                self.current_piece = Some(moved_piece);
                self.last_successful_movement = Instant::now();
                self.try_reset_lock_delay();
                return true;
            }
        }
        false
    }
    
    /// Move the current piece right if possible
    pub fn move_right(&mut self) -> bool {
        if let Some(ref current_piece) = self.current_piece {
            let moved_piece = current_piece.with_right_move();
            if self.board.can_place(&moved_piece) {
                self.current_piece = Some(moved_piece);
                self.last_successful_movement = Instant::now();
                self.try_reset_lock_delay();
                return true;
            }
        }
        false
    }
    
    /// Move the current piece down if possible, lock if not
    pub fn move_down(&mut self) -> bool {
        if let Some(ref current_piece) = self.current_piece {
            let moved_piece = current_piece.with_down_move();
            if self.board.can_place(&moved_piece) {
                self.score_system.add_soft_drop_score(1);
                self.current_piece = Some(moved_piece);
                self.last_successful_movement = Instant::now();
                return true;
            } else if !self.lock_delay_active {
                // Start lock delay
                self.lock_delay_active = true;
                self.lock_delay_timer = Duration::ZERO;
                self.lock_delay_resets = 0;
            }
        }
        false
    }
    
    /// Rotate the current piece clockwise if possible
    pub fn rotate_clockwise(&mut self) -> bool {
        if let Some(ref current_piece) = self.current_piece {
            if let Some(rotated_piece) = RotationSystem::rotate_clockwise(current_piece, &self.board) {
                self.current_piece = Some(rotated_piece);
                self.last_successful_movement = Instant::now();
                self.try_reset_lock_delay();
                return true;
            }
        }
        false
    }
    
    /// Rotate the current piece counter-clockwise if possible
    pub fn rotate_counterclockwise(&mut self) -> bool {
        if let Some(ref current_piece) = self.current_piece {
            if let Some(rotated_piece) = RotationSystem::rotate_counterclockwise(current_piece, &self.board) {
                self.current_piece = Some(rotated_piece);
                self.last_successful_movement = Instant::now();
                self.try_reset_lock_delay();
                return true;
            }
        }
        false
    }
    
    /// Perform a hard drop, instantly placing the piece at the lowest possible position
    pub fn hard_drop(&mut self) -> bool {
        if let Some(mut piece) = self.current_piece.take() {
            let mut drop_distance = 0;
            
            // Move down until collision
            loop {
                let moved_piece = piece.with_down_move();
                if !self.board.can_place(&moved_piece) {
                    break;
                }
                piece = moved_piece;
                drop_distance += 1;
            }
            
            // Add score for the drop
            self.score_system.add_hard_drop_score(drop_distance);
            
            // Place the piece
            self.current_piece = Some(piece);
            self.lock_piece();
            return true;
        }
        false
    }
    
    /// Hold the current piece and replace with next or held piece
    pub fn hold_piece(&mut self) -> bool {
        if !self.can_hold {
            return false;
        }
        
        if let Some(current_piece) = self.current_piece.take() {
            let current_type = current_piece.piece_type;
            
            // If we already have a held piece, swap them
            if let Some(held_type) = self.held_piece {
                let col = (BOARD_WIDTH as i32 / 2) - 1;
                let row = match held_type {
                    PieceType::I => -1,
                    _ => 0,
                };
                self.current_piece = Some(Piece::new(held_type, row, col));
            } else {
                // Otherwise, spawn a new piece
                self.spawn_new_piece();
            }
            
            // Update the held piece
            self.held_piece = Some(current_type);
            self.can_hold = false;
            return true;
        }
        
        false
    }
    
    /// Detect T-spins based on the T piece position and the corners
    fn detect_tspin(&self) -> TSpinType {
        if let Some(ref piece) = self.current_piece {
            if piece.piece_type == PieceType::T {
                // Get the 4 corners around the T piece center
                let (row, col) = (piece.row as usize, piece.col as usize);
                let corners = [
                    (row - 1, col - 1), // Top-left
                    (row - 1, col + 1), // Top-right
                    (row + 1, col - 1), // Bottom-left
                    (row + 1, col + 1), // Bottom-right
                ];
                
                // Count filled corners
                let mut filled_corners = 0;
                for &(r, c) in &corners {
                    if r < BOARD_HEIGHT && c < BOARD_WIDTH {
                        if let Some(cell) = self.board.get_cell(r, c) {
                            if *cell != super::board::Cell::Empty {
                                filled_corners += 1;
                            }
                        } else {
                            // Out of bounds is considered filled
                            filled_corners += 1;
                        }
                    } else {
                        // Out of bounds is considered filled
                        filled_corners += 1;
                    }
                }
                
                // Detect T-spin types
                if filled_corners >= 3 {
                    // Check the front corners based on rotation to determine mini vs full T-spin
                    match piece.rotation {
                        super::piece::Rotation::North => {
                            let front_corners_filled = 
                                (self.is_cell_filled(row + 1, col - 1) as u8) +
                                (self.is_cell_filled(row + 1, col + 1) as u8);
                            if front_corners_filled >= 1 {
                                return TSpinType::Full;
                            } else {
                                return TSpinType::Mini;
                            }
                        },
                        super::piece::Rotation::East => {
                            let front_corners_filled = 
                                (self.is_cell_filled(row - 1, col - 1) as u8) +
                                (self.is_cell_filled(row + 1, col - 1) as u8);
                            if front_corners_filled >= 1 {
                                return TSpinType::Full;
                            } else {
                                return TSpinType::Mini;
                            }
                        },
                        super::piece::Rotation::South => {
                            let front_corners_filled = 
                                (self.is_cell_filled(row - 1, col - 1) as u8) +
                                (self.is_cell_filled(row - 1, col + 1) as u8);
                            if front_corners_filled >= 1 {
                                return TSpinType::Full;
                            } else {
                                return TSpinType::Mini;
                            }
                        },
                        super::piece::Rotation::West => {
                            let front_corners_filled = 
                                (self.is_cell_filled(row - 1, col + 1) as u8) +
                                (self.is_cell_filled(row + 1, col + 1) as u8);
                            if front_corners_filled >= 1 {
                                return TSpinType::Full;
                            } else {
                                return TSpinType::Mini;
                            }
                        }
                    }
                }
            }
        }
        TSpinType::None
    }
    
    // Helper function to check if a cell is filled or out of bounds
    fn is_cell_filled(&self, row: usize, col: usize) -> bool {
        if row >= BOARD_HEIGHT || col >= BOARD_WIDTH {
            return true; // Out of bounds is considered filled
        }
        match self.board.get_cell(row, col) {
            Some(cell) if *cell != super::board::Cell::Empty => true,
            _ => false
        }
    }
    
    /// Lock the current piece in place and handle line clears
    fn lock_piece(&mut self) {
        if let Some(piece) = self.current_piece.take() {
            // Check for T-spin before placing the piece
            let tspin_type = self.detect_tspin();
            
            // Lock the piece on the board
            self.board.place_piece(&piece);
            
            // Clear completed lines
            let lines_cleared = self.board.clear_lines();
            
            // Check for perfect clear after lines are cleared
            let is_perfect_clear = lines_cleared > 0 && self.board.is_perfect_clear();
            
            // Add score based on the clear type (include t-spin bonus)
            self.score_system.add_score_for_lines_with_tspin(lines_cleared, tspin_type);
            
            // Add perfect clear bonus if achieved
            if is_perfect_clear {
                self.score_system.add_perfect_clear_bonus(lines_cleared);
            }
            
            // Update gravity based on level
            self.gravity_delay = Self::calculate_gravity_delay(self.score_system.level);
            
            // Allow holding again
            self.can_hold = true;
            
            // Reset lock delay
            self.lock_delay_active = false;
            self.lock_delay_timer = Duration::ZERO;
            
            // Spawn the next piece
            self.spawn_new_piece();
        }
    }
    
    /// Calculate the gravity delay based on the current level
    fn calculate_gravity_delay(level: u32) -> Duration {
        // Modern Tetris gravity formula (simplified)
        let frames = match level {
            1 => 60,  // 1 drop per second
            2 => 48,
            3 => 36,
            4 => 28,
            5 => 22,
            6 => 16,
            7 => 12,
            8 => 8,
            9 => 6,
            10..=12 => 4,
            13..=15 => 3,
            16..=18 => 2,
            19..=28 => 1,
            _ => 1,   // Max speed at level 29+
        };
        
        // Convert frames to milliseconds (assuming 60 FPS)
        Duration::from_millis((frames as u64 * 1000) / 60)
    }
    
    /// Reset the game to its initial state
    pub fn reset(&mut self) {
        self.board.clear();
        self.current_piece = None;
        self.held_piece = None;
        self.can_hold = true;
        self.state = GameState::Playing;
        self.score_system = ScoreSystem::new();
        self.randomizer = Box::new(BagRandomizer::new());
        self.time_since_last_drop = Duration::ZERO;
        self.gravity_delay = Duration::from_millis(1000);
        self.lock_delay_active = false;
        self.lock_delay_timer = Duration::ZERO;
        self.lock_delay_resets = 0;
        self.last_successful_movement = Instant::now();
        
        // Spawn the first piece
        self.spawn_new_piece();
    }
    
    /// Pause or unpause the game
    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            GameState::GameOver => GameState::GameOver, // Can't unpause game over
        };
    }
    
    /// Spawns a new piece at the top of the board
    fn spawn_new_piece(&mut self) {
        let piece_type = self.randomizer.next();
        let col = (BOARD_WIDTH as i32 / 2) - 1; // Center position, slightly to the left

        // Adjust initial row position based on piece type
        let row = match piece_type {
            PieceType::I => -1, // I needs to start higher
            _ => 0,
        };

        let new_piece = Piece::new(piece_type, row, col);
        
        // Check for game over
        if !self.board.can_place(&new_piece) {
            self.state = GameState::GameOver;
            self.current_piece = None;
            return;
        }
        
        self.current_piece = Some(new_piece);
    }
    
    /// Get the upcoming pieces
    pub fn peek_next_pieces(&self, count: usize) -> Vec<PieceType> {
        self.randomizer.peek(count)
    }
}