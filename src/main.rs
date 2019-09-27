
mod board;

use board::*;


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
            if depth == DEBUG_DEPTH {
                println!("Best response for {:?} after \n{} is \n{}", piece.other(), b.pprint(), b.with_move(&n).pprint());
            }
            if score > -resp_score {
                score = -resp_score;
                best = Some(m);
            }
        } else {
            if depth == DEBUG_DEPTH {
                println!("Best response for {:?} after \n{} is \ngive up\n\n", piece.other(), b.pprint());
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
    let piece = Piece::White;

    println!("Start\n{}", b0.pprint());
    let m = max_move(&b0, &piece, DEBUG_DEPTH);
    println!("Move {:?}", m);

    if let (Some(m2), _) = m {
        println!("\nEnd\n{}", b0.with_move(&m2).pprint());
    }
}

