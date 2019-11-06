use bv::BitVec;
use std::collections::VecDeque;

const BOARD_SIZE: u8 = 8+2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Team {
    White,
    Black,
}
impl Team {
    pub fn teams() -> Vec<Team> {
        return vec![Team::White, Team::Black];
    }

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
    pub fn in_a_line_with(&self, other: Pos) -> bool {
        let dr = self.row.max(other.row) - self.row.min(other.row);
        let dc = self.col.max(other.col) - self.col.min(other.col);

        dr == 0 || dc == 0 || dr == dc
    }
    pub fn along_line(&self, other: Pos) -> Vec<Pos> {
        let mut v = Vec::new();
        let mut walk = self.clone();
        while walk != other {
            if walk.col < other.col {
                walk.col += 1;
            }
            if walk.col > other.col {
                walk.col -= 1;
            }
            if walk.row < other.row {
                walk.row += 1;
            }
            if walk.row > other.row {
                walk.row -= 1;
            }
            v.push(walk);
        }
        return v;
    }
    pub fn to_linear(&self, num_cols: u8) -> usize {
       self.row as usize * num_cols as usize + self.col as usize
    }
    fn offset(base: u8, offset: i8) -> u8 {
        if offset < 0 {
            base - (offset.abs() as u8)
        } else{
            base + (offset.abs() as u8)
        }
    }
    pub fn with_offset(&self, dir: (i8, i8), dist: i8) -> Pos {
        Pos { row: Pos::offset(self.row, dist*dir.0), col: Pos::offset(self.col, dist * dir.1)}
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
    pub fn new(board_size: u8) -> DistState {
        DistState {
            left: vec![0; (board_size * board_size) as usize],
            right: vec![0; (board_size * board_size) as usize],
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
    pub fn new(board_size: u8, players: Vec<Player>) -> Board {
        let mut b = BitVec::new_fill(false, (board_size*board_size) as u64);

        for r in 0..board_size {
            for c in 0..board_size {
                if r == 0 || c == 0 || r == board_size-1 || c == board_size-1 {
                    b.set((r * board_size + c) as u64,  true);
                }
            }
        }
        for p in &players {
            b.set(p.pos.to_linear(board_size) as u64, true);
        }
        return Board {
            walls: b,
            players: players,
        };
    }
    pub fn wall_set(&mut self, p: Pos, val: bool) {
        self.walls.set(p.to_linear(BOARD_SIZE) as u64, val);
    }
    pub fn wall_at(&self, p: Pos) -> bool {
        self.walls.get((p.to_linear(BOARD_SIZE)) as u64)
    }
    pub fn pprint(&self) -> String {
        let mut s = String::new();
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                let pos = Pos { row: r, col: c};
                if !self.wall_at(pos) {
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
    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.players.iter()
    }

    const QUEEN_DIRS: [(i8,i8); 8] = [(-1,-1),(-1,0),(-1,1),
                                      ( 0,-1)       ,( 0,1),
                                      ( 1,-1),( 1,0),( 1,1)];

    pub fn with_move(&self, player_ix: usize, pos: Pos, shot: Pos) -> Board {
        let mut board = self.clone();
        board.wall_set(self.players[player_ix].pos, false);
        board.wall_set(pos, true);
        board.wall_set(shot, true);
        board.players[player_ix].pos = pos;
        board
    }

    fn queen_range<'a>(&'a self, from: Pos, blank: Pos) -> impl Iterator<Item = Pos> + 'a {
        Board::QUEEN_DIRS.iter().flat_map(move |dir|
                                   (1..).map(move |dist| from.with_offset(*dir, dist))
                                   .take_while(move |place| !self.wall_at(*place) || *place == blank))
    }

    pub fn successors<'a>(&'a self, team: Team) -> impl Iterator<Item = Board> + 'a {
        self.players.iter().enumerate().filter(move |(_,player)| player.team == team)
            .flat_map(move |(pi, player): (usize, &'a Player)| {
                self.queen_range(player.pos, player.pos).flat_map(move |pos: Pos| {
                    self.queen_range(pos, player.pos).map(move |shot: Pos| {
                        self.with_move(pi, pos, shot)
                    })
                })
            })
    }

    pub fn evaluate(&self, team: Team, dist_state: &mut DistState) -> i64 {
        self.bfs(team, &mut dist_state.next, &mut dist_state.left);
        self.bfs(team.other(), &mut dist_state.next, &mut dist_state.right);
        let mut score = 0;
        let mut is_end = true;
        for (&a,&b) in dist_state.left.iter().zip(dist_state.right.iter()) {
            if a < b {
                score = score + 2;
            }
            if a > b {
                score = score - 2;
            }
            if a == b {
                score = score - 1;
            }
            if a != u8::max_value() && b != u8::max_value() {
                is_end = false;
            }
        }
        if is_end {
            if score >= 0 {
                return i64::max_value();
            }
            return i64::min_value() + 1;
        }
        return score;
    }

    fn bfs(&self, team: Team, next: &mut VecDeque<(Pos, u8)>, distances: &mut Vec<u8>) {
        for i in 0..distances.len() {
            distances[i] = u8::max_value();
        }
        next.clear();
        self.players.iter()
            .filter(|p| p.team == team)
            .map(|p| (p.pos, 0))
            .for_each(|it| next.push_back(it));

        while let Some((pos,depth)) = next.pop_front() {
            for neigh in self.queen_range(pos, pos) {
                let place = &mut distances[neigh.to_linear(BOARD_SIZE)];
                if depth + 1 < *place {
                    *place = depth + 1;
                    next.push_back((neigh, depth+1));
                }
            }
        }
    }
}

