use std::io::Read;

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let hill = input.parse::<Hill>()?;

    let r = hill.trees_on_slope(3, 1)?;
    println!("{}", r);

    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut m = 1;
    for (x, y) in slopes {
        let r = hill.trees_on_slope(*x, *y)?;
        m *= r;
    }
    println!("{}", m);

    Ok(())
}

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(Debug)]
struct Hill {
    trees: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Hill {
    fn tree_at_point(&self, x: usize, y: usize) -> Result<bool, HillError> {
        if y > self.height {
            return Err(HillError::OutOfRange(x, y));
        }

        let wrapped_x = x % self.width;

        let row = self
            .trees
            .get(y)
            .ok_or_else(|| HillError::OutOfRange(x, y))?;
        let tree = row
            .get(wrapped_x)
            .ok_or_else(|| HillError::OutOfRange(x, y))?;

        Ok(*tree)
    }

    fn trees_on_slope(&self, x_slope: usize, y_slope: usize) -> Result<usize, HillError> {
        let mut trees = 0;
        // We don't count (0,0), so start (0+x_slope, 0+y_slope)
        let mut current_x = x_slope;
        let mut current_y = y_slope;

        while current_y < self.height {
            if self.tree_at_point(current_x, current_y)? {
                trees += 1;
            }
            current_x += x_slope;
            current_y += y_slope;
        }

        Ok(trees)
    }
}

#[derive(Debug, thiserror::Error)]
enum HillError {
    #[error("Out of range: {0}x{1} is out of range")]
    OutOfRange(usize, usize),
}

#[derive(Debug, thiserror::Error)]
enum HillParseError {
    #[error("Inconsistent width")]
    InconsistentWidth,
    #[error("Invalid character: {0}")]
    InvalidCharacter(char),
}

impl std::str::FromStr for Hill {
    type Err = HillParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().split('\n');

        let mut trees = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in lines {
            let chars = line.chars();
            let mut row = Vec::new();

            for ch in chars {
                match ch {
                    '.' => row.push(false),
                    '#' => row.push(true),
                    _ => return Err(HillParseError::InvalidCharacter(ch)),
                }
            }

            if width == 0 {
                width = row.len();
            } else if row.len() != width {
                return Err(HillParseError::InconsistentWidth);
            }

            trees.push(row);

            height += 1;
        }

        Ok(Hill {
            trees,
            width,
            height,
        })
    }
}
