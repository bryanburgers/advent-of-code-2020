use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (ticket_fields, ticket, tickets) = parse_input(&input);

    let mut invalid_sum = 0;
    let mut valid_tickets = Vec::new();
    for ticket in tickets {
        let mut valid_ticket = true;
        for value in &ticket.0 {
            if !ticket_fields.valid(*value) {
                invalid_sum += value;
                valid_ticket = false;
            }
        }
        if valid_ticket {
            valid_tickets.push(ticket);
        }
    }
    println!("{}", invalid_sum);

    let mut possible_positions = PossiblePositions::new(&ticket_fields, ticket.0.len());
    for ticket in &valid_tickets {
        for (idx, ticket_value) in ticket.0.iter().enumerate() {
            for (ticket_field_name, ticket_field) in &ticket_fields.0 {
                if !ticket_field.valid(*ticket_value) {
                    possible_positions.remove_possibility(ticket_field_name, idx);
                }
            }
        }
    }
    let mut product = 1;
    for (field, positions) in possible_positions.0 {
        if field.contains("departure") {
            if let Some(position) = positions.the_remaining() {
                let value = ticket.0[position];
                product *= value;
            }
        }
    }
    println!("{:?}", product);
}

fn parse_input(input: &str) -> (TicketFields, Ticket, Vec<Ticket>) {
    let mut parts = input.split("\n\n");
    let first_part = parts.next().unwrap();
    let ticket_fields = first_part.parse().unwrap();

    let second_part = parts.next().unwrap();
    let after_header = &second_part[(second_part.find('\n').unwrap() + 1)..];
    let ticket = after_header.trim().parse().unwrap();

    let third_part = parts.next().unwrap();
    let tickets = third_part
        .trim()
        .lines()
        .skip(1)
        .map(|line| line.parse())
        .collect::<Result<Vec<Ticket>, _>>()
        .unwrap();

    (ticket_fields, ticket, tickets)
}

#[derive(Debug)]
struct Ticket(Vec<usize>);

impl std::str::FromStr for Ticket {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s
            .trim()
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| "Fail")?;
        Ok(Self(vec))
    }
}

#[derive(Debug)]
struct TicketField {
    min0: usize,
    max0: usize,
    min1: usize,
    max1: usize,
}

impl TicketField {
    fn valid(&self, v: usize) -> bool {
        (self.min0 <= v && v <= self.max0) || (self.min1 <= v && v <= self.max1)
    }
}

impl std::str::FromStr for TicketField {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_dash = s.find('-').ok_or("Failed to find first dash")?;
        let min0 = &s[0..first_dash];
        let s = &s[(first_dash + 1)..];
        let or_text = s.find(" or ").ok_or("Failed to find or")?;
        let max0 = &s[0..or_text];
        let s = &s[(or_text + 4)..];
        let second_dash = s.find('-').ok_or("Failed to find second dash")?;
        let min1 = &s[0..second_dash];
        let max1 = &s[(second_dash + 1)..];

        let min0 = min0
            .parse::<usize>()
            .map_err(|_| "Failed to parse min0 as a number")?;
        let max0 = max0
            .parse::<usize>()
            .map_err(|_| "Failed to parse max0 as a number")?;
        let min1 = min1
            .parse::<usize>()
            .map_err(|_| "Failed to parse min1 as a number")?;
        let max1 = max1
            .parse::<usize>()
            .map_err(|_| "Failed to parse max1 as a number")?;

        Ok(Self {
            min0,
            max0,
            min1,
            max1,
        })
    }
}

#[derive(Debug)]
struct TicketFields(HashMap<String, TicketField>);

impl TicketFields {
    fn valid(&self, v: usize) -> bool {
        for field in self.0.values() {
            if field.valid(v) {
                return true;
            }
        }
        false
    }
}

impl std::str::FromStr for TicketFields {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hashmap = HashMap::new();
        let lines = s.trim().split('\n');
        for line in lines {
            let line = line.trim();
            let colon = line.find(':').ok_or("Failed to find the colon")?;
            let name = line[0..colon].to_string();
            let field = (&line[(colon + 2)..]).parse()?;
            hashmap.insert(name, field);
        }
        Ok(Self(hashmap))
    }
}

#[derive(Debug)]
struct PossiblePositions(HashMap<String, Positions>);

impl PossiblePositions {
    fn new(fields: &TicketFields, len: usize) -> Self {
        let mut hashmap = HashMap::new();
        for field in fields.0.keys() {
            hashmap.insert(field.to_string(), Positions::from_len(len));
        }
        Self(hashmap)
    }

    fn remove_possibility(&mut self, field: &str, position: usize) {
        let remove_rest = if let Some(positions) = self.0.get_mut(field) {
            if positions.remove(position) {
                positions.the_remaining()
            } else {
                None
            }
        } else {
            None
        };

        let mut remove_stack: VecDeque<(String, usize)> = VecDeque::new();
        if let Some(remaining) = remove_rest {
            remove_stack.push_back((field.to_string(), remaining));
        }

        while let Some((field, position)) = remove_stack.pop_front() {
            for (field2, positions) in self.0.iter_mut() {
                if field2 == &field {
                    continue;
                }

                if positions.remove(position) {
                    if let Some(remaining) = positions.the_remaining() {
                        remove_stack.push_back((field2.to_string(), remaining));
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Positions(u32);

impl Positions {
    fn from_len(len: usize) -> Self {
        let val = (1 << len) - 1;
        Self(val as u32)
    }

    fn remove(&mut self, v: usize) -> bool {
        let mask = (1 << v) as u32;
        if self.0 & mask > 0 {
            self.0 &= !mask;
            true
        } else {
            false
        }
    }

    fn contains(&self, v: usize) -> bool {
        let mask = (1 << v) as u32;
        self.0 & mask > 0
    }

    fn the_remaining(&self) -> Option<usize> {
        if self.0.count_ones() != 1 {
            None
        } else {
            let mut r = 0;
            let mut n = self.0;
            while n != 1 {
                r += 1;
                n = n >> 1;
            }
            Some(r)
        }
    }
}

impl std::fmt::Debug for Positions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_first = true;
        write!(f, "{{ ")?;
        for i in 0..32 {
            if self.contains(i) {
                if is_first {
                    is_first = false;
                    write!(f, "{}", i)?;
                } else {
                    write!(f, ", {}", i)?;
                }
            }
        }
        write!(f, " }}")?;
        Ok(())
    }
}
