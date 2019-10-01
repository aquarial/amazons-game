
mod board;

use board::*;


fn max_move(board: &Board, team: &Team, depth: i32, dist_state: &mut DistState) -> (Option<Board>, i64) {
    if depth <= 1 {
        let best = board.successors(&team).iter()
            .map(|b| (b, b.evaluate(&team.other(), dist_state)))
            .min_by_key(|it| it.1)
            .map(|it| (it.0.clone(), it.1));
        if let Some((board, score)) = best {
            return (Some(board), score);
        } else {
            return (None, i64::min_value());
        }
    }

    let mut best: Option<Board> = None;
    let mut score: i64 = i64::min_value();
    for b in board.successors(&team){
        if score != i64::min_value() && b.evaluate(team, dist_state) < 0 {
            continue;
        }

        if let (Some(n), resp_score) = max_move(&b, &team.other(), depth-1, dist_state) {
            if depth == DEBUG_DEPTH {
                println!("Best response for {:?} after \n{} is \n{}", team.other(), b.pprint(), n.pprint());
            }
            if score < -resp_score {
                score = -resp_score;
                best = Some(b);
            }
        } else {
            if depth == DEBUG_DEPTH {
                println!("Best response for {:?} after \n{} is \ngive up\n\n", team.other(), b.pprint());
            }
            best = Some(b);
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
    let b = max_move(&b0, &team, DEBUG_DEPTH, &mut diststate);
    println!("Board {:?}", b);
}

