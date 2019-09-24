use bv::BitVec;

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
    r: u64,
    c: u64,
}

#[derive(Clone, Debug)]
struct Board {
    size: u64,
    walls: BitVec,
    players: Vec<Player>,
}

impl Board {
    fn new(size: u64) -> Board {
        let full_size = size + 2;
        let mut b = BitVec::new_fill(false, (full_size*full_size).into());
        for r in 0..full_size {
            for c in 0..full_size {
                if r == 0 || c == 0 || r == full_size-1 || c == full_size-1 {
                    b.set(r * full_size + c,  true);
                }
            }
        }

        let mut players = Vec::new();
        players.push(Player{team:Piece::White, r:2, c:2});
        players.push(Player{team:Piece::Black, r:3, c:3});
        for p in &players {
            b.set(p.r * full_size + p.c, true);
        }
        return Board {
            size: full_size,
            walls: b,
            players: players,
        };
    }
    fn wall_set(&mut self, r:u64, c:u64, val: bool) {
        self.walls.set(r * self.size + c, val);
    }
    fn wall_at(&self, r:u64, c:u64) -> bool {
        self.walls.get(r * self.size + c)
    }
    fn pprint(&self) -> String {
        let mut s = String::new();
        for r in 0..self.size {
            for c in 0..self.size {
                if !self.wall_at(r,c) {
                    s.push('.');
                    continue;
                }
                match self.players.iter().find(|p| p.r == r && p.c == c) {
                    Some(p) => {
                        if p.team == Piece::Black {
                            s.push('@');
                        } else {
                            s.push('+');
                        }
                    },
                    None => s.push('#'),
                }
            }
            s.push('\n');
        }
        return s;
    }
    fn queen_range(&self, r: u64, c: u64) -> Vec<(u64, u64)> {
        let mut v = Vec::new();
        for dx in -1 ..= 1 {
            for dy in -1 ..= 1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                fn offset(base: u64, offset: i64) -> u64 {
                    if offset < 0 {
                        base - ((-offset) as u64)
                    } else {
                        base + (offset as u64)
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
    fn successors(&self, piece: &Piece) -> Vec<Board> {
        let mut next = self.clone();
        let mut v = Vec::new();
        for (pi, p) in self.players.clone().iter().filter(|p| p.team == *piece).enumerate() {
            next.wall_set(p.r, p.c, false);
            for (npr, npc) in self.queen_range(p.r, p.c) {
                for (nsr, nsc) in self.queen_range(npr, npc) {
                    let mut new_b = next.clone();
                    new_b.players[pi].r = npr;
                    new_b.players[pi].c = npc;
                    new_b.wall_set(npr, npc, true);
                    new_b.wall_set(nsr, nsc, true);
                    v.push(new_b);
                }
            }
            next.wall_set(p.r, p.c, true);
        }
        return v;
    }
}

fn main() {
    let mut b0 = vec![Board::new(3)];
    let mut piece = Piece::White;

    loop {
        piece = piece.other();
        let next: Vec<Board> = b0.iter_mut().flat_map(|b| b.successors(&piece)).collect();
        println!("{:?} went, Size {}", piece, next.len());
        if next.len() == 0 {
            break;
        } else {
            for i in 0..next.len() {
                println!("{:?}\n{}\n{:?}\n", next[i].players,next[i].pprint(), next[i]);
            }
        }
        b0 = next;
        break;
    }
    println!("{} situations, {:?} went last", b0.len(), piece);
}

