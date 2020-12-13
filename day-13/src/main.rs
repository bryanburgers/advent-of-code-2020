use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.trim().split('\n');
    let line1 = lines.next().unwrap().trim();
    let line2 = lines.next().unwrap().trim();
    let earliest_departure = line1.parse::<usize>().unwrap();
    let bus_ids: Vec<BusId> = line2.split(',').map(|s| BusId::parse(s)).collect();

    let mut earliest_departure_time = None;
    let mut earliest_departure_id = None;
    for id in &bus_ids {
        if let Some(wait_time) = id.wait_time(earliest_departure) {
            if let Some(current_best) = earliest_departure_time {
                if wait_time < current_best {
                    earliest_departure_time = Some(wait_time);
                    earliest_departure_id = Some(*id);
                }
            } else {
                earliest_departure_time = Some(wait_time);
                earliest_departure_id = Some(*id);
            }
        }
    }
    if let Some(bus_id) = earliest_departure_id {
        let wait_time = bus_id.wait_time(earliest_departure).unwrap();
        let bus_id = match bus_id {
            BusId::Known(id) => id,
            _ => unreachable!(),
        };
        println!("{:?} => {}, {}", bus_id, wait_time, bus_id * wait_time);
    }

    let solution = find_solution(100000000000000, &bus_ids);
    println!("{}", solution);
}

#[derive(Debug, Clone, Copy)]
enum BusId {
    Known(usize),
    Unknown,
}

impl BusId {
    fn parse(s: &str) -> BusId {
        if s == "x" {
            BusId::Unknown
        } else {
            BusId::Known(s.parse().unwrap())
        }
    }

    fn wait_time(&self, earliest_departure: usize) -> Option<usize> {
        match self {
            BusId::Known(id) => Some(id - earliest_departure % id),
            BusId::Unknown => None,
        }
    }
}

fn check_solution(t: usize, offsets: &[BusOffset]) -> bool {
    offsets.iter().all(|bus| bus.valid(t))
}

fn find_solution(start_t: usize, buses: &[BusId]) -> usize {
    let offsets: Vec<BusOffset> = buses
        .iter()
        .zip(0..)
        .filter_map(|(bus_id, offset)| match bus_id {
            BusId::Known(id) => Some(BusOffset { offset, id: *id }),
            BusId::Unknown => None,
        })
        .collect();

    let mut t = start_t;
    loop {
        let mut next_offset = 1;
        if check_solution(t, &offsets) {
            return t;
        }

        for offset in &offsets {
            if offset.valid(t) {
                next_offset *= offset.id;
            }
        }

        t += next_offset;
    }
}

#[derive(Clone, Copy, Debug)]
struct BusOffset {
    offset: usize,
    id: usize,
}

impl From<(usize, usize)> for BusOffset {
    fn from(v: (usize, usize)) -> Self {
        Self {
            offset: v.0,
            id: v.1,
        }
    }
}

impl BusOffset {
    fn valid(&self, t: usize) -> bool {
        (t + self.offset) % self.id == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_offset_1() {
        let offsets: Vec<BusOffset> = vec![
            (0, 7).into(),
            (1, 13).into(),
            (4, 59).into(),
            (6, 31).into(),
            (7, 19).into(),
        ];
        assert_eq!(check_solution(1068781, &offsets), true)
    }

    #[test]
    fn check_offset_2() {
        let offsets: Vec<BusOffset> = vec![(0, 17).into(), (2, 13).into(), (3, 19).into()];
        assert_eq!(check_solution(3417, &offsets), true)
    }

    #[test]
    fn check_offset_3() {
        let offsets: Vec<BusOffset> = vec![
            (0, 67).into(),
            (1, 7).into(),
            (2, 59).into(),
            (3, 61).into(),
        ];
        assert_eq!(check_solution(754018, &offsets), true)
    }

    #[test]
    fn find_offset_1() {
        let buses = vec![
            BusId::Known(7),
            BusId::Known(13),
            BusId::Unknown,
            BusId::Unknown,
            BusId::Known(59),
            BusId::Unknown,
            BusId::Known(31),
            BusId::Known(19),
        ];
        assert_eq!(find_solution(0, &buses), 1068781);
    }

    #[test]
    fn find_offset_1_with_start() {
        let buses = vec![
            BusId::Known(7),
            BusId::Known(13),
            BusId::Unknown,
            BusId::Unknown,
            BusId::Known(59),
            BusId::Unknown,
            BusId::Known(31),
            BusId::Known(19),
        ];
        assert_eq!(find_solution(1060000, &buses), 1068781);
        assert_eq!(find_solution(1060001, &buses), 1068781);
    }

    #[test]
    fn find_offset_2() {
        let buses = vec![
            BusId::Known(17),
            BusId::Unknown,
            BusId::Known(13),
            BusId::Known(19),
        ];
        assert_eq!(find_solution(0, &buses), 3417);
    }

    #[test]
    fn find_offset_3() {
        let buses = vec![
            BusId::Known(67),
            BusId::Known(7),
            BusId::Known(59),
            BusId::Known(61),
        ];
        assert_eq!(find_solution(0, &buses), 754018);
    }

    #[test]
    fn find_offset_4() {
        let buses = vec![
            BusId::Known(67),
            BusId::Unknown,
            BusId::Known(7),
            BusId::Known(59),
            BusId::Known(61),
        ];
        assert_eq!(find_solution(0, &buses), 779210);
    }

    #[test]
    fn find_offset_5() {
        let buses = vec![
            BusId::Known(67),
            BusId::Known(7),
            BusId::Unknown,
            BusId::Known(59),
            BusId::Known(61),
        ];
        assert_eq!(find_solution(0, &buses), 1261476);
    }

    #[test]
    fn find_offset_6() {
        let buses = vec![
            BusId::Known(1789),
            BusId::Known(37),
            BusId::Known(47),
            BusId::Known(1889),
        ];
        assert_eq!(find_solution(0, &buses), 1202161486);
    }
}
