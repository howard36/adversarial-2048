use crate::state::{Move, Role, State, Direction};
use crate::Player;
use std::collections::HashMap;

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
    value: f64,

    children: Vec<NodeKey>, // TODO: change to (weak?) pointer
}

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

/*
fn get_children(node: &Node) {
    let grid = hash_to_grid(node.hash);
    if node.next_to_move == Role::Slider {
        if let Some(new_grid) = slide_up(&grid) {
            children.push(grid_to_hash(new_grid));
        }
    } else {

    }
}
*/

impl NodeData {
    fn new(key: &NodeKey) -> NodeData {
        NodeData {
            search_depth: 0,
            value: 0f64,
            children: vec![], // TODO
        }
    }
}

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

    fn negamax(&mut self, key: NodeKey, depth: i32, sign: i32) -> f64 {
        let NodeKey { turns, grid } = key;
        if depth == 0 {
            return (sign * turns) as f64; // TODO: optimize leaf case
        }

        let g = match self.sym_map.get(&grid) {
            Some(&g) => g,
            None => {
                let new_grids = symmetries(grid);
                let max_grid = *new_grids.iter().max().unwrap();
                for grid in new_grids {
                    self.sym_map.insert(grid, max_grid);
                }
                max_grid
            }
        };
        let key = NodeKey { turns, grid: g };

        let node: &NodeData = self
            .node_map
            .entry(key)
            .or_insert_with_key(|key| NodeData::new(key));

        if depth <= node.search_depth {
            // already computed
            return node.value;
        }

        // TODO: use children.enumerate to save bext move?
        let value = node
            .children
            .clone()
            .into_iter()
            .map(|child_key| -self.negamax(child_key, depth - 1, -sign))
            .fold(f64::MIN, f64::max);

        let node = self.node_map.get_mut(&key).unwrap();
        node.value = value;
        node.search_depth = depth;

        value
    }

    fn best_root_move(&self) -> Move {
        Move::Slide(Direction::Up)
    }
}

impl Player for Ai {
    fn pick_move(&mut self, s: &State) -> Move {
        let sign = if s.next_to_move() == Role::Slider {
            1
        } else {
            -1
        };
        self.negamax(self.root_key, 5, sign);
        self.best_root_move()
    }

    fn update_move(&mut self, m: &Move, s: &State) {
        // root = root.children[1]
    }
}

// TODO: change return value to iterator
fn symmetries(grid: Grid) -> Vec<Grid> {
    vec![]
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
