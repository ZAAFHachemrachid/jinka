use crate::board::Board;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

// A node in the search space
#[derive(Eq, PartialEq)]
struct Node {
    board: Board,
    cost: usize,      // Cost so far
    heuristic: usize, // Heuristic estimate
}

impl Node {
    pub fn new(board: Board, cost: usize, heuristic: usize) -> Self {
        Node {
            board,
            cost,
            heuristic,
        }
    }

    // Priority value: cost + heuristic
    pub fn priority(&self) -> usize {
        self.cost + self.heuristic
    }
}

// Custom ordering for the priority queue
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority().cmp(&self.priority()) // Reverse for min-heap behavior
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Manhattan Distance heuristic
fn manhattan_distance(board: &Board) -> usize {
    let mut distance = 0;
    for i in 0..board.size {
        for j in 0..board.size {
            let value = board.tiles[i][j];
            if value != 0 {
                let target_row = (value - 1) / board.size;
                let target_col = (value - 1) % board.size;
                distance += (i as isize - target_row as isize).abs() as usize
                    + (j as isize - target_col as isize).abs() as usize;
            }
        }
    }
    distance
}

// A* algorithm implementation
pub fn solve(initial: Board) -> Option<Vec<Board>> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();

    // Start with the initial board
    open_set.push(Node::new(initial.clone(), 0, manhattan_distance(&initial)));

    while let Some(current) = open_set.pop() {
        if current.board.is_goal() {
            return Some(vec![current.board]); // Solved! Traceback not implemented for simplicity
        }

        if closed_set.contains(&current.board) {
            continue;
        }
        closed_set.insert(current.board.clone());

        for neighbor in current.board.get_neighbors() {
            if !closed_set.contains(&neighbor) {
                let cost = current.cost + 1; // Cost of moving to a neighbor
                let heuristic = manhattan_distance(&neighbor);
                open_set.push(Node::new(neighbor, cost, heuristic));
            }
        }
    }

    None // No solution found
}
