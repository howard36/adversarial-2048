#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put1() {
        use game::*;
        let s = INITIAL_STATE;
        let m = Move::Put { x: 1, y: 2, val: 2 };
        let s = next_state(&s, m).unwrap();
        assert_eq!(s.grid, [
            [0, 0, 0, 0],
            [0, 0, 2, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]);
        assert_eq!(s, State {
            grid: [
                [0, 0, 0, 0],
                [0, 0, 2, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            next_to_move: Role::Slider,
            score: 0,
            terminal: false,
        });
    }
}

mod game {
    #[derive(Debug, PartialEq)]
    pub enum Role {
        Slider,
        Putter,
    }

    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    pub enum Move {
        Slide(Direction),
        Put { x: usize, y: usize, val: i32 },
    }

    #[derive(Debug, PartialEq)]
    pub struct State {
        pub grid: [[i32; 4]; 4],
        pub next_to_move: Role,
        pub score: i32,
        pub terminal: bool,
    }

    pub const INITIAL_STATE: State = State {
        grid: [[0; 4]; 4],
        next_to_move: Role::Putter,
        score: 0,
        terminal: false,
    };

    #[derive(Debug)]
    pub struct InvalidMove; // grid stayed the same

    fn slide_up(s: &State) -> Result<State, InvalidMove> {
        let grid = s.grid.clone();
        // modify grid

        Ok(State {
            grid,
            next_to_move: Role::Putter,
            score: s.score, // TODO
            terminal: false,
        })
    }

    fn slide_down(s: &State) -> Result<State, InvalidMove> {
        let grid = s.grid.clone();
        // modify grid

        Ok(State {
            grid,
            next_to_move: Role::Putter,
            score: s.score,
            terminal: false,
        })
    }

    fn slide_left(s: &State) -> Result<State, InvalidMove> {
        let grid = s.grid.clone();
        // modify grid

        Ok(State {
            grid,
            next_to_move: Role::Putter,
            score: s.score,
            terminal: false,
        })
    }

    fn slide_right(s: &State) -> Result<State, InvalidMove> {
        let grid = s.grid.clone();
        // modify grid

        Ok(State {
            grid,
            next_to_move: Role::Putter,
            score: s.score,
            terminal: false,
        })
    }

    fn put(s: &State, x: usize, y: usize, val: i32) -> Result<State, InvalidMove> {
        if s.grid[x][y] == 0 {
            let mut grid = s.grid.clone();
            grid[x][y] = val;

            Ok(State {
                grid,
                next_to_move: Role::Slider,
                score: s.score,
                terminal: false, // TODO:: should be "unknown" or try all 4 slides
            })
        } else {
            Err(InvalidMove)
        }
    }

    pub fn next_state(s: &State, m: Move) -> Result<State, InvalidMove> {
        match m {
            Move::Slide(d) => match d {
                Direction::Up => slide_up(s),
                Direction::Down => slide_down(s),
                Direction::Left => slide_left(s),
                Direction::Right => slide_right(s),
            },
            Move::Put { x, y, val } => put(s, x, y, val),
        }
    }
}

mod search_tree {
    
}

enum Strategy {
    Human,
    Random,
    AI,
}

struct Player {
    role: game::Role,
    strategy: Strategy,
}

pub fn play() {
    println!("Hello world!");
}
