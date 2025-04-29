use std::collections::VecDeque;
use rand::{thread_rng, Rng, seq::SliceRandom};
use super::piece::PieceType;

/// Trait for piece randomizers in Tetris
pub trait Randomizer {
    /// Get the next piece from the randomizer
    fn next(&mut self) -> PieceType;
    
    /// Peek at the next n pieces without consuming them
    fn peek(&self, count: usize) -> Vec<PieceType>;
    
    /// Clone this randomizer (required for Game cloning)
    fn clone_box(&self) -> Box<dyn Randomizer>;
}

/// A randomizer that implements the "7-bag" system used in modern Tetris
/// Ensures all 7 piece types appear before any repeats
pub struct BagRandomizer {
    // Current bag of pieces
    bag: Vec<PieceType>,
    // Queue of pieces that have been generated but not yet consumed
    preview_queue: VecDeque<PieceType>,
}

impl BagRandomizer {
    /// Creates a new 7-bag randomizer
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut randomizer = BagRandomizer {
            bag: vec![],
            preview_queue: VecDeque::new(),
        };
        
        // Generate initial bag
        randomizer.refill_bag(&mut rng);
        
        // Fill preview queue
        for _ in 0..5 {
            if randomizer.bag.is_empty() {
                randomizer.refill_bag(&mut rng);
            }
            
            randomizer.preview_queue.push_back(randomizer.bag.pop().unwrap());
        }
        
        randomizer
    }
    
    /// Refills the internal bag with one of each piece type, randomly ordered
    fn refill_bag(&mut self, rng: &mut impl Rng) {
        self.bag = vec![
            PieceType::I,
            PieceType::O,
            PieceType::T,
            PieceType::S,
            PieceType::Z,
            PieceType::J,
            PieceType::L,
        ];
        self.bag.shuffle(rng);
    }
}

impl Clone for BagRandomizer {
    fn clone(&self) -> Self {
        BagRandomizer {
            bag: self.bag.clone(),
            preview_queue: self.preview_queue.clone(),
        }
    }
}

impl Randomizer for BagRandomizer {
    fn next(&mut self) -> PieceType {
        // Take the next piece from the queue
        let next_piece = self.preview_queue.pop_front().unwrap();
        
        // Get a new piece for the preview
        let mut rng = thread_rng();
        if self.bag.is_empty() {
            self.refill_bag(&mut rng);
        }
        
        // Add a new piece to the back of the queue
        self.preview_queue.push_back(self.bag.pop().unwrap());
        
        next_piece
    }
    
    fn peek(&self, count: usize) -> Vec<PieceType> {
        self.preview_queue.iter()
            .take(count.min(self.preview_queue.len()))
            .cloned()
            .collect()
    }
    
    fn clone_box(&self) -> Box<dyn Randomizer> {
        Box::new(self.clone())
    }
}