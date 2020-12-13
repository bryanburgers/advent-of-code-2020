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
