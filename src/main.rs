
mod solver;

use solver::*;
use solver::board::*;

use std::io;
use std::io::Write;
use std::collections::HashMap;

use termion::color;
use termion::cursor;
//use termion::raw::RawTerminal;
use termion::clear;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use termion::event::Event;

fn parse_num(c: char) -> Option<i8> {
    for (i,t) in "12345678".chars().enumerate() {
        if c == t {
            return Some((i+1) as i8);
        }
    }
    for (i,t) in "abcdefgh".chars().enumerate() {
        if c == t {
            return Some((i+1) as i8);
        }
    }
    return None;
}

fn parse_pos(s: &str) -> Option<Pos> {
    let pos: Vec<i8> = s.chars().map(parse_num).filter_map(|i| i).collect();
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
    Ai(EvalStrategy),
    Human,
}

fn render_board(amazons: &mut Amazons) {
    let mut draw_board = DrawableBoard::new();
    amazons.curr_board().draw_board(&mut draw_board);

    println!();
    for r in 0..draw_board.board.len() {
        print!("  ");
        for c in 0..draw_board.board[r].len() {
            print!("{}", render_token(draw_board.board[r][c], (r+c)%2 == 0));
            print!("{}", render_token(draw_board.board[r][c], (r+c)%2 == 0));
        }
        print!("{}", color::Bg(color::Reset));
        println!();
    }
    print!("{}", color::Bg(color::Reset));
}

fn render_token(dt: DrawableToken, even: bool) -> String {
    format!("{}{}{}", token_fg(dt), token_bg(dt, even), token_char(dt))
}

fn token_char(dt: DrawableToken) -> String {
    match dt {
        DrawableToken::Empty => String::from(" "),
        DrawableToken::Wall => String::from("#"),
        DrawableToken::Piece(Team::Red) => String::from("R"),
        DrawableToken::Piece(Team::Blue) => String::from("B"),
    }
}

fn token_fg(dt: DrawableToken) -> String {
    match dt {
        DrawableToken::Empty => format!("{}", color::Fg(color::LightBlack)),
        DrawableToken::Wall => format!("{}", color::Fg(color::White)),
        DrawableToken::Piece(Team::Red) => format!("{}", color::Fg(color::Rgb(250, 60, 60))),
        DrawableToken::Piece(Team::Blue) => format!("{}", color::Fg(color::Rgb(32, 155, 250))),
    }
}

fn token_bg(dt: DrawableToken, even: bool) -> String {
    let checkered = if even {
        format!("{}", color::Bg(color::Rgb(128, 76, 21)))
    } else {
        format!("{}", color::Bg(color::Rgb(150, 105, 57)))
    };

    match dt {
        DrawableToken::Wall => checkered,//format!("{}", color::Bg(color::Rgb(30, 30, 30))),
        DrawableToken::Empty => checkered,
        DrawableToken::Piece(_) => checkered,
    }
}

fn main() {
    let mut input: HashMap<Team, Player> = HashMap::new();

    if std::env::args().nth(1) == Some(String::from("--ai-battle")) {
        for t in Team::teams() {
            input.insert(t, Player::Ai(EvalStrategy::QueenDistance));
        }
    } else {
        for t in Team::teams() {
            while input.get(&t) == None {
                println!("{:?} is controlled by? [human, ai queen, ai king]", t);
                let mut line = String::new();
                io::stdin().read_line(&mut line);
                let parts: Vec<String> = line.trim().split_ascii_whitespace().map(|s| String::from(s)).collect();
                if parts.len() >= 1 {
                    if parts[0] == "ai" {
                        if parts.len() >= 2 && parts[1] == "king" {
                            input.insert(t, Player::Ai(EvalStrategy::KingDistance));
                        } else {
                            input.insert(t, Player::Ai(EvalStrategy::QueenDistance));
                        }
                    }
                    if parts[0] == "human" {
                        input.insert(t, Player::Human);
                    }
                }
            }
        }
    }

    let mut amazons = Amazons::new_5x5();
    let mut team = Team::Red;

    loop {
        let player = input[&team];
        render_board(&mut amazons);
        println!();
        println!("{:?} to go, controlled by {:?}", team, player);

        match player {
            Player::Ai(s) => {
                if amazons.ai_move(team, s) {
                    team = team.other();
                } else {
                    println!("AI for team {:?} gives up", team);
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
                        amazons.ai_move(team, EvalStrategy::QueenDistance);
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

