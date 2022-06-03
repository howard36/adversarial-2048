use adversarial_2048::{ai::Ai, human::Human, random::Random, Game};

fn main() {
    let slider = Box::new(Human);
    let placer = Box::new(Ai::new());
    let mut g = Game::new(slider, placer);
    g.play();
}
