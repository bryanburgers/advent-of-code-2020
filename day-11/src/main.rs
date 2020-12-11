use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut floorplan = Floorplan::parse(&input);
    loop {
        let new_floorplan = floorplan.step();
        if new_floorplan == floorplan {
            // Done
            break;
        } else {
            floorplan = new_floorplan;
        }
    }
    println!("{}", floorplan.count_occupied());
}

#[derive(Clone, Eq, PartialEq)]
struct Floorplan {
    width: usize,
    height: usize,
    seats: Vec<Seat>,
}

impl Floorplan {
    fn seat_at_point(&self, point: (usize, usize)) -> Seat {
        let (x, y) = point;
        if x >= self.width {
            panic!("Invalid point: x is out of range");
        }
        if y >= self.height {
            panic!("Invalid point: y is out of range");
        }

        let index = y * self.width + x;
        self.seats[index]
    }

    fn count_occupied(&self) -> usize {
        self.seats
            .iter()
            .filter(|seat| **seat == Seat::Occupied)
            .count()
    }

    fn step(&self) -> Self {
        let mut next_seats = self.seats.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let index = y * self.width + x;
                next_seats[index] = self.step_seat((x, y));
            }
        }

        Floorplan {
            width: self.width,
            height: self.height,
            seats: next_seats,
        }
    }

    fn step_seat(&self, point: (usize, usize)) -> Seat {
        let current_seat = self.seat_at_point(point);
        match current_seat {
            Seat::Floor => Seat::Floor,
            Seat::Empty => {
                if self
                    .surrounding_seats(point)
                    .into_iter()
                    .any(|seat| seat == Seat::Occupied)
                {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
            Seat::Occupied => {
                if self
                    .surrounding_seats(point)
                    .into_iter()
                    .filter(|seat| *seat == Seat::Occupied)
                    .count()
                    >= 4
                {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
        }
    }

    fn surrounding_seats(&self, point: (usize, usize)) -> Vec<Seat> {
        self.surrounding_points(point)
            .into_iter()
            .map(|point| self.seat_at_point(point))
            .collect()
    }

    fn surrounding_points(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut r = Vec::with_capacity(8);
        let (x, y) = point;

        if x > 0 && y > 0 {
            r.push((x - 1, y - 1)); // NW
        }
        if y > 0 {
            r.push((x, y - 1)); // N
        }
        if x < self.width - 1 && y > 0 {
            r.push((x + 1, y - 1)); // NE
        }

        if x > 0 {
            r.push((x - 1, y)); // W
        }
        if x < self.width - 1 {
            r.push((x + 1, y)); // E
        }

        if x > 0 && y < self.height - 1 {
            r.push((x - 1, y + 1)); // SW
        }
        if y < self.height - 1 {
            r.push((x, y + 1)); // S
        }
        if x < self.width - 1 && y < self.height - 1 {
            r.push((x + 1, y + 1)); // SE
        }

        r
    }

    fn parse(input: &str) -> Self {
        let lines = input.trim().split('\n');

        let mut seats = Vec::new();
        let mut width = None;
        let mut height = 0;
        for line in lines {
            let line = line.trim();
            let mut w = 0;
            for ch in line.chars() {
                let seat = match ch {
                    '.' => Seat::Floor,
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    _ => panic!("Unknown character '{}'", ch),
                };
                w += 1;
                seats.push(seat);
            }

            if let Some(width) = width {
                if w != width {
                    panic!("Mismatched line lengths!");
                }
            } else {
                width = Some(w);
            }
            height += 1;
        }

        Floorplan {
            seats,
            width: width.unwrap_or(0),
            height,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl std::fmt::Debug for Floorplan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let seat = self.seat_at_point((x, y));
                match seat {
                    Seat::Empty => f.write_str("L")?,
                    Seat::Occupied => f.write_str("#")?,
                    Seat::Floor => f.write_str(".")?,
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}
