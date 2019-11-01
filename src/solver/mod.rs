pub mod board;

use board::*;

pub struct Amazons {
    board_size: u8,
    boards: Vec<Board>,
    cache: Vec<DistState>,
}


impl Amazons {
    pub fn new_8x8() -> Amazons {
        let board_size = 8 + 2;

        let mut players = Vec::new();
        players.push(Player{ team:Team::White, pos:Pos {row:  3, col:  3} });
        players.push(Player{ team:Team::White, pos:Pos {row:  3, col:  6} });
        players.push(Player{ team:Team::Black, pos:Pos {row:  6, col:  3} });
        players.push(Player{ team:Team::Black, pos:Pos {row:  6, col:  6} });

        let mut board = Board::new(board_size, players);

        Amazons {
            board_size: board_size,
            boards: vec![board],
            cache: vec![DistState::new(board_size)],
        }
    }

    pub fn player_move(&mut self, pos: Pos, mv: Pos, shot: Pos) -> bool {
        let board = self.boards[self.boards.len() - 1].clone();

        if pos == mv || mv == shot || !pos.in_a_line_with(mv) {
            println!("Moves not in a line!");
            return false;
        }
        if !mv.in_a_line_with(shot) {
            println!("Shoot is not in a line!");
            return false;
        }
        if let Some(er) = pos.along_line(mv).iter().find(|&&p| board.wall_at(p)) {
            println!("Already a piece at {:?}", er);
            return false;
        }
        if let Some(er) = mv.along_line(shot).iter().filter(|&&p| p != pos).find(|&&p| board.wall_at(p)) {
            println!("Already a piece at {:?}", er);
            return false;
        }
        if let Some((pi, _)) = board.players.iter().enumerate().find(|(_,play)| play.pos == pos) {
            self.boards.push(board.with_move(pi, mv, shot));
            return true;
        }
        println!("Move not a player");
        return false;
    }
}
