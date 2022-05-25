use crate::Player;
use crate::state::{self, State, Move, Direction, Role};
use std::io;

pub struct Human;

impl Player for Human {
    fn pick_move(&self, s: &State) -> Move {
        println!("Current state:");
        state::print_grid(&s.grid);

        if s.next_to_move == Role::Slider {
            loop {
                println!("Enter a direction (u/d/l/r): ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input line");

                let dir = match input.trim() {
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
            loop {
                println!("Enter a location x y: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input line");

                let mut nums = input.trim().split(' ');
                println!("{}", input);
                let x = match nums.next() {
                    Some(x) => x,
                    None => continue,
                };
                println!("x = {}, len = {}", x, x.len());
                let x = match x.parse::<usize>() {
                    Ok(x) => x,
                    Err(_) => continue,
                };
                println!("{}", x);
                let y = match nums.next() {
                    Some(y) => y,
                    None => continue,
                };
                println!("y = {}, len = {}", y, y.len());
                let y = match y.parse::<usize>() {
                    Ok(y) => y,
                    Err(_) => continue,
                };
                println!("{}", y);

                if s.grid[x][y] == 0 {
                    return Move::Place { x, y, val: 2 };
                }
            }
        }
    }
}
