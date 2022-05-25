use adversarial_2048::{Game, human::Human, random::Random};

fn main() {
    let mut g = Game::new(Box::new(Human), Box::new(Random));
    g.play();
}
