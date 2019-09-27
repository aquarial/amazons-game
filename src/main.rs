use bv::BitVec;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Piece {
    White,
    Black,
}
impl Piece {
    fn other(&self) -> Piece {
        match self {
            Piece::White => Piece::Black,
            Piece::Black => Piece::White,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Player {
    team: Piece,
    r: u8,
    c: u8,
}

#[derive(Clone, Debug)]
struct Board {
    walls: BitVec,
    players: Vec<Player>,
}

#[derive(Clone, Debug)]
struct Move {
    player: Player,
    new_pos: (u8, u8),
    new_shot: (u8, u8),
}

const BOARD_SIZE: u8 = 5;

impl Board {
    fn new() -> Board {
        let mut b = BitVec::new_fill(false, (BOARD_SIZE*BOARD_SIZE) as u64);
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                if r == 0 || c == 0 || r == BOARD_SIZE-1 || c == BOARD_SIZE-1 {
                    b.set((r * BOARD_SIZE + c) as u64,  true);
                }
            }
        }

        let mut players = Vec::new();
        players.push(Player{team:Piece::White, r:2, c:2});
        players.push(Player{team:Piece::Black, r:3, c:3});
        for p in &players {
            b.set((p.r * BOARD_SIZE + p.c) as u64, true);
        }
        return Board {
            walls: b,
            players: players,
        };
    }
    fn wall_set(&mut self, r:u8, c:u8, val: bool) {
        self.walls.set((r * BOARD_SIZE + c) as u64, val);
    }
    fn wall_at(&self, r:u8, c:u8) -> bool {
        self.walls.get((r * BOARD_SIZE + c) as u64)
    }
    fn pprint(&self) -> String {
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
    fn with_move(&self, m: &Move) -> Board {
        let mut new_b = self.clone();
        for p in new_b.players.iter_mut() {
            if m.player == *p {
                p.r = m.new_pos.0;
                p.c = m.new_pos.1;
                break;
            }
        }
        new_b.wall_set(m.player.r, m.player.c, false);
        new_b.wall_set(m.new_pos.0, m.new_pos.1, true);
        new_b.wall_set(m.new_shot.0, m.new_shot.1, true);
        return new_b;
    }
    fn successors(&self, piece: &Piece) -> Vec<Move> {
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

    fn evaluate(&self, piece: &Piece) -> i64 {
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


fn max_move(board: &Board, piece: &Piece, depth: i32) -> (Option<Move>, i64) {
    if depth <= 1 {
        let best = board.successors(&piece).iter().min_by_key(|m| board.with_move(&m).evaluate(&piece.other())).cloned();
        if let Some(v) = best {
            return (Some(v.clone()), -board.with_move(&v).evaluate(&piece.other()));
        } else {
            return (None, i64::min_value());
        }
    }

    let mut best: Option<Move> = None;
    let mut score: i64 = i64::min_value();
    for m in board.successors(&piece){
        let b = board.with_move(&m);

        if let (Some(n), resp_score) = max_move(&b, &piece.other(), depth-1) {
            println!("Make {:?} makes {:?} for \n{}", piece, n, board.with_move(&n).pprint());
            if score > -resp_score {
                score = -resp_score;
                best = Some(m);
            }
        } else {
            best = Some(m);
            score = i64::max_value();
            break;
        }
    }

    return (best, score);
}

fn main() {
    let b0 = Board::new();
    let piece = Piece::White;

    println!("Start\n{}", b0.pprint());
    let m = max_move(&b0, &piece, 80);
    println!("Move {:?}", m);

    if let (Some(m2), _) = m {
        println!("\nEnd\n{}", b0.with_move(&m2).pprint());
    }
}

