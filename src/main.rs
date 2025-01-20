mod a_star;
mod board;

use a_star::a_star;
use board::Board;

fn main() {
    // Example of an N×N puzzle (5×5 in this case)
    let start = Board::new(vec![
        vec![1, 2, 3, 4],
        vec![13, 5, 11, 6],
        vec![0, 10, 7, 15],
        vec![14, 8, 9, 12],
    ]);

    // Run the A* algorithm
    if let Some(solution) = a_star(start) {
        println!("Solution found in {} moves:", solution.len() - 1);
        for board in solution {
            for row in &board.tiles {
                println!("{:?}", row);
            }
            println!();
        }
    } else {
        println!("No solution found!");
    }
}
