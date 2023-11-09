//! Chomp Game
//! Adam Hiatt
use std::process;

use chomp::{Board, MAX_COLUMNS, MAX_ROWS};
use prompted::input;

///Takes user input and creates a new game board
/// Returns [Some(Board)] with new board
fn create_new_board() -> Result<chomp::Board, String> {
    let rows;
    let columns;
    let rows_answer = input!("Pick number of rows between 1 and {}:", MAX_ROWS);

    match rows_answer.parse::<u8>() {
        Ok(n) => {
            if (1..=MAX_ROWS).contains(&n) {
                rows = n;
            } else {
                let err = format!("Rows must be between 1 and {}", MAX_ROWS);
                println!("{}", err);
                return Err(err.to_string());
            }
        }
        Err(_) => {
            let err = format!("Rows must be between 1 and {}", MAX_ROWS);
            println!("{}", err);
            return Err(err.to_string());
        }
    }

    let column_answer = input!("Pick number of columns between 1 and {}:", MAX_COLUMNS);
    match column_answer.parse::<u8>() {
        Ok(n) => {
            if (1..=MAX_COLUMNS).contains(&n) {
                columns = n;
            } else {
                let err = format!("Columns must be between 1 and {}", MAX_COLUMNS);
                println!("{}", err);
                return Err(err.to_string());
            }
        }
        Err(_) => {
            let err = format!("Columns must be between 1 and {}", MAX_COLUMNS);
            println!("{}", err);
            return Err(err.to_string());
        }
    }

    Ok(Board::new(columns, rows))
}

///Gets a users next move
/// Returns [Ok(u8, u8)] if the move is valid
fn get_user_selection(board: &Board) -> Result<(u8, u8), String> {
    let row_answer = input!("Pick row number between 1 and {}:", board.rows);
    let row;
    let col;
    match row_answer.parse::<u8>() {
        Ok(n) => {
            if n >= 1 && n <= board.rows {
                row = n;
            } else {
                let err = format!("Row must be between 1 and {}", board.rows);
                println!("{}", err);
                return Err(err.to_string());
            }
        }
        Err(_) => {
            let err = format!("Row must be between 1 and {}", board.rows);
            println!("{}", err);
            return Err(err.to_string());
        }
    }

    let col_answer = input!("Pick column number between 1 and {}:", board.columns);
    match col_answer.parse::<u8>() {
        Ok(n) => {
            if n >= 1 && n <= board.columns {
                col = n;
                println!();
            } else {
                let err = format!("col must be between 1 and {}", board.columns);
                println!("{}", err);
                return Err(err.to_string());
            }
        }
        Err(_) => {
            let err = format!("col must be between 1 and {}", board.columns);
            println!("{}", err);
            return Err(err.to_string());
        }
    }

    Ok((row, col))
}

///Plays a game of Chomp
fn play_game() {
    let mut board;
    let mut user_move;
    loop {
        if let Ok(b) = create_new_board() {
            board = b.clone();
            break;
        }
    }
    board.print();
    loop {
        loop {
            if let Ok(m) = get_user_selection(&board) {
                user_move = m;
            } else {
                continue;
            }
            match board.chomp(user_move.0, user_move.1) {
                Ok(lose) => {
                    if lose {
                        println!("You lose!");
                        process::exit(0)
                    }
                    break;
                }
                Err(x) => {
                    println!("{}\n", x);
                    continue;
                }
            }
        }
        println!("Board after player move: ");
        board.print();
        println!();

        if let Some(winning_move) = board.winning_move() {
            match board.chomp(winning_move.0, winning_move.1) {
                Ok(win) => {
                    if win {
                        println!("You win!");
                        process::exit(0)
                    }
                }
                Err(_) => {
                    panic!("Invalid move by AI");
                }
            }
        } else {
            let smallest_move = board.find_smallest_chomp();
            match smallest_move {
                Some(m) => {
                    if let Ok(win) = board.chomp(m.0, m.1) {
                        if win {
                            println!("You win!");
                            process::exit(0)
                        }
                    }
                }
                None => panic!("Invalid move by AI"),
            }
        }
        println!("Board after AI move: ");
        board.print();
        println!();
    }
}

fn main() {
    play_game();
}
