mod tile;
mod transform;

use tile::*;
use transform::*;

use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let tiles = input.trim().split("\n\n").map(|line| line.parse()).collect::<Result<Vec<IdentifiedTile>, _>>().unwrap();

    let (rows, cols) = match tiles.len() {
        1 => (1, 1),
        4 => (2, 2),
        9 => (3, 3),
        16 => (4, 4),
        25 => (5, 5),
        36 => (6, 6),
        49 => (7, 7),
        64 => (8, 8),
        81 => (9, 9),
        100 => (10, 10),
        121 => (11, 11),
        144 => (12, 12),
        _ => panic!("Get lost"),
    };

    let mut borders: HashMap<TileIndex, (TileBorder, TileBorder, TileBorder, TileBorder)> = HashMap::new();
    let mut all: HashSet<TileIndex> = HashSet::new();
    for tile in &tiles {
        for transform in Transform::all() {
            let id = tile.id;
            let tile = tile.tile.transform(transform);
            let idx = TileIndex { id, transform };
            borders.insert(idx, (tile.top_border(), tile.right_border(), tile.bottom_border(), tile.left_border()));
            all.insert(idx);
        }
    }

    let all = solve(rows, cols, 0, 0, Solution::default(), HashSet::new(), all.clone(), &borders);
    println!("{} solutions:", all.len());
    for solution in all {
        let tl = solution.tiles.get(&(0, 0)).unwrap();
        let tr = solution.tiles.get(&(cols - 1, 0)).unwrap();
        let bl = solution.tiles.get(&(0, rows - 1)).unwrap();
        let br = solution.tiles.get(&(cols - 1, rows - 1)).unwrap();
        println!("{} {} {} {} => {}", tl.id, tr.id, bl.id, br.id, tl.id * tr.id * bl.id * br.id);
    }
}


fn solve(rows: usize, cols: usize, x: usize, y: usize, current: Solution, used: HashSet<usize>, remaining: HashSet<TileIndex>, borders: &HashMap<TileIndex, (TileBorder, TileBorder, TileBorder, TileBorder)>) -> Vec<Solution> {
    let mut possibilities = Vec::new();

    let target_left_border: Option<TileBorder>;
    if x > 0 {
        let to_the_left = current.tiles.get(&(x-1, y)).unwrap();
        let right_border = borders.get(to_the_left).unwrap().1;
        target_left_border = Some(right_border);
    }
    else {
        target_left_border = None;
    }

    let target_top_border: Option<TileBorder>;
    if y > 0 {
        let to_the_top = current.tiles.get(&(x, y-1)).unwrap();
        let bottom_border = borders.get(to_the_top).unwrap().2;
        target_top_border = Some(bottom_border);
    }
    else {
        target_top_border = None;
    }

    for index in &remaining {
        if used.contains(&index.id) {
            // println!("Skipping {:?} because {} has already been used", index, index.id);
            continue;
        }

        // println!("Trying {:?}", index);
        let (top_border, _right_border, _bottom_border, left_border) = borders.get(index).unwrap();
        if let Some(target_left_border) = target_left_border {
            if *left_border != target_left_border {
                // println!("Nope. Left border doesn't match.");
                continue;
            }
        }
        if let Some(target_top_border) = target_top_border {
            if *top_border != target_top_border {
                // println!("Nope. Top border doesn't match.");
                continue;
            }
        }
        // println!("Yep!");
        possibilities.push(index);
    }

    let mut results = Vec::new();

    for possibility in possibilities {
        if x == cols - 1 && y == rows - 1 {
            let mut current = current.clone();
            current.tiles.insert((x, y), *possibility);
            results.push(current);
        }
        else {
            // Recusively solve!
            let new_x;
            let new_y;
            if y == rows - 1 {
                new_x = x + 1;
                new_y = 0;
            } else {
                new_x = x;
                new_y = y + 1;
            }
            let mut current = current.clone();
            current.tiles.insert((x, y), *possibility);
            let mut used = used.clone();
            used.insert(possibility.id);
            let remaining = remaining.clone();
            let mut solutions = solve(rows, cols, new_x, new_y, current, used, remaining, borders);
            results.append(&mut solutions);
        }
    }

    results
}

#[derive(Debug, Clone)]
struct Solution {
    tiles: HashMap<(usize, usize), TileIndex>,
}

impl Default for Solution {
    fn default() -> Self {
        Solution {
            tiles: HashMap::new()
        }
    }
}

#[derive(Clone, Copy)]
struct IdentifiedTile {
    id: usize,
    tile: Tile,
}

impl std::str::FromStr for IdentifiedTile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, '\n');
        let first_line = parts.next().ok_or("Less than 2 lines")?;
        let rest = parts.next().ok_or("Less than 2 lines")?;

        let colon = first_line.find(':').ok_or("Missing colon")?;
        let id = first_line[5..colon].parse().map_err(|_| "Failed to parse number")?;
        let tile = rest.parse()?;

        Ok(Self {
            id,
            tile,
        })
    }
}

impl std::fmt::Debug for IdentifiedTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile {}:\n{:?}", self.id, self.tile)
    }
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
struct TileIndex {
    id: usize,
    transform: transform::Transform,
}
