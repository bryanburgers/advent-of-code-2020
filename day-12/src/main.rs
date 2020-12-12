use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let actions: Vec<Action> = input
        .trim()
        .split('\n')
        .map(|s| Action::parse(s.trim()))
        .collect();

    let mut ferry_state = FerryState::default();
    for action in actions {
        ferry_state.apply(action);
    }
    let manhatten_distance = ferry_state.x.abs() + ferry_state.y.abs();
    println!("{}", manhatten_distance);
}

#[derive(Clone, Copy, Debug)]
struct FerryState {
    x: i64,
    y: i64,
    direction: Direction,
}

impl Default for FerryState {
    fn default() -> Self {
        FerryState {
            x: 0,
            y: 0,
            direction: Direction::East,
        }
    }
}

impl FerryState {
    fn apply(&mut self, action: Action) {
        match action {
            Action::MoveDirection(direction, len) => self.move_direction(direction, len),
            Action::Forward(len) => self.move_direction(self.direction, len),
            Action::RotateLeft(rotation) => {
                self.direction = self.direction.rotate_left(rotation);
            }
            Action::RotateRight(rotation) => {
                self.direction = self.direction.rotate_right(rotation);
            }
        }
    }

    fn move_direction(&mut self, direction: Direction, len: i64) {
        match direction {
            Direction::North => {
                self.y -= len;
            }
            Direction::South => {
                self.y += len;
            }
            Direction::West => {
                self.x -= len;
            }
            Direction::East => {
                self.x += len;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Action {
    MoveDirection(Direction, i64),
    Forward(i64),
    RotateLeft(Rotation),
    RotateRight(Rotation),
}

impl Action {
    fn parse(s: &str) -> Action {
        let number = (&s[1..]).parse::<i64>().expect("Number after letter");
        match &s[0..1] {
            "F" => Action::Forward(number),
            "N" => Action::MoveDirection(Direction::North, number),
            "S" => Action::MoveDirection(Direction::South, number),
            "E" => Action::MoveDirection(Direction::East, number),
            "W" => Action::MoveDirection(Direction::West, number),
            "R" => Action::RotateRight(match number {
                90 => Rotation::Ninety,
                180 => Rotation::OneEighty,
                270 => Rotation::TwoSeventy,
                _ => panic!("Invalid rotation {}", number),
            }),
            "L" => Action::RotateLeft(match number {
                90 => Rotation::Ninety,
                180 => Rotation::OneEighty,
                270 => Rotation::TwoSeventy,
                _ => panic!("Invalid rotation {}", number),
            }),
            c => panic!("Invalid character {}", c),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_right(self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Ninety => match self {
                Self::North => Self::East,
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
            },
            Rotation::OneEighty => self
                .rotate_right(Rotation::Ninety)
                .rotate_right(Rotation::Ninety),
            Rotation::TwoSeventy => self
                .rotate_right(Rotation::Ninety)
                .rotate_right(Rotation::Ninety)
                .rotate_right(Rotation::Ninety),
        }
    }

    fn rotate_left(self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Ninety => match self {
                Self::North => Self::West,
                Self::West => Self::South,
                Self::South => Self::East,
                Self::East => Self::North,
            },
            Rotation::OneEighty => self
                .rotate_left(Rotation::Ninety)
                .rotate_left(Rotation::Ninety),
            Rotation::TwoSeventy => self
                .rotate_left(Rotation::Ninety)
                .rotate_left(Rotation::Ninety)
                .rotate_left(Rotation::Ninety),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Ninety,
    OneEighty,
    TwoSeventy,
}
