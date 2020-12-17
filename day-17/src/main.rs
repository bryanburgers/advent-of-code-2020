use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut life = parse_2d_into_life_3d(&input);
    for _ in 0..6 {
        life = life.step();
    }
    println!("{}", life.count_alive());

    let mut life = parse_2d_into_life_4d(&input);
    for _ in 0..6 {
        life = life.step();
    }
    println!("{}", life.count_alive());
}

fn parse_2d_into_life_3d(s: &str) -> Life3d {
    let mut life = Life3d::default();

    let mut y = 0;
    for line in s.trim().lines() {
        let mut x = 0;
        for ch in line.trim().chars() {
            match ch {
                '#' => { life.mark_alive(x, y, 0) },
                '.' => {},
                ch => panic!("Unexpected character {}", ch),
            }
            x += 1;
        }

        y += 1;
    }

    life
}

fn parse_2d_into_life_4d(s: &str) -> Life4d {
    let mut life = Life4d::default();

    let mut y = 0;
    for line in s.trim().lines() {
        let mut x = 0;
        for ch in line.trim().chars() {
            match ch {
                '#' => { life.mark_alive(x, y, 0, 0) },
                '.' => {},
                ch => panic!("Unexpected character {}", ch),
            }
            x += 1;
        }

        y += 1;
    }

    life
}

struct Life3d {
    x_range: (i64, i64),
    y_range: (i64, i64),
    z_range: (i64, i64),
    alive: HashSet<(i64, i64, i64)>,
}

impl Default for Life3d {
    fn default() -> Self {
        Self {
            x_range: (0, 0),
            y_range: (0, 0),
            z_range: (0, 0),
            alive: HashSet::default(),
        }
    }
}

impl Life3d {
    fn count_alive(&self) -> usize {
        self.alive.len()
    }

    fn is_alive(&self, x: i64, y: i64, z: i64) -> bool {
        self.alive.contains(&(x, y, z))
    }

    fn mark_alive(&mut self, x: i64, y: i64, z: i64) {
        self.alive.insert((x, y, z));
        if x < self.x_range.0 {
            self.x_range.0 = x;
        }
        if x > self.x_range.1 {
            self.x_range.1 = x;
        }
        if y < self.y_range.0 {
            self.y_range.0 = y;
        }
        if y > self.y_range.1 {
            self.y_range.1 = y;
        }
        if z < self.z_range.0 {
            self.z_range.0 = z;
        }
        if z > self.z_range.1 {
            self.z_range.1 = z;
        }
    }

    fn alive_next_generation(&self, x: i64, y: i64, z: i64) -> bool {
        let cube_iter = CubeIter::new(x, y, z);
        let mut alive_count = 0;
        for (x, y, z) in cube_iter {
            if self.is_alive(x, y, z) {
                alive_count += 1;
            }
        }

        match (self.is_alive(x, y, z), alive_count) {
            (true, 2) => true,
            (true, 3) => true,
            (false, 3) => true,
            _ => false,
        }
    }

    fn step(&self) -> Self {
        let mut new = Life3d::default();

        for x in (self.x_range.0-1)..=(self.x_range.1+1) {
            for y in (self.y_range.0-1)..=(self.y_range.1+1) {
                for z in (self.z_range.0-1)..=(self.z_range.1+1) {
                    if self.alive_next_generation(x, y, z) {
                        new.mark_alive(x, y, z);
                    }
                }
            }
        }

        new
    }
}

impl std::fmt::Debug for Life3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in self.z_range.0..=self.z_range.1 {
            writeln!(f, "z={}", z)?;
            for y in self.y_range.0..=self.y_range.1 {
                for x in self.x_range.0..=self.x_range.1 {
                    if self.is_alive(x, y, z) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
struct CubeIter {
    x: i64,
    y: i64,
    z: i64,

    offset_x: i64,
    offset_y: i64,
    offset_z: i64,
}

impl CubeIter {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        CubeIter {
            x: -1,
            y: -1,
            z: -1,

            offset_x: x,
            offset_y: y,
            offset_z: z,
        }
    }
    fn step(&mut self) -> bool {
        if self.z > 1 {
            return false;
        }
        self.x += 1;
        if self.x > 1 {
            self.x = -1;
            self.y += 1;
        }
        if self.y > 1 {
            self.y = -1;
            self.z += 1;
        }
        self.z <= 1
    }
}

impl Iterator for CubeIter {
    type Item = (i64, i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.z > 1 {
            return None;
        }
        if self.x == 0 && self.y == 0 && self.z == 0 {
            self.step();
        }
        let r = (self.x + self.offset_x, self.y + self.offset_y, self.z + self.offset_z);
        self.step();
        Some(r)
    }
}

struct Life4d {
    x_range: (i64, i64),
    y_range: (i64, i64),
    z_range: (i64, i64),
    w_range: (i64, i64),
    alive: HashSet<(i64, i64, i64, i64)>,
}

impl Default for Life4d {
    fn default() -> Self {
        Self {
            x_range: (0, 0),
            y_range: (0, 0),
            z_range: (0, 0),
            w_range: (0, 0),
            alive: HashSet::default(),
        }
    }
}

impl Life4d {
    fn count_alive(&self) -> usize {
        self.alive.len()
    }

    fn is_alive(&self, x: i64, y: i64, z: i64, w: i64) -> bool {
        self.alive.contains(&(x, y, z, w))
    }

    fn mark_alive(&mut self, x: i64, y: i64, z: i64, w: i64) {
        self.alive.insert((x, y, z, w));
        if x < self.x_range.0 {
            self.x_range.0 = x;
        }
        if x > self.x_range.1 {
            self.x_range.1 = x;
        }
        if y < self.y_range.0 {
            self.y_range.0 = y;
        }
        if y > self.y_range.1 {
            self.y_range.1 = y;
        }
        if z < self.z_range.0 {
            self.z_range.0 = z;
        }
        if z > self.z_range.1 {
            self.z_range.1 = z;
        }
        if w < self.w_range.0 {
            self.w_range.0 = w;
        }
        if w > self.w_range.1 {
            self.w_range.1 = w;
        }
    }

    fn alive_next_generation(&self, x: i64, y: i64, z: i64, w: i64) -> bool {
        let cube_iter = HyperCubeIter::new(x, y, z, w);
        let mut alive_count = 0;
        for (x, y, z, w) in cube_iter {
            if self.is_alive(x, y, z, w) {
                alive_count += 1;
            }
        }

        match (self.is_alive(x, y, z, w), alive_count) {
            (true, 2) => true,
            (true, 3) => true,
            (false, 3) => true,
            _ => false,
        }
    }

    fn step(&self) -> Self {
        let mut new = Life4d::default();

        for x in (self.x_range.0-1)..=(self.x_range.1+1) {
            for y in (self.y_range.0-1)..=(self.y_range.1+1) {
                for z in (self.z_range.0-1)..=(self.z_range.1+1) {
                    for w in (self.w_range.0-1)..=(self.w_range.1+1) {
                        if self.alive_next_generation(x, y, z, w) {
                            new.mark_alive(x, y, z, w);
                        }
                    }
                }
            }
        }

        new
    }
}

#[derive(Copy, Clone, Debug)]
struct HyperCubeIter {
    x: i64,
    y: i64,
    z: i64,
    w: i64,

    offset_x: i64,
    offset_y: i64,
    offset_z: i64,
    offset_w: i64,
}

impl HyperCubeIter {
    pub fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
        HyperCubeIter {
            x: -1,
            y: -1,
            z: -1,
            w: -1,

            offset_x: x,
            offset_y: y,
            offset_z: z,
            offset_w: w,
        }
    }
    fn step(&mut self) -> bool {
        if self.w > 1 {
            return false;
        }
        self.x += 1;
        if self.x > 1 {
            self.x = -1;
            self.y += 1;
        }
        if self.y > 1 {
            self.y = -1;
            self.z += 1;
        }
        if self.z > 1 {
            self.z = -1;
            self.w += 1;
        }
        self.w <= 1
    }
}

impl Iterator for HyperCubeIter {
    type Item = (i64, i64, i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.w > 1 {
            return None;
        }
        if self.x == 0 && self.y == 0 && self.z == 0 && self.w == 0 {
            self.step();
        }
        let r = (self.x + self.offset_x, self.y + self.offset_y, self.z + self.offset_z, self.w + self.offset_w);
        self.step();
        Some(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_iter() {
        let cube_iter = CubeIter::new(0, 0, 0);

        let hashset: HashSet<(i64, i64, i64)> = cube_iter.collect();
        assert_eq!(hashset.len(), 26);
        assert!(hashset.contains(&(-1, -1, -1)));
        assert!(hashset.contains(&(-1, 1, 0)));
        assert!(!hashset.contains(&(0, 0, 0)));

        let cube_iter = CubeIter::new(10, 10, 20);

        let hashset: HashSet<(i64, i64, i64)> = cube_iter.collect();
        assert_eq!(hashset.len(), 26);
        assert!(hashset.contains(&(9, 9, 19)));
        assert!(hashset.contains(&(9, 11, 20)));
        assert!(!hashset.contains(&(10, 10, 10)));
    }

    #[test]
    fn hyper_cube_iter() {
        let cube_iter = HyperCubeIter::new(0, 0, 0, 0);

        let hashset: HashSet<(i64, i64, i64, i64)> = cube_iter.collect();
        assert_eq!(hashset.len(), 80);
        assert!(hashset.contains(&(-1, -1, -1, -1)));
        assert!(hashset.contains(&(-1, 1, 0, 1)));
        assert!(!hashset.contains(&(0, 0, 0, 0)));
    }
}
