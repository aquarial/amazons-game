
mod board;

use board::*;


fn max_move(board: &Board, team: &Team, depth: i32, dist_state: &mut DistState) -> (Option<Move>, i64) {
    if depth <= 1 {
        let best = board.successors(&team).iter()
            .min_by_key(|m| board.with_move(&m).evaluate(&team.other(), dist_state))
            .cloned();
        if let Some(v) = best {
            return (Some(v.clone()), -board.with_move(&v).evaluate(&team.other(), dist_state));
        } else {
            return (None, i64::min_value());
        }
    }

    let mut best: Option<Move> = None;
    let mut score: i64 = i64::min_value();
    for m in board.successors(&team){
        let b = board.with_move(&m);
        if score == i64::min_value() && b.evaluate(team, dist_state) < 0 {
            continue;
        }

        if let (Some(n), resp_score) = max_move(&b, &team.other(), depth-1, dist_state) {
            if depth == DEBUG_DEPTH {
                println!("Best response for {:?} after \n{} is \n{}", team.other(), b.pprint(), b.with_move(&n).pprint());
            }
            if score < -resp_score {
                score = -resp_score;
                best = Some(m);
            }
        } else {
            if depth == DEBUG_DEPTH {
                println!("Best response for {:?} after \n{} is \ngive up\n\n", team.other(), b.pprint());
            }
            best = Some(m);
            score = i64::max_value();
        }
    }

    return (best, score);
}

const DEBUG_DEPTH: i32 = 2;

fn main() {
    let b0 = Board::new();
    let team = Team::White;
    let mut diststate = DistState::new();

    println!("Start\n{}", b0.pprint());
    let m = max_move(&b0, &team, DEBUG_DEPTH, &mut diststate);
    println!("Move {:?}", m);

    if let (Some(m2), _) = m {
        println!("\nEnd\n{}", b0.with_move(&m2).pprint());
    }
}

