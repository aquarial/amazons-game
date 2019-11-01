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

    pub fn undo_2_move(&mut self) {
        if self.boards.len() >= 3 {
            self.boards.pop();
            self.boards.pop();
        }
    }

    pub fn player_move(&mut self, team: Team, pos: Pos, mv: Pos, shot: Pos) -> bool {
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
            println!("Can't move through piece at {:?}", er);
            return false;
        }
        if let Some(er) = mv.along_line(shot).iter().filter(|&&p| p != pos).find(|&&p| board.wall_at(p)) {
            println!("Can't place token through piece at {:?}", er);
            return false;
        }
        if let Some((pi, _)) = board.players.iter().enumerate().find(|(_,play)| play.pos == pos) {
            if board.players[pi].team == team {
                self.boards.push(board.with_move(pi, mv, shot));
                return true;
            }
        }
        println!("You don't have a piece at the position");
        return false;
    }

    pub fn ai_move(&mut self, team: Team) -> bool {
        // TODO Multi-threading based on # of caches
        let c0 = &mut self.cache[0];
        let board = self.boards[self.boards.len() - 1].clone();
        let mut s = Solver { board_size: self.board_size, board: board};
        return match s.max_move(team, 1, c0) {
            (Some(b), _) => {
                self.boards.push(b);
                true
            }
            (None, _) => {
                false
            }
        }
    }

    pub fn curr_board(&self) -> &Board {
        return &self.boards[self.boards.len() - 1];
    }
}


struct Solver {
    board_size: u8,
    board: Board,
}

impl Solver {
    fn max_move(&mut self, team: Team, depth: i32, cache: &mut DistState) -> (Option<Board>, i64) {
        let best = self.board.successors(team)
            .map(|b| (b.evaluate(team, cache), b))
            .max_by_key(|it| it.0);
        if let Some((score, board)) = best {
            return (Some(board), score);
        } else {
            return (None, i64::min_value() + 1);
        }
    }
}
