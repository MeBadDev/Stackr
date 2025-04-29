# Stackr: A Modern Tetris Engine in Rust

Stackr is a feature-rich Tetris implementation written in Rust, focusing on modern gameplay mechanics, extensibility, and AI integration.

## Features

- **Modern Tetris Mechanics**:
  - SRS (Super Rotation System) for piece rotations
  - 7-bag randomizer ensuring fair piece distribution
  - Hold piece functionality
  - Ghost piece display (visual aid for piece placement)
  - Lock delay (pieces don't immediately lock when landing)

- **Scoring System**:
  - Classic line clear scoring (Singles, Doubles, Triples, Tetris)
  - T-spin detection and bonus scoring
  - Perfect Clear detection (all blocks cleared from the board)
  - Combo system for consecutive line clears

- **AI Bot Integration**:
  - Built-in AI bot that can play the game automatically
  - Board evaluation based on multiple heuristics
  - Move selection using simulation and optimization

## Getting Started

### Prerequisites

- Rust (1.54.0 or higher)
- Cargo (comes with Rust :))

### Building

```bash
# Clone the repository
git clone https://github.com/yourusername/stackr.git
cd stackr

# Build the project
cargo build

# Run the demo
cargo run
```

## Demo Features

The demo program showcases various features of the engine:

1. Basic piece movement and rotation
2. Hard drop functionality
3. Hold piece mechanism
4. Perfect Clear demonstration
5. AI bot gameplay

## Bot Gameplay

The AI bot demonstrates intelligent Tetris gameplay using:

- **Board Evaluation**: The bot evaluates board states based on several metrics:
  - Aggregate height (total height of all columns)
  - Completed lines
  - Holes (empty cells with filled cells above)
  - Bumpiness (difference in height between adjacent columns)
  - Well formations (columns much lower than their neighbors)

- **Move Finding**: The bot considers all possible positions and rotations for each piece to find the optimal move.

- **Look Ahead**: The bot can evaluate multiple future pieces to plan ahead (configurable).

## Customization

You can customize the bot's behavior by adjusting the weights in `bot/evaluator.rs`:

```rust
// Example of adjusting evaluation weights for different play styles
let aggressive_weights = EvaluationWeights {
    aggregate_height_weight: -0.310066,
    complete_lines_weight: 0.960666,
    holes_weight: -0.75663,
    bumpiness_weight: -0.184483,
    landing_height_weight: -0.0,
    well_weight: 0.3,
};

let bot = TetrisBot::with_weights(aggressive_weights);
```

## Future Improvements

- Graphical user interface
- Advanced bot strategies
- Performance optimizations
- Modern Tetris platform (TETR.IO for example) integration

## License

This project is licensed under the [GPL-3.0 License](./LICENSE).
You're free to use it, but please credit me and consider contributing your improvements!

## Acknowledgments

- The Classic Tetris community for inspiration
- Modern Tetris guidelines for gameplay mechanics
- Various Tetris AI research papers for bot implementation strategies