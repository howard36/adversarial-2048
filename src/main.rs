use adversarial_2048::{ai::Ai, human::Human, random::Random, Game};

fn main() {
    let mut g = Game::new(Box::new(Human), Box::new(Ai::new()));
    g.play();
}
