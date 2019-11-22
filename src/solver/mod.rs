pub mod board;

use board::*;
use smallvec::SmallVec;

pub struct Amazons {
    board_size: i8,
    boards: Vec<Board>,
    cache: DistState,
}


impl Amazons {
    pub fn new_8x8() -> Amazons {
        let board_size = 8 + 2;

        let mut players = Vec::new();
        players.push(Player{ team:Team::White, pos:Pos {row:  3, col:  3} });
        players.push(Player{ team:Team::White, pos:Pos {row:  3, col:  6} });
        players.push(Player{ team:Team::Black, pos:Pos {row:  6, col:  3} });
        players.push(Player{ team:Team::Black, pos:Pos {row:  6, col:  6} });

        Amazons {
            board_size: board_size,
            boards: vec![Board::new(board_size, players)],
            cache: DistState::with_board_size(board_size),
        }
    }

    pub fn undo_2_move(&mut self) {
        if self.boards.len() >= 3 {
            self.boards.pop();
            self.boards.pop();
        }
    }

    pub fn team_pieces(&self, team: Team) -> Vec<Pos> {
        self.boards[self.boards.len() - 1].players()
            .filter(|p| p.team == team)
            .map(|p| p.pos)
            .collect()
    }

    pub fn player_move(&mut self, team: Team, pos: Pos, mv: Pos, shot: Pos) -> bool {
        let board = self.boards[self.boards.len() - 1].clone();

        for coord in vec![pos, mv, shot] {
            if coord.row >= self.board_size || coord.col >= self.board_size {
                println!("Coord {:?} is outside board_size ({}, {})", coord,
                         self.board_size, self.board_size);
            }
        }
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
        if let Some((pi, p)) = board.players().enumerate().find(|(_,play)| play.pos == pos) {
            if p.team == team {
                self.boards.push(board.with_move(pi, mv, shot));
                return true;
            }
        }
        println!("You don't have a piece at the position");
        return false;
    }

    pub fn ai_move(&mut self, team: Team, strategy: EvalStrategy) -> bool {
        // TODO Multi-threading based on # of caches
        let c0 = &mut self.cache;
        let board = &self.boards[self.boards.len() - 1];
        return match max_move(&board, team, strategy, 4, c0) {
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

fn max_move(board: &Board, team: Team, strategy: EvalStrategy, depth: i32, cache: &mut DistState) -> (Option<Board>, i64) {
    if depth <= 1 {
        let best = board.successors(team)
            .map(|b| (b.evaluate(team, strategy, cache), b))
            .max_by_key(|it| it.0);
        if let Some((score, board)) = best {
            return (Some(board), score);
        } else {
            return (None, i64::min_value() + 1);
        }
    }

    let mut best: Option<Board> = None;
    let mut score: i64 = i64::min_value() + 1;

    for (_, b) in top_n(board.successors(team).map(|i| (i.evaluate(team, strategy, cache), i))) {
        //if score != i64::min_value() && b.evaluate(team, dist_state) < starting_val - 1 {
        //    // can't do this in the end-game!
        //    //continue;
        //}

        let (_, resp_score) = max_move(&b, team.other(), strategy, depth-1, cache);

        if score < -resp_score {
            score = -resp_score;
            best = Some(b);
        }
    }

    return (best, score);
}

fn top_n(iter: impl Iterator<Item = (i64, Board)>) -> SmallVec<[(i64, Board); 20]> {
    let mut vec = SmallVec::<[(i64, Board); 20]>::new(); // = Vec::with_capacity(count + 1);

    iter.for_each(|new| {
        match vec.binary_search_by_key(& -new.0, |a| -a.0) {
            Ok(i) => vec.insert(i, new),
            Err(i) => vec.insert(i, new),
        }
        vec.truncate(18)
    });

    return vec;
}

