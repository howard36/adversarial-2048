use crate::state::{Direction, Move, State, PLACER_MOVES, SLIDER_MOVES, INITIAL_STATE};
use crate::Player;
use std::cmp;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use crate::utils::{self, log};

type Grid = [[u8; 4]; 4];

const TURNS_MOD: i32 = 64;

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

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct NodeKey {
    turns: i32,
    grid: Grid, // TODO: replace this with hash
}

#[derive(Debug)]
struct NodeData {
    // All nodes in the subtree up to this depth have been searched
    // Used for iterative deepening and memoized values for certain depths
    // If negamax_depth <= search_depth, just return the saved value
    // If this is a terminal node, set search_depth to infinity
    // search_depth = 0 if no children have been visited
    search_depth: i32,

    // upper and lower bounds of the actual value of the state
    upper_bound: i32,
    lower_bound: i32,

    // unflipped keys
    children: Vec<NodeKey>,

    // the best child, based on previous run of iterative deepening
    // None if not yet computed
    best_child: Option<NodeKey>,
}

#[wasm_bindgen]
pub struct Ai {
    // index = depth (root depth is 0)
    // even depth -> Placer, odd depth -> Slider
    // also encodes symmetry: 8 keys map to the same node
    sym_map: Vec<HashMap<Grid, Grid>>,
    node_map: Vec<HashMap<Grid, NodeData>>,
    root_key: NodeKey,
    search_depth: i32,
}

impl Ai {
    fn key_to_node(&mut self, key: NodeKey) -> (NodeKey, &mut NodeData) {
        let NodeKey { turns, grid } = key;
        let idx = (turns % TURNS_MOD) as usize;
        let max_grid = match self.sym_map[idx].get(&grid) {
            Some(&g) => g,
            None => {
                // TODO: change max to include 0,0 + 0,1
                let new_grids = symmetries(&grid);
                let max_grid = *new_grids.iter().max().unwrap();
                for flipped_grid in new_grids {
                    self.sym_map[idx].insert(flipped_grid, max_grid);
                }
                max_grid
            }
        };
        let flipped_key = NodeKey {
            turns,
            grid: max_grid,
        };
        let node = self.node_map[idx]
            .entry(max_grid)
            .or_insert_with(|| new_node(&flipped_key));
        (flipped_key, node)
    }

    fn negamax(&mut self, key: NodeKey, max_depth: i32, alpha: i32, beta: i32) -> i32 {
        let (key, node) = self.key_to_node(key);
        let mut a = alpha;
        let mut b = beta;

        if node.search_depth >= max_depth {
            // already computed
            if node.lower_bound >= b {
                return node.lower_bound;
            }
            if node.upper_bound <= a {
                return node.upper_bound;
            }
            a = cmp::max(a, node.lower_bound);
            b = cmp::min(b, node.upper_bound);
            if a >= b {
                return a;
            }
        } else {
            // overwrite existing bounds for old depth
            node.lower_bound = i32::MIN;
            node.upper_bound = i32::MAX;
        }

        if key.turns >= max_depth {
            // TODO: optimize leaf case
            let sign = 2 * (key.turns % 2) - 1;
            let value = sign * heuristic(&key.grid);
            node.upper_bound = value;
            node.lower_bound = value;
            node.search_depth = 0;
            return value;
        }

        let mut value = i32::MIN;
        // TODO: use children.enumerate to save bext move? (or save ordering)
        let mut best_child = None;
        // TODO: try let vec: &Vec = &node.children?
        for child_key in node.children.clone() {
            let v = -self.negamax(child_key, max_depth, -b, -a);
            if v > value {
                best_child = Some(child_key);
                value = v;
                a = cmp::max(a, value);
                if a >= b {
                    break;
                }
            }
        }

        let idx = (key.turns % TURNS_MOD) as usize;
        let node = self.node_map[idx].get_mut(&key.grid).unwrap();

        // 3 cases: v in (-infty, a], (a, b), or [b, +infty)
        // Set upper bound, both bounds, or lower bound in respective cases
        if value < b {
            node.upper_bound = value;
        }
        if value > a {
            node.lower_bound = value;
        }
        node.search_depth = max_depth;
        node.best_child = best_child;

        value
    }

    fn best_root_move(&mut self) -> Move {
        let moves: &[Move] = if self.root_key.turns % 2 == 0 {
            &PLACER_MOVES
        } else {
            &SLIDER_MOVES
        };

        let (_, root_node) = self.key_to_node(self.root_key);
        let best_child = root_node.best_child.unwrap();
        let (best_child_flipped, _) = self.key_to_node(best_child);

        let mut best_move = moves[0];
        for m in moves {
            if let Some(key) = apply_move(&self.root_key, *m) {
                let (key, _) = self.key_to_node(key);
                if key == best_child_flipped {
                    best_move = *m;
                }
            }
        }
        best_move
    }

    pub fn print_node(&mut self, key: NodeKey) {
        println!("printing node {key:?}");
        let node = self.key_to_node(key);
        println!("{node:?}");
    }
}

// TODO: this should update turns
// TODO: replace with apply_all_moves
fn apply_move(key: &NodeKey, m: Move) -> Option<NodeKey> {
    let NodeKey { turns, grid } = key;
    let mut turn_increment = 1;
    match m {
        Move::Slide(d) => match d {
            Direction::Up => slide_up(grid),
            Direction::Down => slide_down(grid),
            Direction::Left => slide_left(grid),
            Direction::Right => slide_right(grid),
        },
        Move::Place { x, y, val } => {
            turn_increment = val - 1; // 1 or 3
            place(grid, x, y, (val / 2) as u8)
        }
    }
    .map(|grid| NodeKey {
        turns: *turns + turn_increment,
        grid,
    })
}

fn new_node(key: &NodeKey) -> NodeData {
    let moves: &[Move] = if key.turns % 2 == 0 {
        &PLACER_MOVES
    } else {
        if dead_grid(&key.grid) {
            //println!("Dead grid at {} turns", key.turns);
            return NodeData {
                search_depth: i32::MAX, // exact value known
                upper_bound: -1000_000_000 + key.turns,
                lower_bound: -1000_000_000 + key.turns,
                children: vec![],
                best_child: None,
            };
        }
        &SLIDER_MOVES
    };
    // TODO: lazy child init (None, Some(Vec<NodeKey>))
    let children: Vec<NodeKey> = moves
        .into_iter()
        .filter_map(|&m| apply_move(&key, m))
        .collect();

    NodeData {
        search_depth: -1, // no heuristic calculated yet
        upper_bound: i32::MAX,
        lower_bound: i32::MIN,
        children,
        best_child: None,
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
        // TODO: assert state matches self.root_key.grid
        let max_depth = self.root_key.turns + self.search_depth;
        let v = self.negamax(self.root_key, max_depth, -i32::MAX, i32::MAX);
        println!(
            "negamax root value = {}, turns = {}",
            v, self.root_key.turns
        );
        self.best_root_move()
    }

    fn update_move(&mut self, m: &Move, _s: &State) {
        log!("updating move");
        let old_turns = (self.root_key.turns % TURNS_MOD) as usize;
        self.root_key = apply_move(&self.root_key, *m).unwrap();
        self.sym_map[old_turns].clear();
        self.node_map[old_turns].clear();
        //println!("{:?}", self.root_key);
    }
}

fn heuristic(grid: &Grid) -> i32 {
    let mut score: i32 = 0;
    let mut penalty: i32 = 0;

    // adjustable hyperparameters
    const H_DIFF: i32 = 1;
    const V_DIFF: i32 = 1;
    const H_REV: i32 = 3;
    const V_REV: i32 = 3;
    const H_EQ: i32 = 2;

    let mut sq = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            sq[i][j] = (grid[i][j] * grid[i][j]) as i32;
        }
    }
    const V_EQ: i32 = 2;
    // horizontal differences
    for i in 0..4 {
        for j in 0..3 {
            let d = sq[i][j + 1] - sq[i][j];
            penalty += (2 * H_DIFF + H_REV) * d.abs() + H_REV * d;
            if d == 0 {
                score += H_EQ * sq[i][j];
            }
        }
    }
    // vertical differences
    for i in 0..3 {
        for j in 0..4 {
            let d = sq[i + 1][j] - sq[i][j];
            penalty += (2 * V_DIFF + V_REV) * d.abs() + V_REV * d;
            if d == 0 {
                score += V_EQ * sq[i][j];
            }
        }
    }
    score - penalty
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

#[wasm_bindgen]
pub struct WasmPlace {
    x: usize,
    y: usize,
    val: i32,
}

#[wasm_bindgen]
impl WasmPlace {
    pub fn x(&self) -> usize { self.x }
    pub fn y(&self) -> usize { self.y }
    pub fn val(&self) -> i32 { self.val }

}

#[wasm_bindgen]
impl Ai {
    pub fn new(search_depth: i32) -> Ai {
        utils::set_panic_hook();
        let root_key = NodeKey {
            turns: 0,
            grid: [[0u8; 4]; 4],
        };
        let mut sym_map = Vec::new();
        let mut node_map = Vec::new();
        for _ in 0..TURNS_MOD {
            sym_map.push(HashMap::new());
            node_map.push(HashMap::new());
        }

        Ai {
            sym_map,
            node_map,
            root_key,
            search_depth,
        }
    }


    pub fn update_slider_move(&mut self, direction: i32) {
        let m = Move::Slide(match direction {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Invalid Direction"),
        });
        self.update_move(&m, &INITIAL_STATE);
    }

    pub fn get_placer_move(&mut self) -> WasmPlace {
        let m = self.pick_move(&INITIAL_STATE);
        self.update_move(&m, &INITIAL_STATE);
        match m {
            Move::Place { x, y, val } => WasmPlace { x, y, val },
            _ => panic!("Invalid move returned from pick_move"),
        }
    }

    pub fn init_from_grid(&mut self, grid1d: &[u8]) {
        log!("init from {grid1d:?}");
        
        let mut grid = [[0u8; 4]; 4];
        let mut turns = -1;
        for i in 0..16 {
            grid[i/4][i%4] = grid1d[i];
            turns += 2 * grid1d[i] as i32;
        }
        self.root_key = NodeKey {
            turns,
            grid,
        };

        for i in 0..TURNS_MOD {
            self.sym_map[i as usize].clear();
            self.node_map[i as usize].clear();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn hashing() {
        let hash1 = 12345;
        let grid = hash_to_grid(hash1);
        let hash2 = grid_to_hash(grid);
        assert_eq!(hash1, hash2);
    }
    */
    use crate::state::INITIAL_STATE;

    #[test]
    #[ignore]
    fn predict_death() {
        let s = INITIAL_STATE;
        let mut ai = Ai::new(13);
        let m = ai.pick_move(&s);
        //let m = Move::Slide(Direction::Left);
        println!("chosen move: {:?}", m);
        ai.update_move(&m, &s);
        ai.update_move(&Move::Place { x: 3, y: 2, val: 2 }, &s);
        let m = ai.pick_move(&s); // wrong choice
        println!("chosen move: {:?}", m);
        ai.print_node(NodeKey {
            turns: 5025,
            grid: [[5, 9, 10, 11], [4, 6, 8, 10], [1, 3, 4, 2], [4, 1, 1, 0]],
        });
        ai.print_node(NodeKey {
            turns: 5026,
            grid: [[11, 10, 9, 5], [10, 8, 6, 4], [2, 4, 3, 1], [0, 0, 2, 4]],
        });
        //ai.print_node(NodeKey { turns: 5027, grid: [[11, 10, 9, 5], [10, 8, 6, 4], [2, 4, 3, 1], [0, 1, 2, 4]] });
        ai.print_node(NodeKey {
            turns: 5027,
            grid: [[11, 10, 9, 5], [10, 8, 6, 4], [2, 4, 3, 1], [1, 0, 2, 4]],
        });
        ai.update_move(&m, &s);
        let m = ai.pick_move(&s);
        println!("chosen move: {:?}", m);
    }

    #[test]
    fn empty_children_bug() {
        let key = NodeKey {
            turns: 771,
            grid: [[8, 7, 6, 5], [7, 6, 4, 3], [5, 4, 3, 2], [1, 3, 2, 1]],
        };
        let node = new_node(&key);
        println!("{node:?}");
    }
}
