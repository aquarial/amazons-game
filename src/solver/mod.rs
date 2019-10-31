pub mod board;

use board::*;

pub struct Solver {
    width: u8,
    board: Board,
}


impl Solver {
    pub fn new_8x8() -> Solver {
        Solver {
            width: 8 + 2,
            board: Board::new(),
        }
    }
}
