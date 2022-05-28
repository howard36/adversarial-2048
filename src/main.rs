use adversarial_2048::{Game, human::Human, random::Random, ai::Ai};

fn main() {
    let mut g = Game::new(Box::new(Random), Box::new(Ai::new()));
    g.play();
}
