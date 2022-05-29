pub mod ai;
pub mod human;
pub mod random;
mod state;

use state::{Move, Role, State};

pub trait Player {
    fn pick_move(&mut self, s: &State) -> Move;

    #[allow(unused_variables)]
    fn update_move(&mut self, m: &Move, s: &State) {}
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
        println!("Game over! Final state =");
        state::print_grid(self.state.grid());
    }
}
