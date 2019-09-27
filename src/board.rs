use bv::BitVec;
use std::collections::HashSet;
use std::collections::VecDeque;

const BOARD_SIZE: u8 = 8+2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Piece {
    White,
    Black,
}
impl Piece {
    pub fn other(&self) -> Piece {
        match self {
            Piece::White => Piece::Black,
            Piece::Black => Piece::White,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Player {
    pub team: Piece,
    pub r: u8,
    pub c: u8,
}

#[derive(Clone, Debug)]
pub struct Move {
    pub player: Player,
    pub new_pos: (u8, u8),
    pub new_shot: (u8, u8),
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
        players.push(Player{team:Piece::White, r:3, c:3});
        players.push(Player{team:Piece::White, r:3, c:6});
        players.push(Player{team:Piece::Black, r:6, c:3});
        players.push(Player{team:Piece::Black, r:6, c:6});
        for p in &players {
            b.set((p.r * BOARD_SIZE + p.c) as u64, true);
        }
        return Board {
            walls: b,
            players: players,
        };
    }
    pub fn wall_set(&mut self, r:u8, c:u8, val: bool) {
        self.walls.set((r * BOARD_SIZE + c) as u64, val);
    }
    pub fn wall_at(&self, r:u8, c:u8) -> bool {
        self.walls.get((r * BOARD_SIZE + c) as u64)
    }
    pub fn pprint(&self) -> String {
        let mut s = String::new();
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                if !self.wall_at(r,c) {
                    s.push('.');
                    continue;
                }
                match self.players.iter().find(|p| p.r == r && p.c == c) {
                    Some(p) => {
                        if p.team == Piece::Black {
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
                p.r = m.new_pos.0;
                p.c = m.new_pos.1;
                break;
            }
        }
        board.wall_set(m.player.r, m.player.c, false);
        board.wall_set(m.new_pos.0, m.new_pos.1, true);
        board.wall_set(m.new_shot.0, m.new_shot.1, true);
        return board;
    }
    pub fn successors(&self, piece: &Piece) -> Vec<Move> {
        let mut next = self.clone();
        let mut v = Vec::new();
        for (pi, p) in self.players.iter().enumerate() {
            if p.team != *piece {
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

    pub fn evaluate(&self, piece: &Piece) -> i64 {
        let us = self.bfs(&piece);
        let them = self.bfs(&piece.other());
        let mut score = 0;
        for (a,b) in us.iter().zip(them.iter()) {
            if a < b {
                score = score + 1;
            }
            if a > b {
                score = score - 1;
            }
        }
        return score;
    }
    fn bfs(&self, piece: &Piece) -> Vec<u8> {
        let mut distances: Vec<u8> = vec![u8::max_value(); BOARD_SIZE as usize * BOARD_SIZE as usize];
        struct Loc {
            row: u8,
            col: u8,
            depth: u8,
        }
        let mut vecdeq: VecDeque<Loc> = self.players.iter()
            .filter(|p| p.team == *piece)
            .map(|p| Loc { row: p.r, col: p.c, depth: 0}).collect();
        let mut visited: HashSet<(u8, u8)> = HashSet::new();

        while let Some(curr) = vecdeq.pop_front() {
            for next in self.queen_range(curr.row, curr.col) {
                if !visited.contains(&next) {
                    let loc = Loc { row: next.0, col: next.1, depth: curr.depth+1};
                    distances[loc.row as usize * BOARD_SIZE as usize + loc.col as usize] = curr.depth + 1;
                    visited.insert((loc.row, loc.col));
                    vecdeq.push_back(loc);
                }
            }
        }
        return distances;
    }
}
