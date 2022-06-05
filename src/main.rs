#[allow(unused_imports)]
use adversarial_2048::{ai::Ai, human::Human, random::Random, Game};

fn main() {
    let slider = Box::new(Ai::new(13));
    let placer = Box::new(Random);
    let mut g = Game::new(slider, placer);
    g.play();
}
