#[derive(Debug, PartialEq)]
pub enum Role {
    Slider,
    Placer,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Move {
    Slide(Direction),
    Place { x: usize, y: usize, val: i32 },
}

#[derive(Debug, PartialEq)]
pub struct State {
    grid: [[i32; 4]; 4],
    next_to_move: Role,
    score: i32,
    terminal: bool,
}

pub const INITIAL_STATE: State = State {
    grid: [[0; 4]; 4],
    next_to_move: Role::Placer,
    score: 0,
    terminal: false,
};

#[derive(Debug)]
pub struct InvalidMove; // grid stayed the same

fn slide_up(s: &State) -> Result<State, InvalidMove> {
    let mut grid = [[0; 4]; 4];
    let mut score = s.score;
    for i in 0..4 {
        let mut end = 0;
        for j in 0..4 {
            if s.grid[j][i] != 0 {
                if grid[end][i] == 0 {
                    grid[end][i] = s.grid[j][i];
                } else {
                    if grid[end][i] == s.grid[j][i] {
                        grid[end][i] *= 2;
                        score += grid[end][i];
                        end += 1;
                    } else {
                        end += 1;
                        grid[end][i] = s.grid[j][i];
                    }
                }
            }
        }
    }

    if grid == s.grid {
        Err(InvalidMove)
    } else {
        Ok(State {
            grid,
            next_to_move: Role::Placer,
            score,
            terminal: false,
        })
    }
}

fn slide_down(s: &State) -> Result<State, InvalidMove> {
    let mut grid = [[0; 4]; 4];
    let mut score = s.score;
    for i in 0..4 {
        let mut end = 0;
        for j in 0..4 {
            if s.grid[3-j][i] != 0 {
                if grid[3-end][i] == 0 {
                    grid[3-end][i] = s.grid[3-j][i];
                } else {
                    if grid[3-end][i] == s.grid[3-j][i] {
                        grid[3-end][i] *= 2;
                        score += grid[3-end][i];
                        end += 1;
                    } else {
                        end += 1;
                        grid[3-end][i] = s.grid[3-j][i];
                    }
                }
            }
        }
    }

    if grid == s.grid {
        Err(InvalidMove)
    } else {
        Ok(State {
            grid,
            next_to_move: Role::Placer,
            score,
            terminal: false,
        })
    }
}

fn slide_left(s: &State) -> Result<State, InvalidMove> {
    let mut grid = [[0; 4]; 4];
    let mut score = s.score;
    for i in 0..4 {
        let mut end = 0;
        for j in 0..4 {
            if s.grid[i][j] != 0 {
                if grid[i][end] == 0 {
                    grid[i][end] = s.grid[i][j];
                } else {
                    if grid[i][end] == s.grid[i][j] {
                        grid[i][end] *= 2;
                        score += grid[i][end];
                        end += 1;
                    } else {
                        end += 1;
                        grid[i][end] = s.grid[i][j];
                    }
                }
            }
        }
    }

    if grid == s.grid {
        Err(InvalidMove)
    } else {
        Ok(State {
            grid,
            next_to_move: Role::Placer,
            score,
            terminal: false,
        })
    }
}

fn slide_right(s: &State) -> Result<State, InvalidMove> {
    let mut grid = [[0; 4]; 4];
    let mut score = s.score;
    for i in 0..4 {
        let mut end = 0;
        for j in 0..4 {
            if s.grid[i][3-j] != 0 {
                if grid[i][3-end] == 0 {
                    grid[i][3-end] = s.grid[i][3-j];
                } else {
                    if grid[i][3-end] == s.grid[i][3-j] {
                        grid[i][3-end] *= 2;
                        score += grid[i][3-end];
                        end += 1;
                    } else {
                        end += 1;
                        grid[i][3-end] = s.grid[i][3-j];
                    }
                }
            }
        }
    }

    if grid == s.grid {
        Err(InvalidMove)
    } else {
        Ok(State {
            grid,
            next_to_move: Role::Placer,
            score,
            terminal: false,
        })
    }
}

fn dead_grid(grid: &[[i32; 4]; 4]) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if grid[i][j] == 0 {
                return false;
            }
        }
        for j in 0..3 {
            if grid[i][j] == grid[i][j+1] {
                return false;
            }
            if grid[j][i] == grid[j+1][i] {
                return false;
            }
        }
    }
    true
}

fn place(s: &State, x: usize, y: usize, val: i32) -> Result<State, InvalidMove> {
    if s.grid[x][y] == 0 {
        let mut grid = s.grid.clone();
        grid[x][y] = val;

        Ok(State {
            grid,
            next_to_move: Role::Slider,
            score: s.score,
            terminal: dead_grid(&s.grid),
        })
    } else {
        Err(InvalidMove)
    }
}

pub fn next_state(s: &State, m: Move) -> Result<State, InvalidMove> {
    match m {
        Move::Slide(d) => match d {
            Direction::Up    => slide_up(s),
            Direction::Down  => slide_down(s),
            Direction::Left  => slide_left(s),
            Direction::Right => slide_right(s),
        },
        Move::Place { x, y, val } => place(s, x, y, val),
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place1() {
        let s = INITIAL_STATE;
        let m = Move::Place { x: 1, y: 2, val: 2 };
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

    #[test]
    fn slide_left() {
        let s = INITIAL_STATE;
        let m = Move::Place { x: 1, y: 2, val: 2 };
        let s1 = next_state(&s, m).unwrap();
        let m2 = Move::Slide(Direction::Left);
        let s2 = next_state(&s1, m2).unwrap();

        assert_eq!(s2.grid, [
            [0, 0, 0, 0],
            [2, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]);
    }

    #[test]
    fn slide_left2() {
        let s = State {
            grid: [
                [2, 2, 2, 2],
                [0, 4, 0, 4],
                [2, 0, 4, 2],
                [4, 4, 2, 2],
            ],
            next_to_move: Role::Slider,
            score: 0,
            terminal: false,
        };
        let m2 = Move::Slide(Direction::Left);
        let s2 = next_state(&s, m2).unwrap();

        assert_eq!(s2.grid, [
            [4, 4, 0, 0],
            [8, 0, 0, 0],
            [2, 4, 2, 0],
            [8, 4, 0, 0],
        ]);
    }

    #[test]
    fn slide_left3() {
        let s = State {
            grid: [
                [2, 4, 2, 0],
                [2, 2, 4, 0],
                [2, 0, 4, 2],
                [0, 2, 4, 2],
            ],
            next_to_move: Role::Slider,
            score: 0,
            terminal: false,
        };
        let m2 = Move::Slide(Direction::Left);
        let s2 = next_state(&s, m2).unwrap();

        assert_eq!(s2.grid, [
            [2, 4, 2, 0],
            [4, 4, 0, 0],
            [2, 4, 2, 0],
            [2, 4, 2, 0],
        ]);
    }

    #[test]
    fn slide_right() {
        let s = State {
            grid: [
                [2, 4, 2, 0],
                [2, 2, 4, 0],
                [2, 0, 2, 2],
                [0, 2, 4, 2],
            ],
            next_to_move: Role::Slider,
            score: 0,
            terminal: false,
        };
        let m2 = Move::Slide(Direction::Right);
        let s2 = next_state(&s, m2).unwrap();

        assert_eq!(s2.grid, [
            [0, 2, 4, 2],
            [0, 0, 4, 4],
            [0, 0, 2, 4],
            [0, 2, 4, 2],
        ]);
    }

    #[test]
    fn slide_right2() {
        let s = State {
            grid: [
                [2, 2, 2, 2],
                [0, 4, 0, 4],
                [2, 0, 4, 2],
                [4, 4, 2, 2],
            ],
            next_to_move: Role::Slider,
            score: 0,
            terminal: false,
        };
        let m2 = Move::Slide(Direction::Right);
        let s2 = next_state(&s, m2).unwrap();

        assert_eq!(s2.grid, [
            [0, 0, 4, 4],
            [0, 0, 0, 8],
            [0, 2, 4, 2],
            [0, 0, 8, 4],
        ]);
    }

    #[test]
    fn slide_up() {
        let s = State {
            grid: [
                [2, 2, 2, 2],
                [0, 4, 0, 4],
                [2, 0, 4, 2],
                [4, 4, 2, 2],
            ],
            next_to_move: Role::Slider,
            score: 0,
            terminal: false,
        };
        let m2 = Move::Slide(Direction::Up);
        let s2 = next_state(&s, m2).unwrap();

        assert_eq!(s2.grid, [
            [4, 2, 2, 2],
            [4, 8, 4, 4],
            [0, 0, 2, 4],
            [0, 0, 0, 0],
        ]);
    }

    #[test]
    fn slide_up2() {
        let s = State {
            grid: [
                [0, 0, 0, 0],
                [0, 0, 2, 0],
                [4, 0, 0, 0],
                [4, 0, 2, 0],
            ],
            next_to_move: Role::Slider,
            score: 0,
            terminal: false,
        };
        let m2 = Move::Slide(Direction::Up);
        let s2 = next_state(&s, m2).unwrap();

        assert_eq!(s2.grid, [
            [8, 0, 4, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]);
    }

    #[test]
    fn slide_down() {
        let s = State {
            grid: [
                [2, 2, 2, 2],
                [0, 4, 0, 4],
                [2, 0, 4, 2],
                [4, 4, 2, 2],
            ],
            next_to_move: Role::Slider,
            score: 0,
            terminal: false,
        };
        let m2 = Move::Slide(Direction::Down);
        let s2 = next_state(&s, m2).unwrap();

        assert_eq!(s2.grid, [
            [0, 0, 0, 0],
            [0, 0, 2, 2],
            [4, 2, 4, 4],
            [4, 8, 2, 4],
        ]);
    }
}
