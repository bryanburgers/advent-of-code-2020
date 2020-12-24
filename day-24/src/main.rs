use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut floor = Floor::default();
    for line in input.trim().lines() {
        let tile = DirectionParser::new(line).fold(Tile::reference(), |tile, dir| tile.dir(dir));
        floor.flip(tile);
    }
    println!("{}", floor.count_blacks());

    for _ in 0..100 {
        floor = floor.generation();
    }
    println!("{}", floor.count_blacks());
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    NW,
    NE,
    E,
    SE,
    SW,
    W,
}

#[derive(Clone, Debug)]
pub struct DirectionParser<'a> {
    input: std::str::Chars<'a>,
}

impl<'a> DirectionParser<'a> {
    fn new(input: &'a str) -> Self {
        DirectionParser {
            input: input.chars(),
        }
    }
}

impl<'a> Iterator for DirectionParser<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input.next()? {
            'e' => Some(Direction::E),
            'w' => Some(Direction::W),
            'n' => match self.input.next()? {
                'e' => Some(Direction::NE),
                'w' => Some(Direction::NW),
                _ => None,
            },
            's' => match self.input.next()? {
                'e' => Some(Direction::SE),
                'w' => Some(Direction::SW),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tile {
    x: isize,
    y: isize,
}

impl Tile {
    fn new(x: isize, y: isize) -> Self {
        debug_assert!((x + y) % 2 == 0);
        Self { x, y }
    }

    pub fn reference() -> Self {
        Tile::new(0, 0)
    }

    pub fn dir(&self, dir: Direction) -> Self {
        use Direction::*;
        match dir {
            NW => Tile::new(self.x - 1, self.y - 1),
            NE => Tile::new(self.x + 1, self.y - 1),
            E => Tile::new(self.x + 2, self.y),
            SE => Tile::new(self.x + 1, self.y + 1),
            SW => Tile::new(self.x - 1, self.y + 1),
            W => Tile::new(self.x - 2, self.y),
        }
    }

    pub fn neighbors(&self) -> NeighborIterator {
        NeighborIterator::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NeighborIterator {
    tile: Tile,
    next: Option<Direction>,
}

impl NeighborIterator {
    fn new(tile: Tile) -> Self {
        NeighborIterator {
            tile,
            next: Some(Direction::NW),
        }
    }
}

impl Iterator for NeighborIterator {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        use Direction::*;
        let r = self.next;
        self.next = match r {
            Some(NW) => Some(NE),
            Some(NE) => Some(E),
            Some(E) => Some(SE),
            Some(SE) => Some(SW),
            Some(SW) => Some(W),
            Some(W) => None,
            None => None,
        };
        r.map(|r| self.tile.dir(r))
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

#[derive(Debug, Default)]
pub struct Floor {
    tiles: HashSet<Tile>,
}

impl Floor {
    fn flip(&mut self, tile: Tile) {
        if !self.tiles.insert(tile) {
            self.tiles.remove(&tile);
        }
    }

    fn count_blacks(&self) -> usize {
        self.tiles.len()
    }

    fn get(&self, tile: Tile) -> Color {
        if self.tiles.contains(&tile) {
            Color::Black
        } else {
            Color::White
        }
    }

    fn set(&mut self, tile: Tile, color: Color) {
        if color == Color::Black {
            self.tiles.insert(tile);
        } else {
            self.tiles.remove(&tile);
        }
    }

    fn generation(&self) -> Self {
        let mut next_day = Self::default();
        let mut consider_tiles: HashSet<Tile> = HashSet::new();
        for tile in &self.tiles {
            for neighbor in tile.neighbors() {
                consider_tiles.insert(neighbor);
            }
            consider_tiles.insert(*tile);
        }

        for tile in consider_tiles {
            let mut nearby_blacks = 0;
            for neighbor in tile.neighbors() {
                if self.get(neighbor) == Color::Black {
                    nearby_blacks += 1;
                }
            }
            let color = match (self.get(tile), nearby_blacks) {
                (Color::Black, 1) => Color::Black,
                (Color::Black, 2) => Color::Black,
                (Color::Black, _) => Color::White,
                (Color::White, 2) => Color::Black,
                (Color::White, _) => Color::White,
            };
            next_day.set(tile, color);
        }
        next_day
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile1() {
        use Direction::*;
        let t0 = Tile::reference();
        let t1 = t0.dir(NW).dir(W).dir(SW);
        let t2 = t0.dir(W).dir(W);

        assert_eq!(t1, t2);
    }

    #[test]
    fn parser() {
        use Direction::*;
        let mut parser = DirectionParser::new("esew");
        assert_eq!(parser.next(), Some(E));
        assert_eq!(parser.next(), Some(SE));
        assert_eq!(parser.next(), Some(W));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn neighbors() {
        use Direction::*;

        let tile = Tile::reference().dir(NW).dir(NW).dir(W).dir(NE);

        let neighbors: Vec<Tile> = tile.neighbors().collect();
        assert!(neighbors.contains(&tile.dir(NW)));
        assert!(neighbors.contains(&tile.dir(NE)));
        assert!(neighbors.contains(&tile.dir(E)));
        assert!(neighbors.contains(&tile.dir(SE)));
        assert!(neighbors.contains(&tile.dir(SW)));
        assert!(neighbors.contains(&tile.dir(W)));
        assert_eq!(neighbors.len(), 6);
    }
}
