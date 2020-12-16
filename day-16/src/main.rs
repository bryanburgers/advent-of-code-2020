use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (ticket_fields, _ticket, tickets) = parse_input(&input);

    let mut invalid_sum = 0;
    for ticket in &tickets {
        for value in &ticket.0 {
            if !ticket_fields.valid(*value) {
                invalid_sum += value;
            }
        }
    }
    println!("{}", invalid_sum);
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
