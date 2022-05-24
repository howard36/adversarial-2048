use crate::Player;
use crate::state::{self, State, Move, Direction, Role};
use std::io;

pub struct Human;

impl Player for Human {
    fn pick_move(&self, s: &State) -> Move {
        println!("Current state:");
        state::print_grid(&s.grid);

        if s.next_to_move == Role::Slider {
            println!("Enter a direction (u/d/l/r): ");
            
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input line");
                let dir = match input.as_str() {
                    "u" => Direction::Up,
                    "d" => Direction::Down,
                    "l" => Direction::Left,
                    "r" => Direction::Right,
                    _ => continue,
                };
                let m = Move::Slide(dir);
                if state::next_state(&s, &m).is_ok() {
                    return m;
                }
            }
        } else {
            unimplemented!();
        }

    }

    fn update_move(&self, m: &Move, s: &State) {
    }
}
