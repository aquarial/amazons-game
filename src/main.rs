
mod board;

use board::*;


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

const DEBUG_DEPTH: i32 = 2;

fn alpha_to_row(c: char) -> Option<u8> {
    for (i,t) in "12345678".chars().enumerate() {
        if c == t {
            return Some(i as u8);
        }
    }
    for (i,t) in "abcdefgh".chars().enumerate() {
        if c == t {
            return Some(i as u8);
        }
    }
    return None;
}

fn record_move(s: &str) -> Result<Pos, String> {
    let pos: Vec<Option<u8>> = s.chars().map(|c| c).map(alpha_to_row).collect();
    if pos.len() != 2 {
        return Err(format!("Wrong ix for {}", s));
    }
    let r = match pos[0] {
        Some(r0) => r0,
        None => {
            return Err(format!("Wrong row for {}", s));
        }
    };
    let c = match pos[1] {
        Some(c0) => c0,
        None => {
            return Err(format!("Wrong col for {}", s));
        }
    };

    Ok(Pos{row:r, col:c})
}

fn main() {
    let b0 = Board::new();
    let team = Team::White;
    let mut diststate = DistState::new();

    println!("Start\n{}", b0.pprint());
    let b = max_move(&b0, team, DEBUG_DEPTH, &mut diststate);
    if let (Some(b1), b1score) = b {
        println!("Best opening for {} points:", b1score);
        println!("{}", b1.pprint());
        println!();
        let resp = max_move(&b1, team.other(), DEBUG_DEPTH-1, &mut diststate);
        if let (Some(b2), _) = resp {
            println!("Best response:");
            println!("{}", b2.pprint());
        }
    }
}

