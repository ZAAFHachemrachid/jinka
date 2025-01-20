mod a_star;
mod board;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use a_star::a_star;
use board::Board;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct InputBoard {
    tiles: Vec<Vec<u8>>,
}

#[derive(Serialize)]
struct Solution {
    moves: Vec<Board>,
}


async fn solve_puzzle(input: web::Json<InputBoard>) -> impl Responder {
    let board = Board::new(input.tiles.clone());
    if let Some(solution) = a_star(board) {
        HttpResponse::Ok().json(Solution { moves: solution })
    } else {
        HttpResponse::NotFound().body("No solution found")
    }
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>N-Puzzle Solver</title>
        </head>
        <body>
            <h1>N-Puzzle Solver</h1>
            <form id="puzzle-form">
                <label for="tiles">Enter the board configuration (as a JSON array of arrays):</label><br>
                <textarea id="tiles" name="tiles" rows="10" cols="50"></textarea><br>
                <button type="submit">Solve</button>
            </form>
            <div id="solution"></div>
            <script>
                document.getElementById('puzzle-form').addEventListener('submit', function(event) {
                    event.preventDefault();
                    const tiles = document.getElementById('tiles').value;
                    fetch('/solve', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                        },
                        body: JSON.stringify({ tiles: JSON.parse(tiles) }),
                    })
                    .then(response => response.json())
                    .then(data => {
                        let solutionDiv = document.getElementById('solution');
                        solutionDiv.innerHTML = '';
                        if (data.moves) {
                            data.moves.forEach(board => {
                                let boardDiv = document.createElement('div');
                                board.tiles.forEach(row => {
                                    boardDiv.innerHTML += `<p>${JSON.stringify(row)}</p>`;
                                });
                                solutionDiv.appendChild(boardDiv);
                            });
                        } else {
                            solutionDiv.innerHTML = '<p>No solution found.</p>';
                        }
                    })
                    .catch(error => console.error('Error:', error));
                });
            </script>
        </body>
        </html>
        "#,
    )
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/solve", web::post().to(solve_puzzle))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
