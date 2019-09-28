use bv::BitVec;
use std::collections::HashSet;
use std::collections::VecDeque;

const BOARD_SIZE: u8 = 8+2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Team {
    White,
    Black,
}
impl Team {
    pub fn other(&self) -> Team {
        match self {
            Team::White => Team::Black,
            Team::Black => Team::White,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pos {
    pub row: u8,
    pub col: u8,
}
impl Pos {
    pub fn to_linear(&self, num_cols: u8) -> usize {
       self.row as usize * num_cols as usize + self.col as usize
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Player {
    pub team: Team,
    pub pos: Pos,
}

#[derive(Clone, Debug)]
pub struct Move {
    pub player: Player,
    pub new_pos: Pos,
    pub new_shot: Pos,
}

#[derive(Clone, Debug)]
pub struct DistState {
    left: Vec<u8>,
    right: Vec<u8>,
}
impl DistState {
    pub fn new() -> DistState {
        DistState {
            left: vec![u8::max_value(); (BOARD_SIZE * BOARD_SIZE) as usize],
            right: vec![u8::max_value(); (BOARD_SIZE * BOARD_SIZE) as usize],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    walls: BitVec,
    players: Vec<Player>,
}

impl Board {
    pub fn new() -> Board {
        let mut b = BitVec::new_fill(false, (BOARD_SIZE*BOARD_SIZE) as u64);
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                if r == 0 || c == 0 || r == BOARD_SIZE-1 || c == BOARD_SIZE-1 {
                    b.set((r * BOARD_SIZE + c) as u64,  true);
                }
            }
        }

        let mut players = Vec::new();
        players.push(Player{ team:Team::White, pos:Pos {row: 3, col: 3} });
        players.push(Player{ team:Team::White, pos:Pos {row: 3, col: 6} });
        players.push(Player{ team:Team::Black, pos:Pos {row: 6, col: 3} });
        players.push(Player{ team:Team::Black, pos:Pos {row: 6, col: 6} });
        for p in &players {
            b.set(p.pos.to_linear(BOARD_SIZE) as u64, true);
        }
        return Board {
            walls: b,
            players: players,
        };
    }
    pub fn wall_set(&mut self, p: &Pos, val: bool) {
        self.walls.set(p.to_linear(BOARD_SIZE) as u64, val);
    }
    pub fn wall_at(&self, p: &Pos) -> bool {
        self.walls.get((p.to_linear(BOARD_SIZE)) as u64)
    }
    pub fn pprint(&self) -> String {
        let mut s = String::new();
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                let pos = Pos { row: r, col: c};
                if !self.wall_at(&pos) {
                    s.push('.');
                    continue;
                }
                match self.players.iter().find(|p| p.pos == pos) {
                    Some(p) => {
                        if p.team == Team::Black {
                            s.push('B');
                        } else {
                            s.push('W');
                        }
                    },
                    None => s.push('#'),
                }
            }
            s.push('\n');
        }
        return s;
    }
    fn queen_range(&self, r: u8, c: u8) -> Vec<(u8, u8)> {
        let mut v = Vec::new();
        for dx in -1 ..= 1 {
            for dy in -1 ..= 1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                fn offset(base: u8, offset: i8) -> u8 {
                    if offset < 0 {
                        base - (offset.abs() as u8)
                    } else {
                        base + offset as u8
                    }
                }
                for dist in 1 .. {
                    if !self.wall_at(offset(r, dy*dist), offset(c, dx*dist)) {
                        v.push((offset(r, dy*dist), offset(c, dx*dist)));
                    } else {
                        break;
                    }
                }
            }
        }
        return v;
    }
    pub fn with_move(&self, m: &Move) -> Board {
        let mut board = self.clone();
        for p in board.players.iter_mut() {
            if m.player == *p {
                p.pos = m.new_pos.clone();
                break;
            }
        }
        board.wall_set(&m.player.pos, false);
        board.wall_set(&m.new_pos, true);
        board.wall_set(&m.new_shot, true);
        return board;
    }

    pub fn successors(&self, team: &Team) -> Vec<Move> {
        let mut next = self.clone();
        let mut v = Vec::new();
        for (pi, p) in self.players.iter().enumerate() {
            if p.team != *team {
                continue;
            }
            next.wall_set(p.r, p.c, false);
            for (npr, npc) in next.queen_range(p.r, p.c) {
                for (nsr, nsc) in next.queen_range(npr, npc) {
                    v.push(Move {
                        player: self.players[pi].clone(),
                        new_pos: (npr, npc),
                        new_shot: (nsr, nsc),
                    });
                }
            }
            next.wall_set(p.r, p.c, true);
        }
        return v;
    }

    pub fn evaluate(&self, piece: &Team, dist_state: &mut DistState) -> i64 {
        self.bfs(&piece, &mut dist_state.left);
        self.bfs(&piece.other(), &mut dist_state.right);
        let mut score = 0;
        for (a,b) in dist_state.left.iter().zip(dist_state.right.iter()) {
            if a < b {
                score = score + 1;
            }
            if a > b {
                score = score - 1;
            }
        }
        return score;
    }
    fn bfs(&self, piece: &Team, distances: &mut Vec<u8>) {
        for i in 0..distances.len() {
            distances[i] = 0;
        }
        struct Loc {
            pos: Pos,
            depth: u8,
        }
        let mut vecdeq: VecDeque<Loc> = self.players.iter()
            .filter(|p| p.team == *piece)
            .map(|p| Loc { pos: p.pos.clone(), depth: 0}).collect();

        while let Some(curr) = vecdeq.pop_front() {
            for next in self.queen_range(&curr.pos) {
                let place = &mut distances[next.to_linear(BOARD_SIZE)];
                if *place == 0 {
                    *place = curr.depth + 1;
                    vecdeq.push_back(Loc { pos: next, depth: curr.depth+1});
                }
            }
        }
    }
}

