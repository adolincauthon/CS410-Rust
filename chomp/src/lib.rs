pub const MAX_COLUMNS: u8 = 5;
pub const MAX_ROWS: u8 = 4;
use std::collections::HashSet;

/// Board representing a chomp Game
/// rows: number of rows in the board
/// columns: number of columns in the board
/// state: the state of a board at any given time
#[derive(Clone)]
pub struct Board {
    pub rows: u8,
    pub columns: u8,
    pub state: HashSet<(u8, u8)>,
}

impl Board {
    pub fn new(columns: u8, rows: u8) -> Self {
        let mut state = HashSet::new();
        for i in 1..=rows {
            for j in 1..=columns {
                state.insert((i, j));
            }
        }
        Board {
            rows,
            columns,
            state,
        }
    }

    ///Prints a graphical representation of the board
    pub fn print(&self) {
        for i in 1u8..=self.rows {
            for j in 1u8..=self.columns {
                if self.state.contains(&(i, j)) {
                    print!("x");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    /// Chomps a piece at the row and column, and consumes all pieces to the
    /// right and below the piece. Returns [Result<bool>] with the bool denoting
    /// if that chomp lost the game `true` or did not lose `false`
    pub fn chomp(&mut self, row: u8, column: u8) -> Result<bool, &'static str> {
        if row > self.rows || column > self.columns {
            return Err("Block does not exist");
        }
        if self.state.contains(&(row, column)) {
            for i in column..=self.columns {
                for j in row..=self.rows {
                    self.state.remove(&(j, i));
                    if j == 1 && i == 1 {
                        return Ok(true);
                    }
                }
            }
            return Ok(false);
        }
        Err("Block has already been chomped")
    }

    /// Determines the next winning move based on minimax algorithm
    /// Returns [Some(u8, u8)] with coordinates of next move or
    /// [None]  if no winning move is found
    pub fn winning_move(&self) -> Option<(u8, u8)> {
        for r in 1..=self.rows {
            for c in 1..=self.columns {
                if !self.state.contains(&(r, c)) || (r == 1 && c == 1) {
                    continue;
                }
                let mut new_board = self.clone();

                match new_board.chomp(r, c) {
                    Ok(_) => (),
                    Err(_) => return None,
                };

                let opponent_move = new_board.winning_move();
                if opponent_move.is_none() {
                    return Some((r, c));
                }
            }
        }
        None
    }

    /// Returns [Some(u8, u8)] containing the coordinates that
    /// consume the least amount of the board or [None] if the board
    /// is empty
    pub fn find_smallest_chomp(&mut self) -> Option<(u8, u8)> {
        for j in (1..=self.rows).rev() {
            for i in (1..=self.columns).rev() {
                if self.state.contains(&(j, i)) {
                    return Some((j, i));
                }
            }
        }
        None
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new(MAX_COLUMNS, MAX_ROWS)
    }
}

#[test]
fn smallest_of_chomps() {
    let mut board = Board::new(5, 4);
    assert_eq!(board.find_smallest_chomp(), Some((4, 5)));
    _ = board.chomp(4, 5);
    board.print();
    assert_eq!(board.find_smallest_chomp(), Some((4, 4)));
    _ = board.chomp(1, 2);
    assert_eq!(board.find_smallest_chomp(), Some((4, 1)));
    _ = board.chomp(3, 1);
    assert_eq!(board.find_smallest_chomp(), Some((2, 1)));
}

#[test]
fn print_new_board_and_chomp() {
    let mut board = Board::new(5, 4);
    println!("Before chomp: ");
    board.print();
    _ = board.chomp(3, 2);
    println!("After chomp: ");
    board.print();
}

#[test]
fn chomp_the_unchompable() {
    let mut board = Board::new(5, 4);
    _ = board.chomp(3, 2);
    match board.chomp(4, 3) {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!("Block has already been chomped", x),
    }

    match board.chomp(5, 3) {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!("Block does not exist", x),
    }
}

#[test]
fn chomp_wars_the_clone_wars() {
    let board = Board::new(5, 4);
    let cloned_board = board.clone();
    for i in 1u8..4 {
        for j in 1u8..5 {
            assert!(board.state.contains(&(i, j)) && cloned_board.state.contains(&(i, j)));
        }
    }
}

#[test]
fn winning_chomper() {
    let mut board = Board::new(5, 4);
    _ = board.chomp(1, 2);
    match board.winning_move() {
        Some(x) => assert_eq!(x, (2, 1)),
        None => assert!(false),
    }
}

#[test]
fn chomp_print() {
    let mut board = Board::new(5, 4);
    _ = board.chomp(1, 2);
    assert_eq!(board.state.contains(&(1, 2)), false);
    board.print();
}
