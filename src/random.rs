use crate::state::{self, Direction, Move, Role, State, SLIDER_MOVES};
use crate::Player;
use itertools::iproduct;
use rand::prelude::IteratorRandom;

pub struct Random;

impl Player for Random {
    fn pick_move(&mut self, s: &State) -> Move {
        let mut rng = rand::thread_rng();
        if s.next_to_move() == Role::Slider {
            SLIDER_MOVES
            .into_iter()
            .filter(|&m| state::next_state(s, m).is_ok())
            .choose(&mut rng)
            .unwrap()
        } else {
            let grid = s.grid();
            iproduct!(0..4, 0..4, 0..2)
                .filter(|&(i, j, _)| grid[i][j] == 0)
                .map(|(i, j, k)| Move::Place {
                    x: i,
                    y: j,
                    val: 2 * k + 2,
                })
                .choose(&mut rng)
                .unwrap()
        }
    }
}
