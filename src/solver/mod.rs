pub mod board;

use board::*;

pub struct Solver {
    board_size: u8,
    board: Board,
    cache: DistState,
}


impl Solver {
    pub fn new_8x8() -> Solver {
        let board_size = 8 + 2;

        let players = Vec::new();
        players.push(Player{ team:Team::White, pos:Pos {row:  3, col:  3} });
        players.push(Player{ team:Team::White, pos:Pos {row:  3, col:  6} });
        players.push(Player{ team:Team::Black, pos:Pos {row:  6, col:  3} });
        players.push(Player{ team:Team::Black, pos:Pos {row:  6, col:  6} });

        let mut board = Board::new(board_size, players);

        Solver {
            board_size: board_size,
            board: board,
            cache: DistState::new(board_size),
        }
    }
}
