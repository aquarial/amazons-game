use bv::BitVec;
use std::collections::HashSet;
use std::collections::VecDeque;

const BOARD_SIZE: u8 = 8+2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    next: VecDeque<(Pos, u8)>,
}
impl DistState {
    pub fn new() -> DistState {
        DistState {
            left: vec![u8::max_value(); (BOARD_SIZE * BOARD_SIZE) as usize],
            right: vec![u8::max_value(); (BOARD_SIZE * BOARD_SIZE) as usize],
            next: VecDeque::new(),
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
    fn queen_range(&self, pos: &Pos) -> Vec<Pos> {
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
                    let place = Pos {row: offset(pos.row, dist*dy), col: offset(pos.col, dist*dx)};
                    if !self.wall_at(&place) {
                        v.push(place);
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
            next.wall_set(&p.pos, false);
            for np in next.queen_range(&p.pos) {
                for ns in next.queen_range(&np) {
                    v.push(Move {
                        player: self.players[pi].clone(),
                        new_pos: np,
                        new_shot: ns,
                    });
                }
            }
            next.wall_set(&p.pos, true);
        }
        return v;
    }

    pub fn evaluate(&self, team: &Team, dist_state: &mut DistState) -> i64 {
        self.bfs(&team, &mut dist_state.next, &mut dist_state.left);
        self.bfs(&team.other(), &mut dist_state.next, &mut dist_state.right);
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
    fn bfs(&self, team: &Team, next: &mut VecDeque<(Pos, u8)>, distances: &mut Vec<u8>) {
        for i in 0..distances.len() {
            distances[i] = 0;
        }
        next.clear();
        self.players.iter()
            .filter(|p| p.team == *team)
            .map(|p| (p.pos, 0))
            .for_each(|it| next.push_back(it));

        while let Some((pos,depth)) = next.pop_front() {
            for neigh in self.queen_range(&pos) {
                let place = &mut distances[neigh.to_linear(BOARD_SIZE)];
                if *place == 0 {
                    *place = depth + 1;
                    next.push_back((neigh, depth+1));
                }
            }
        }
    }
}

