mod a_star;
mod board;

use a_star::solve;
use board::Board;

fn main() {
    let initial_board = Board::new(vec![vec![1, 2, 3], vec![4, 0, 5], vec![6, 7, 8]]);

    println!("Initial Board:");
    println!("{}", initial_board);

    match solve(initial_board) {
        Some(solution) => {
            println!("Solution found!");
            for (i, step) in solution.iter().enumerate() {
                println!("Step {}:\n{}", i + 1, step);
            }
        }
        None => {
            println!("No solution exists!");
        }
    }
}
