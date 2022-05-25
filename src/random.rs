use crate::Player;
use crate::state::{self, State, Move, Role, Direction};
use itertools::iproduct;
use rand::prelude::IteratorRandom;

pub struct Random;


impl Player for Random {
    fn pick_move(&self, s: &State) -> Move {
        let mut rng = rand::thread_rng();
        if s.next_to_move() == Role::Slider {
            vec![
                Move::Slide(Direction::Up),
                Move::Slide(Direction::Down),
                Move::Slide(Direction::Left),
                Move::Slide(Direction::Right),
            ].into_iter()
                .filter(|m| state::next_state(s, &m).is_ok())
                .choose(&mut rng).unwrap()
        } else {
            let grid = s.grid();
            iproduct!(0..4, 0..4, 0..2)
                .filter(|&(i, j, _)| grid[i][j] == 0)
                .map(|(i, j, k)| Move::Place { x: i, y: j, val: 2*k+2 })
                .choose(&mut rng).unwrap()
        }
    }
}
