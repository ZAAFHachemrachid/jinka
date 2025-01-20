use std::collections::HashSet;

// Represents the N×N board state
#[derive(Clone, Eq, PartialEq, Hash, Debug, serde::Serialize)]
pub struct Board {
    pub tiles: Vec<Vec<u8>>,      // The board as a 2D vector
    pub zero_pos: (usize, usize), // (row, col) position of the empty tile (0)
    pub size: usize,              // The size of the board (N)
}

impl Board {
    // Create a new board
    pub fn new(tiles: Vec<Vec<u8>>) -> Self {
        let size = tiles.len(); // N is determined by the row count
                                // Find the position of the zero tile
        let zero_pos = tiles
            .iter()
            .enumerate()
            .flat_map(|(row, cols)| {
                cols.iter()
                    .enumerate()
                    .map(move |(col, &val)| (row, col, val))
            })
            .find(|&(_, _, val)| val == 0)
            .map(|(row, col, _)| (row, col))
            .unwrap();

        Board {
            tiles,
            zero_pos,
            size,
        }
    }

    // Check if the board is in the goal state
    pub fn is_goal(&self) -> bool {
        let mut goal = vec![vec![0; self.size]; self.size];
        let mut value = 1;
        for row in 0..self.size {
            for col in 0..self.size {
                goal[row][col] = if value == self.size * self.size {
                    0
                } else {
                    value as u8
                };
                value += 1;
            }
        }
        self.tiles == goal
    }

    // Generate neighbors (valid moves)
    pub fn neighbors(&self) -> Vec<(Board, usize)> {
        let mut neighbors = Vec::new();
        let (zero_row, zero_col) = self.zero_pos;

        // Possible moves: UP, DOWN, LEFT, RIGHT
        const MOVES: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for &(dr, dc) in &MOVES {
            let new_row = zero_row as isize + dr;
            let new_col = zero_col as isize + dc;

            if new_row >= 0
                && new_row < self.size as isize
                && new_col >= 0
                && new_col < self.size as isize
            {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                // Create a new board with the swapped tiles
                let mut new_tiles = self.tiles.clone();
                new_tiles[zero_row][zero_col] = new_tiles[new_row][new_col];
                new_tiles[new_row][new_col] = 0;

                neighbors.push((Board::new(new_tiles), 1)); // Cost of 1 per move
            }
        }

        neighbors
    }
}
