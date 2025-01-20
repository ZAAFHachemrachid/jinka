use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Board {
    pub tiles: Vec<Vec<usize>>, // 2D Board representation
    pub size: usize,            // Size of the board (N x N)
}

impl Board {
    // Create a new board
    pub fn new(tiles: Vec<Vec<usize>>) -> Self {
        let size = tiles.len();
        Board { tiles, size }
    }

    // Locate the empty tile (0)
    pub fn find_empty_tile(&self) -> (usize, usize) {
        for (i, row) in self.tiles.iter().enumerate() {
            if let Some(j) = row.iter().position(|&x| x == 0) {
                return (i, j);
            }
        }
        panic!("No empty tile (0) found on the board");
    }

    // Generate all possible moves (UP, DOWN, LEFT, RIGHT)
    pub fn get_neighbors(&self) -> Vec<Board> {
        let (empty_row, empty_col) = self.find_empty_tile();
        let mut neighbors = Vec::new();

        let directions = vec![
            (-1, 0), // UP
            (1, 0),  // DOWN
            (0, -1), // LEFT
            (0, 1),  // RIGHT
        ];

        for (dr, dc) in directions {
            let new_row = (empty_row as isize + dr) as usize;
            let new_col = (empty_col as isize + dc) as usize;

            if new_row < self.size && new_col < self.size {
                let mut new_tiles = self.tiles.clone();
                new_tiles[empty_row][empty_col] = new_tiles[new_row][new_col];
                new_tiles[new_row][new_col] = 0;
                neighbors.push(Board::new(new_tiles));
            }
        }
        neighbors
    }

    // Check if the current state is the goal state
    pub fn is_goal(&self) -> bool {
        let mut expected = 1;
        for i in 0..self.size {
            for j in 0..self.size {
                if i == self.size - 1 && j == self.size - 1 {
                    return self.tiles[i][j] == 0; // Last tile should be empty
                }
                if self.tiles[i][j] != expected {
                    return false;
                }
                expected += 1;
            }
        }
        true
    }
}

// Display the board for debugging
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.tiles {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}
