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
        let ix0 = 3;
        let ix1 = 6;
        players.push(Player{ team:Team::White, pos:Pos {row: ix0, col: ix0} });
        players.push(Player{ team:Team::White, pos:Pos {row: ix0, col: ix1} });
        players.push(Player{ team:Team::Black, pos:Pos {row: ix1, col: ix0} });
        players.push(Player{ team:Team::Black, pos:Pos {row: ix1, col: ix1} });

        let mut board = Board::new(board_size, players);

        Solver {
            board_size: board_size,
            board: board,
            cache: DistState::new(board_size),
        }
    }
}
