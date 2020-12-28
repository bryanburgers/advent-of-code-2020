use crate::transform::Transform;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tile {
    inner: u128,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            inner: 0,
        }
    }
}

impl Tile {
    pub fn transform(&self, transform: Transform) -> Self {
        match transform {
            Transform::R0 => *self,
            Transform::R1 => self.rotate(),
            Transform::R2 => self.rotate().rotate(),
            Transform::R3 => self.rotate().rotate().rotate(),
            Transform::MR0 => self.mirror().transform(Transform::R0),
            Transform::MR1 => self.mirror().transform(Transform::R1),
            Transform::MR2 => self.mirror().transform(Transform::R2),
            Transform::MR3 => self.mirror().transform(Transform::R3),
        }
    }

    fn rotate(&self) -> Self {
        let mut tile = Tile::default();
        for x in 0..10 {
            for y in 0..10 {
                let new_x = 9 - y;
                let new_y = x;
                tile.set(new_x, new_y, self.get(x, y));
            }
        }
        tile
    }

    fn mirror(&self) -> Self {
        let mut tile = Tile::default();
        for x in 0..10 {
            for y in 0..10 {
                let new_x = 9 - x;
                let new_y = y;
                tile.set(new_x, new_y, self.get(x, y));
            }
        }
        tile
    }

    pub fn top_border(&self) -> TileBorder {
        let mut border = TileBorder::default();
        for x in 0..10 {
            border.set(x, self.get(x, 0));
        }
        border
    }

    pub fn bottom_border(&self) -> TileBorder {
        let mut border = TileBorder::default();
        for x in 0..10 {
            border.set(x, self.get(x, 9));
        }
        border
    }

    pub fn left_border(&self) -> TileBorder {
        let mut border = TileBorder::default();
        for y in 0..10 {
            border.set(y, self.get(0, y));
        }
        border
    }

    pub fn right_border(&self) -> TileBorder {
        let mut border = TileBorder::default();
        for y in 0..10 {
            border.set(y, self.get(9, y));
        }
        border
    }
}

impl Tile {
    fn idx(&self, x: usize, y: usize) -> usize {
        y * 10 + x
    }
    fn set(&mut self, x: usize, y: usize, v: bool) {
        let idx = self.idx(x, y);
        let mask = (1 << 100) - 1;

        if v {
            self.inner |= 1 << idx;
        } else {
            self.inner &= !(1 << idx) & mask;
        }

        // self.inner[x][y] = v;
    }

    fn get(&self, x: usize, y: usize) -> bool {
        let idx = self.idx(x, y);
        (self.inner & (1 << idx)) > 0
        // self.inner[x][y]
    }
}

impl std::str::FromStr for Tile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tile = Tile::default();
        for (y, line) in s.trim().lines().enumerate() {
            if y >= 10 {
                return Err("x >= 10");
            }

            for (x, ch) in line.trim().chars().enumerate() {
                if x >= 10 {
                    return Err("x >= 10");
                }

                match ch {
                    '.' => tile.set(x, y, false),
                    '#' => tile.set(x, y, true),
                    _ => return Err("Invalid character"),
                }
            }
        }
        Ok(tile)
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..10 {
            for x in 0..10 {
                match self.get(x, y) {
                    true => f.write_str("#")?,
                    false => f.write_str(".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct TileBorder {
    inner: u16,
}

impl Default for TileBorder {
    fn default() -> Self {
        TileBorder {
            inner: 0
        }
    }
}

impl TileBorder {
    fn set(&mut self, x: usize, v: bool) {
        let mask = (1 << 10) - 1;
        if v {
            self.inner |= 1 << x;
        } else {
            self.inner &= !(1 << x) & mask;
        }
    }

    fn get(&self, x: usize) -> bool {
        (self.inner & (1 << x)) > 0
    }
}

impl std::fmt::Debug for TileBorder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..10 {
            match self.get(x) {
                true => f.write_str("#")?,
                false => f.write_str(".")?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###
        "#;

        let tile1 = input.parse::<Tile>().unwrap();
        let tile2 = input.parse::<Tile>().unwrap();

        assert_eq!(tile1, tile2);
        assert_ne!(tile1, Tile::default());
    }

    #[test]
    fn test_rotate() {
        let mut r0 = Tile::default();
        r0.set(2, 1, true);
        // ..........
        // ..#.......

        let mut r1 = Tile::default();
        r1.set(8, 2, true);
        // ..........
        // ..........
        // ........#.

        let mut r2 = Tile::default();
        r2.set(7, 8, true);
        //     ...
        // .......#..
        // ..........

        let mut r3 = Tile::default();
        r3.set(1, 7, true);
        //     ...
        // .#........
        // ..........
        // ..........

        assert_eq!(r0.rotate(), r1);
        assert_eq!(r0.rotate().rotate(), r2);
        assert_eq!(r0.rotate().rotate().rotate(), r3);
        assert_eq!(r0.rotate().rotate().rotate().rotate(), r0);
    }

    #[test]
    fn test_mirror() {
        let mut m0 = Tile::default();
        m0.set(4, 0, true);
        m0.set(2, 1, true);
        // ....#.....
        // ..#.......

        let mut m1 = Tile::default();
        m1.set(5, 0, true);
        m1.set(7, 1, true);
        // .....#....
        // .......#..

        assert_eq!(m0.mirror(), m1);
        assert_eq!(m0.mirror().mirror(), m0);
    }

    #[test]
    fn test_borders() {
        let input = r#"
        .###....#.
        .........#
        .........#
        .........#
        .........#
        #.........
        #.........
        #.........
        #.........
        .###....#.
        "#;

        let tile = input.parse::<Tile>().unwrap();
        assert_eq!(tile.top_border(), tile.bottom_border());
        assert_ne!(tile.left_border(), tile.right_border());
    }
}
