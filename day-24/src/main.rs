use std::collections::HashMap;
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
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn flip(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

#[derive(Debug, Default)]
pub struct Floor {
    tiles: HashMap<Tile, Color>,
}

impl Floor {
    fn flip(&mut self, tile: Tile) {
        let color = self.tiles.entry(tile).or_default();
        *color = color.flip();
    }

    fn count_blacks(&self) -> usize {
        self.tiles
            .values()
            .filter(|value| **value == Color::Black)
            .count()
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
}
