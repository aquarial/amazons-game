
mod solver;

use solver::*;
use solver::board::*;

use std::collections::HashMap;
use std::io;


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

    //let solver = Solver::new_8x8();
    let mut amazons = Amazons::new_8x8();
    let mut team = Team::White;

    loop {
        let player = input[&team];
        println!("{:?} to go, controlled by {:?}", team, player);
        println!("{}", amazons.curr_board().pprint());

        match player {
            Player::Ai => {
                let succs = amazons.curr_board().successors(team).count();
                let depth = 4;
                println!("Choosing among {} moves with {} depth", succs, depth);

                if amazons.ai_move(team) {
                    team = team.other();
                } else {
                    break;
                }
            },
            Player::Human => {
                let mut buffer = String::new();
                loop {
                    println!("Choose move for team {:?} in format 'RowCol RowCol RowCol'", team);
                    buffer.clear();
                    io::stdin().read_line(&mut buffer);
                    let input = buffer.trim();

                    if input == "ai" {
                        amazons.ai_move(team);
                        team = team.other();
                        break;
                    } else if input == "pieces" {
                        println!("Team {:?} has the following pieces:", team);
                        for p in amazons.team_pieces(team) {
                            println!("    {}{}", p.row, p.col);
                        }
                        println!();
                    } else if input == "undo" {
                        amazons.undo_2_move();
                        break;
                    } else if let Some((p,m,s)) = parse_move(input) {
                        if amazons.player_move(team, p, m, s) {
                            team = team.other();
                            break;
                        }
                    } else {
                        println!("Could not parse coords");
                    }
                }
            }
        }
    }
}

