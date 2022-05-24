use adversarial_2048::{Game, human::Human};

fn main() {
    let mut g = Game::new(Box::new(Human), Box::new(Human));
    g.play();
}
