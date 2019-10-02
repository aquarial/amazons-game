
mod board;

use board::*;
use std::collections::HashMap;
use std::io;


fn max_move(board: &Board, team: Team, depth: i32, dist_state: &mut DistState) -> (Option<Board>, i64) {
    if depth <= 1 {
        let best = board.successors(team)
            .map(|b| (b.evaluate(team, dist_state), b))
            .max_by_key(|it| it.0)
            .map(|it| (it.0, it.1.clone()));
        if let Some((score, board)) = best {
            return (Some(board), score);
        } else {
            return (None, i64::min_value());
        }
    }

    let starting_val = board.evaluate(team, dist_state);

    let mut best: Option<Board> = None;
    let mut score: i64 = i64::min_value();
    for b in board.successors(team){
        if score != i64::min_value() && b.evaluate(team, dist_state) < starting_val {
            continue;
        }

        let (option_resp, resp_score) = max_move(&b, team.other(), depth-1, dist_state);

        if depth == DEBUG_DEPTH {
            let mut s = "game over".to_string();
            if let Some(n) = option_resp {
                s = n.pprint();
            }
            println!("{:?} went \n{}  \n{:?} got {} with \n{}\n\n", team, b.pprint(), team.other(), resp_score, s);
        }

        if score < -resp_score {
            score = -resp_score;
            best = Some(b);
        }
    }

    return (best, score);
}

const DEBUG_DEPTH: i32 = 9000;

fn parse_num(c: char) -> Option<u8> {
    for (i,t) in "12345678".chars().enumerate() {
        if c == t {
            return Some((i+1) as u8);
        }
    }
    for (i,t) in "abcdefgh".chars().enumerate() {
        if c == t {
            return Some((i+1) as u8);
        }
    }
    return None;
}

fn parse_pos(s: &str) -> Option<Pos> {
    let pos: Vec<u8> = s.chars().map(parse_num).filter_map(|i| i).collect();
    if pos.len() == 2 {
        Some(Pos{row:pos[0], col:pos[1]})
    } else {
        None
    }
}

fn parse_move(s: &str) -> Option<(Pos,Pos,Pos)> {
    let vec: Vec<Pos> = s.to_lowercase().split(" ").map(parse_pos).filter_map(|i| i).collect();
    if vec.len() == 3 {
        Some((vec[0], vec[1], vec[2]))
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Player {
    Ai,
    Human,
}

fn main() {

    let depth = 2;

    let mut input: HashMap<Team, Player> = HashMap::new();

    for t in Team::teams() {
        while input.get(&t) == None {
            println!("{:?} is controlled by? [human, ai]", t);
            let mut line = String::new();
            io::stdin().read_line(&mut line);
            if line.trim() == "ai" {
                input.insert(t, Player::Ai);
            }
            if line.trim() == "human" {
                input.insert(t, Player::Human);
            }
        }
    }

    let mut board = Board::new();
    let mut team = Team::White;
    let mut diststate = DistState::new();
    loop {
        let player = input[&team];
        println!("{:?} to go, controlled by {:?}", team, player);
        println!("{}", board.pprint());

        match player {
            Player::Ai => {
                let next = max_move(&board, team, depth, &mut diststate);
                if let (Some(b), _) = next {
                    board = b;
                    team = team.other();
                } else {
                    println!("Ai for team {:?} surrenders!", team);
                    break;
                }
            },
            Player::Human => {
                if let (None, _) =  max_move(&board, team, 1, &mut diststate) {
                    println!("Player for team {:?} has no moves and loses!", team);
                    break;
                }
                let mut line = String::new();
                loop {
                    println!("Choose move for team {:?} in format 'RowCol RowCol RowCol'", team);
                    io::stdin().read_line(&mut line);

                    if let Some((p,m,s)) = parse_move(&line) {
                        println!("Parsed coords: {:?} {:?} {:?}", p, m, s);
                        if let Some(b) = board.with_move_checked(p,m,s) {
                            board = b;
                            team = team.other();
                            break;
                        }
                    }
                }
            }
        }
    }
}

