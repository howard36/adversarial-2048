use crate::state::{Direction, Move, State, PLACER_MOVES, SLIDER_MOVES};
use crate::Player;
use ordered_float::NotNan;
use std::collections::HashMap;
use std::cmp;
use num_traits::bounds::Bounded;

type Grid = [[u8; 4]; 4];

fn slide_up(g: &Grid) -> Option<Grid> {
    let mut grid = [[0; 4]; 4];
    for i in 0..4 {
        let mut end = 0;
        for j in 0..4 {
            if g[j][i] != 0 {
                if grid[end][i] == 0 {
                    grid[end][i] = g[j][i];
                } else if grid[end][i] == g[j][i] {
                    grid[end][i] += 1;
                    end += 1;
                } else {
                    end += 1;
                    grid[end][i] = g[j][i];
                }
            }
        }
    }

    if grid == *g {
        None
    } else {
        Some(grid)
    }
}

fn slide_down(g: &Grid) -> Option<Grid> {
    let mut grid = [[0; 4]; 4];
    for i in 0..4 {
        let mut end = 0;
        for j in 0..4 {
            if g[3 - j][i] != 0 {
                if grid[3 - end][i] == 0 {
                    grid[3 - end][i] = g[3 - j][i];
                } else if grid[3 - end][i] == g[3 - j][i] {
                    grid[3 - end][i] += 1;
                    end += 1;
                } else {
                    end += 1;
                    grid[3 - end][i] = g[3 - j][i];
                }
            }
        }
    }

    if grid == *g {
        None
    } else {
        Some(grid)
    }
}

fn slide_left(g: &Grid) -> Option<Grid> {
    let mut grid = [[0; 4]; 4];
    for i in 0..4 {
        let mut end = 0;
        for j in 0..4 {
            if g[i][j] != 0 {
                if grid[i][end] == 0 {
                    grid[i][end] = g[i][j];
                } else if grid[i][end] == g[i][j] {
                    grid[i][end] += 1;
                    end += 1;
                } else {
                    end += 1;
                    grid[i][end] = g[i][j];
                }
            }
        }
    }

    if grid == *g {
        None
    } else {
        Some(grid)
    }
}

fn slide_right(g: &Grid) -> Option<Grid> {
    let mut grid = [[0; 4]; 4];
    for i in 0..4 {
        let mut end = 0;
        for j in 0..4 {
            if g[i][3 - j] != 0 {
                if grid[i][3 - end] == 0 {
                    grid[i][3 - end] = g[i][3 - j];
                } else if grid[i][3 - end] == g[i][3 - j] {
                    grid[i][3 - end] += 1;
                    end += 1;
                } else {
                    end += 1;
                    grid[i][3 - end] = g[i][3 - j];
                }
            }
        }
    }

    if grid == *g {
        None
    } else {
        Some(grid)
    }
}

fn place(g: &Grid, x: usize, y: usize, val: u8) -> Option<Grid> {
    if g[x][y] == 0 {
        let mut grid = g.clone();
        grid[x][y] = val;
        Some(grid)
    } else {
        None
    }
}

/*
enum Value {
    Estimate(f64),  // estimate based on searched nodes (with depth cutoff)
    Exact(u32),     // exact value of game state, after exhausting subtree
}
*/

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct NodeKey {
    turns: i32,
    grid: Grid, // TODO: replace this with hash
}

//impl Borrow<Grid> for Grid {}

struct NodeData {
    //grid_hash: u64, // 4x4 array
    //next_to_move: Role,

    // also the score of this node (for Slider)
    //depth: u32,

    // all nodes in the subtree up to this depth have been searched
    // used for iterative deepening and memoized values for certain depths
    // if negamax_depth <= search_depth, just return the saved value
    // if this is a terminal node, set search_depth to infinity
    // search_depth = 0 if only this node has been visited
    search_depth: i32,

    // the final score if optimal players start from this state
    value: NotNan<f32>,

    children: Vec<NodeKey>, // TODO: change to (weak?) pointer
}

/*
fn hash_to_grid(hash: u64) -> Grid {
    let mut h = hash;
    let mut grid = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            grid[i][j] = (h % 16) as u8;
            h /= 16;
        }
    }
    grid
}

fn grid_to_hash(grid: Grid) -> u64 {
    let mut hash = 0;
    for i in 0..4 {
        for j in 0..4 {
            hash += (grid[i][j] as u64) << (4 * (4 * i + j));
        }
    }
    hash
}
*/

pub struct Ai {
    // index = depth (root depth is 0)
    // even depth -> Placer, odd depth -> Slider
    // All scores will be odd, because only the Placer can end the game
    // also encodes symmetry: 8 keys map to the same node
    node_map: HashMap<NodeKey, NodeData>,
    sym_map: HashMap<Grid, Grid>,
    root_key: NodeKey,
}

impl Ai {
    pub fn new() -> Ai {
        let root_key = NodeKey {
            turns: 0,
            grid: [[0u8; 4]; 4],
        };
        Ai {
            node_map: HashMap::new(),
            sym_map: HashMap::new(),
            root_key,
        }
    }

    fn negamax(
        &mut self,
        key: NodeKey,
        depth: i32,
        sign: i32,
        alpha: NotNan<f32>,
        beta: NotNan<f32>,
    ) -> NotNan<f32> {
        let NodeKey { turns, grid } = key;
        if depth == 0 {
            return NotNan::new((sign * heuristic(&grid)) as f32).unwrap(); // TODO: optimize leaf case
        }

        let g = match self.sym_map.get(&grid) {
            Some(&g) => g,
            None => {
                let new_grids = symmetries(&grid);
                let max_grid = *new_grids.iter().max().unwrap();
                for grid in new_grids {
                    self.sym_map.insert(grid, max_grid);
                }
                max_grid
            }
        };
        //println!("g = {:?}", g);
        let key = NodeKey { turns, grid: g };

        let node = self
            .node_map
            .entry(key)
            .or_insert_with_key(|key| new_node(key));

        if depth <= node.search_depth {
            // already computed
            return node.value;
        }

        // TODO: use children.enumerate to save bext move?
        let mut value = NotNan::min_value();
        let mut a = alpha;
        for child_key in node.children.clone() {
            value = cmp::max(value, -self.negamax(child_key, depth - 1, -sign, -beta, -a));
            a = cmp::max(a, value);
            if a>= beta {
                break;
            }
        }

        let node = self.node_map.get_mut(&key).unwrap();
        node.value = value;
        node.search_depth = depth;

        value
    }

    fn best_root_move(&self) -> Move {
        let moves: &[Move] = if self.root_key.turns % 2 == 0 {
            &PLACER_MOVES
        } else {
            &SLIDER_MOVES
        };
        let (_, i) = moves
            .iter()
            .enumerate()
            .filter_map(|(i, &m)| {
                apply_move(&self.root_key, m)
                    .map(|k| NodeKey {
                        turns: k.turns,
                        grid: *self.sym_map.get(&k.grid).unwrap(),
                    })
                    .map(|k| (self.node_map.get(&k).unwrap().value, i))
            })
            .max()
            .unwrap();
        //println!("Best root move idx = {}", i);
        moves[i]
    }
}

// TODO: this should update turns
// TODO: replace with apply_all_moves
fn apply_move(key: &NodeKey, m: Move) -> Option<NodeKey> {
    let NodeKey { turns, grid } = key;
    match m {
        Move::Slide(d) => match d {
            Direction::Up => slide_up(grid),
            Direction::Down => slide_down(grid),
            Direction::Left => slide_left(grid),
            Direction::Right => slide_right(grid),
        },
        Move::Place { x, y, val } => place(grid, x, y, (val / 2) as u8), // TODO
    }
    .map(|grid| NodeKey {
        turns: *turns + 1,
        grid,
    })
}

fn new_node(key: &NodeKey) -> NodeData {
    let moves: &[Move] = if key.turns % 2 == 0 {
        &PLACER_MOVES
    } else {
        if dead_grid(&key.grid) {
            println!("Dead grid at {} turns", key.turns);
            return NodeData {
                search_depth: i32::MAX,
                value: NotNan::new(-1000000.0 + key.turns as f32).unwrap(),
                children: vec![],
            };
        }
        &SLIDER_MOVES
    };
    let children: Vec<NodeKey> = moves
        .into_iter()
        .filter_map(|&m| apply_move(&key, m))
        .collect();

    NodeData {
        search_depth: 0,
        value: NotNan::new(0.0).unwrap(),
        children,
    }
}

fn dead_grid(g: &Grid) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if g[i][j] == 0 {
                return false;
            }
        }
        for j in 0..3 {
            if g[i][j] == g[i][j + 1] {
                return false;
            }
            if g[j][i] == g[j + 1][i] {
                return false;
            }
        }
    }
    true
}

impl Player for Ai {
    fn pick_move(&mut self, _s: &State) -> Move {
        //println!("{:?}", s.grid());
        //println!("{:?}", self.root_key.grid);
        // TODO: assert state matches self.root_key.grid
        let sign = 2 * (self.root_key.turns % 2) - 1;
        let v = self.negamax(self.root_key, 10, sign, NotNan::min_value(), NotNan::max_value());
        println!("negamax root value = {}", v);
        self.best_root_move()
    }

    fn update_move(&mut self, m: &Move, _s: &State) {
        self.root_key = apply_move(&self.root_key, *m).unwrap();
    }
}

fn heuristic(grid: &Grid) -> i32 {
    let mut sum: i32 = 0;
    let mut penalty: i32 = 0;
    for i in 0..4 {
        for j in 0..3 {
            sum += 1 << grid[i][j];
            penalty += (1i32 << grid[i][j]) - (1i32 << grid[i][j+1]).abs();
            penalty += (1i32 << grid[j][i]) - (1i32 << grid[j+1][i]).abs();
        }
    }
    sum * 4 - penalty
}

// TODO: optimize with bit operations when Grid = u64
fn symmetries(grid: &Grid) -> [Grid; 8] {
    let mut ret: [Grid; 8] = [[[0u8; 4]; 4]; 8];
    for i in 0..4 {
        for j in 0..4 {
            let num = grid[i][j];
            ret[0][i][j] = num;
            ret[1][3 - i][j] = num;
            ret[2][i][3 - j] = num;
            ret[3][3 - i][3 - j] = num;
            ret[4][j][i] = num;
            ret[5][3 - j][i] = num;
            ret[6][j][3 - i] = num;
            ret[7][3 - j][3 - i] = num;
        }
    }
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashing() {
        let hash1 = 12345;
        let grid = hash_to_grid(hash1);
        let hash2 = grid_to_hash(grid);
        assert_eq!(hash1, hash2);
    }
}
