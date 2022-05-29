use crate::state::{Direction, Move, Role, State, PLACER_MOVES, SLIDER_MOVES};
use crate::Player;
use ordered_float::NotNan;
use std::borrow::Borrow;
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
            value: NotNan::new(0.0).unwrap(),
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

    fn negamax(&mut self, key: NodeKey, depth: i32, sign: i32) -> NotNan<f32> {
        let NodeKey { turns, grid } = key;
        if depth == 0 {
            return NotNan::new((sign * turns) as f32).unwrap(); // TODO: optimize leaf case
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
            .max()
            .unwrap();

        let node = self.node_map.get_mut(&key).unwrap();
        node.value = value;
        node.search_depth = depth;

        value
    }

    fn best_root_move(&self) -> Move {
        let root_node = self.node_map.get(&self.root_key).unwrap();
        let moves: &[Move] = if self.root_key.turns % 2 == 0 {
            &PLACER_MOVES
        } else {
            &SLIDER_MOVES
        };
        let (_, i) = moves
            .iter()
            .enumerate()
            .filter_map(|(i, &m)| self.apply_move(&self.root_key, m).map(|k| (i, k)))
            .map(|(i, k)| (i, self.node_map.get(&k).unwrap()))
            .map(|(i, n)| (n.value, i))
            .max()
            .unwrap();
        moves[i]
    }

    // TODO: this should update turns
    fn apply_move(&self, key: &NodeKey, m: Move) -> Option<NodeKey> {
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
        .map(|grid| self.sym_map.get(&grid).unwrap())
        .map(|&grid| NodeKey {
            turns: *turns,
            grid,
        })
    }
}

impl Player for Ai {
    fn pick_move(&mut self, s: &State) -> Move {
        let sign = 2 * (self.root_key.turns % 2) - 1;
        self.negamax(self.root_key, 5, sign);
        self.best_root_move()
    }

    fn update_move(&mut self, m: &Move, s: &State) {
        // root = root.children[1]
    }
}

// TODO: optimize with bit operations when Grid = u64
fn symmetries(grid: Grid) -> [Grid; 8] {
    let mut ret: [Grid; 8] = [[[0u8; 4]; 4]; 8];
    for i in 0..4 {
        for j in 0..4 {
            let num = grid[i][j];
            ret[0][  i][  j] = num;
            ret[1][3-i][  j] = num;
            ret[2][  i][3-j] = num;
            ret[3][3-i][3-j] = num;
            ret[4][  j][  i] = num;
            ret[5][3-j][  i] = num;
            ret[6][  j][3-i] = num;
            ret[7][3-j][3-i] = num;
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
