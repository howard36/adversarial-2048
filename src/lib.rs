pub mod ai;
pub mod human;
pub mod random;
mod state;
mod utils;

use state::{Move, Role, State};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub trait Player {
    fn pick_move(&mut self, s: &State) -> Move;

    fn update_move(&mut self, _m: &Move, _s: &State) {}
}

pub struct Game {
    slider: Box<dyn Player>,
    placer: Box<dyn Player>,
    state: State,
    //history: Vec<State>,
}

impl Game {
    pub fn new(slider: Box<dyn Player>, placer: Box<dyn Player>) -> Game {
        Game {
            slider,
            placer,
            state: state::INITIAL_STATE,
            //history: Vec::new(),
        }
    }

    pub fn play(&mut self) {
        while !self.state.terminal() {
            let m = if self.state.next_to_move() == Role::Slider {
                state::print_grid(self.state.grid());
                self.slider.pick_move(&self.state)
            } else {
                self.placer.pick_move(&self.state)
            };
            let s = state::next_state(&self.state, m).unwrap();
            //self.history.push(old_state);
            self.slider.update_move(&m, &s);
            self.placer.update_move(&m, &s);
            self.state = s;
        }
        println!("Game over! Score = {}, Final state =", self.state.score());
        state::print_grid(self.state.grid());
    }
}

#[wasm_bindgen]
pub fn greet() {
    println!("Hi");
}
