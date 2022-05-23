mod game;

mod search_tree {
    
}

pub enum Strategy {
    Human,
    Random,
    AI,
}

struct FullGame {
    slider: Strategy,
    putter: Strategy,
    state: game::State,
    history: Vec<game::State>,
}

/*
fn pick_move(&strategy: Strategy, &s: game::State) -> game::Move {
    match strategy {
        Strategy::Human => get_human_move(s),
        Strategy::Random => get_random_move(s),
        Strategy::AI => get_ai_move(s),
    }
}

fn notify_move(&strategy: Strategy, &m: game::Move, &s: game::State) {
    match strategy {
        Strategy::Human => notify_human_move(m, s),
        Strategy::Random => (),
        Strategy::AI => notify_ai_move(m),
    }
}
*/

pub fn play(s1: Strategy, s2: Strategy) {
    println!("Hello world!");
}
