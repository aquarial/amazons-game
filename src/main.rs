
mod solver;
mod util;

use solver::*;
use solver::board::*;

use util::event::*;

use std::collections::HashMap;
use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;

use termion::event::Key;

use tui::layout::{Constraint, Direction, Layout, Rect,};
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Map, MapResolution, Rectangle};
use tui::widgets::{Block, Borders, Widget};


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

fn main() -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    // Setup event handlers
    let events = Events::new();

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());
            Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("World"))
                .paint(|ctx| {
                    ctx.draw(&Rectangle {
                        color: Color::Red,
                        rect: Rect::new(10, 10, 41, 45),
                    });
                    ctx.print(10.0, 30.0, "You are here", Color::Yellow);
                    ctx.print(10.0, 10.0, "x are here", Color::Yellow);
                })
                .x_bounds([0.0, 100.0])
                .y_bounds([0.0, 100.0])
                .render(&mut f, chunks[0]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                _ => {}
            },
            Event::Tick => {
                //app.update();
            }
        }
    }

    Ok(())
}




fn oldmain() {
    let mut input: HashMap<Team, Player> = HashMap::new();

    if std::env::args().nth(1) == Some(String::from("--ai-battle")) {
        for t in Team::teams() {
            input.insert(t, Player::Ai(EvalStrategy::QueenDistance));
        }
    } else {
        for t in Team::teams() {
            while input.get(&t) == None {
                println!("{:?} is controlled by? [human, ai]", t);
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
        println!("{:?} to go, controlled by {:?}", team, player);
        println!("{}", amazons.curr_board().pprint());

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

