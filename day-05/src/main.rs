use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut seats = input.trim().split('\n').map(|line| line.parse()).collect::<Result<Vec<Seat>, _>>().unwrap();

    seats.sort();

    println!("{}", seats.pop().unwrap().id());
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Seat(u32);

impl Seat {
    fn row(&self) -> u32 {
        self.0 >> 3
    }

    fn column(&self) -> u32 {
        self.0 & 7
    }

    fn id(&self) -> u32 {
        self.0
    }
}

impl std::str::FromStr for Seat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut val = 0;
        for ch in s.chars() {
            match ch {
                'F' => { val <<= 1; }
                'B' => { val <<= 1; val += 1; }
                'L' => { val <<= 1; }
                'R' => { val <<= 1; val += 1; }
                _ => {}
            }
        }
        Ok(Seat(val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let seat: Seat = "FBFBBFFRLR".parse().unwrap();
        assert_eq!(seat.row(), 44);
        assert_eq!(seat.column(), 5);
        assert_eq!(seat.id(), 357);
    }
}
