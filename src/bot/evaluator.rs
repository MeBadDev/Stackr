use crate::tetris_core::{Game, Board, Cell, BOARD_WIDTH, BOARD_HEIGHT};

/// Weight configuration for different evaluation metrics
pub struct EvaluationWeights {
    /// Weight for aggregate height of all columns
    pub aggregate_height_weight: f64,
    /// Weight for completed lines
    pub complete_lines_weight: f64,
    /// Weight for number of holes in the board
    pub holes_weight: f64,
    /// Weight for bumpiness (height differences between adjacent columns)
    pub bumpiness_weight: f64,
    /// Weight for landing height of the last piece
    pub landing_height_weight: f64,
    /// Weight for well structures (columns with deep gaps)
    pub well_weight: f64,
}

impl Default for EvaluationWeights {
    fn default() -> Self {
        EvaluationWeights {
            // These weights are based on common Tetris AI heuristics
            // Negative weights penalize bad attributes
            aggregate_height_weight: -0.510066,
            complete_lines_weight: 0.760666,
            holes_weight: -0.35663,
            bumpiness_weight: -0.184483,
            landing_height_weight: -0.0,
            well_weight: 0.3,
        }
    }
}

/// Evaluates the quality of a Tetris board state
pub struct BoardEvaluator {
    weights: EvaluationWeights,
}

impl BoardEvaluator {
    /// Create a new board evaluator with default weights
    pub fn new() -> Self {
        BoardEvaluator {
            weights: EvaluationWeights::default(),
        }
    }

    /// Create a new board evaluator with custom weights
    pub fn with_weights(weights: EvaluationWeights) -> Self {
        BoardEvaluator { weights }
    }

    /// Main evaluation function - scores a game state based on multiple factors
    pub fn evaluate(&self, game: &Game) -> f64 {
        let board = &game.board;
        
        // Calculate various metrics that define the board's "quality"
        let column_heights = self.get_column_heights(board);
        let aggregate_height = column_heights.iter().sum::<u32>() as f64;
        let holes = self.count_holes(board, &column_heights);
        let complete_lines = self.count_complete_lines(board) as f64;
        let bumpiness = self.calculate_bumpiness(&column_heights);
        let wells = self.calculate_wells(&column_heights);
        
        // Apply weights to each metric and get the final score
        (self.weights.aggregate_height_weight * aggregate_height) +
        (self.weights.holes_weight * holes as f64) + 
        (self.weights.complete_lines_weight * complete_lines) +
        (self.weights.bumpiness_weight * bumpiness) + 
        (self.weights.well_weight * wells)
    }

    /// Get the height of each column in the board
    fn get_column_heights(&self, board: &Board) -> Vec<u32> {
        let mut heights = vec![0; BOARD_WIDTH];
        
        for col in 0..BOARD_WIDTH {
            for row in 0..BOARD_HEIGHT {
                if let Some(Cell::Filled(_)) = board.get_cell(row, col) {
                    // Record this column's height from the top
                    heights[col] = (BOARD_HEIGHT - row) as u32;
                    break;
                }
            }
        }
        
        heights
    }

    /// Count the number of holes in the board
    fn count_holes(&self, board: &Board, column_heights: &[u32]) -> u32 {
        let mut holes = 0;
        
        for col in 0..BOARD_WIDTH {
            let col_height = column_heights[col] as usize;
            let top_row = BOARD_HEIGHT - col_height;
            
            // Check for holes below the top block in this column
            for row in top_row + 1..BOARD_HEIGHT {
                if let Some(Cell::Empty) = board.get_cell(row, col) {
                    holes += 1;
                }
            }
        }
        
        holes
    }

    /// Count the number of complete lines in the board
    fn count_complete_lines(&self, board: &Board) -> u32 {
        let mut complete_lines = 0;
        
        for row in 0..BOARD_HEIGHT {
            let mut line_complete = true;
            
            for col in 0..BOARD_WIDTH {
                if let Some(Cell::Empty) = board.get_cell(row, col) {
                    line_complete = false;
                    break;
                }
            }
            
            if line_complete {
                complete_lines += 1;
            }
        }
        
        complete_lines
    }

    /// Calculate the bumpiness (sum of differences between adjacent columns)
    fn calculate_bumpiness(&self, column_heights: &[u32]) -> f64 {
        let mut bumpiness = 0.0;
        
        for i in 0..column_heights.len() - 1 {
            bumpiness += (column_heights[i] as i32 - column_heights[i + 1] as i32).abs() as f64;
        }
        
        bumpiness
    }

    /// Calculate the well factor (deep holes flanked by blocks on both sides)
    fn calculate_wells(&self, column_heights: &[u32]) -> f64 {
        let mut well_sum = 0.0;
        
        // Check each column to see if it's significantly lower than neighbors
        for i in 0..column_heights.len() {
            let current_height = column_heights[i];
            
            let left_height = if i > 0 { column_heights[i - 1] } else { current_height };
            let right_height = if i < column_heights.len() - 1 { column_heights[i + 1] } else { current_height };
            
            if current_height + 3 < left_height && current_height + 3 < right_height {
                // We found a well (deep gap)
                let depth = std::cmp::min(left_height, right_height) - current_height;
                // Wells get increasingly penalized the deeper they are
                well_sum += (depth * depth) as f64;
            }
        }
        
        well_sum
    }
}