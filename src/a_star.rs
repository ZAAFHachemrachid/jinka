use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::board::Board;

// Define a state in the A* search
#[derive(Eq, PartialEq)]
struct State {
    pub board: Board,
    pub cost: usize,   // g(n) + h(n)
    pub g_cost: usize, // g(n)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse for min-heap behavior
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Manhattan distance heuristic
fn manhattan_distance(board: &Board) -> usize {
    let mut distance = 0;

    for (row, tiles_row) in board.tiles.iter().enumerate() {
        for (col, &tile) in tiles_row.iter().enumerate() {
            if tile != 0 {
                let target_row = (tile as usize - 1) / board.size;
                let target_col = (tile as usize - 1) % board.size;
                distance += (target_row as isize - row as isize).abs() as usize
                    + (target_col as isize - col as isize).abs() as usize;
            }
        }
    }

    distance
}

// A* search algorithm
pub fn a_star(start: Board) -> Option<Vec<Board>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Board, Board> = HashMap::new();
    let mut g_score: HashMap<Board, usize> = HashMap::new();

    open_set.push(State {
        board: start.clone(),
        cost: manhattan_distance(&start),
        g_cost: 0,
    });

    g_score.insert(start.clone(), 0);

    while let Some(current_state) = open_set.pop() {
        let current_board = current_state.board;

        // Check if goal is reached
        if current_board.is_goal() {
            let mut path = Vec::new();
            let mut current = current_board;
            while let Some(prev) = came_from.get(&current) {
                path.push(current);
                current = prev.clone();
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        // Explore neighbors
        for (neighbor, move_cost) in current_board.neighbors() {
            let tentative_g_score = g_score.get(&current_board).unwrap() + move_cost;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor.clone(), current_board.clone());
                g_score.insert(neighbor.clone(), tentative_g_score);
                open_set.push(State {
                    board: neighbor.clone(),
                    cost: tentative_g_score + manhattan_distance(&neighbor),
                    g_cost: tentative_g_score,
                });
            }
        }
    }

    None
}
