use std::collections::{HashMap, HashSet};
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let inputs = input
        .trim()
        .split('\n')
        .map(Input::parse)
        .collect::<Vec<_>>();

    let mut all_seen: HashSet<String> = HashSet::new();
    let mut contains: HashMap<String, Vec<String>> = HashMap::new();

    for input in &inputs {
        all_seen.insert(input.outer.clone());
        let mut seen = Vec::new();
        for item in &input.inner {
            all_seen.insert(item.color.clone());
            seen.push(item.color.clone());
        }
        contains.insert(input.outer.clone(), seen);
    }

    let mut total_seen = 0;
    for item in &all_seen {
        if check_contains(item, &String::from("shiny gold"), &contains) {
            total_seen += 1;
        }
    }

    println!("{}", total_seen);

    let mut contains: HashMap<String, Vec<NumberedBag>> = HashMap::new();

    for input in &inputs {
        contains.insert(input.outer.clone(), input.inner.clone());
    }

    let r = inside_bag(&String::from("shiny gold"), &contains);
    println!("{}", r);
}

fn check_contains(color: &String, target: &String, map: &HashMap<String, Vec<String>>) -> bool {
    if let Some(colors) = map.get(color) {
        if colors.contains(target) {
            true
        } else {
            colors
                .iter()
                .any(|color| check_contains(color, target, map))
        }
    } else {
        false
    }
}

fn inside_bag(color: &String, map: &HashMap<String, Vec<NumberedBag>>) -> usize {
    if let Some(bags) = map.get(color) {
        let mut total = 0;
        for bag in bags {
            let r = 1 + inside_bag(&bag.color, &map);
            total += bag.number * r;
        }
        total
    } else {
        0
    }
}

#[derive(Debug, Clone)]
struct Input {
    outer: String,
    inner: Vec<NumberedBag>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut parts = input.trim().split_whitespace();

        let c1 = parts.next().unwrap();
        let c2 = parts.next().unwrap();
        let outer = format!("{} {}", c1, c2);

        assert_eq!(parts.next(), Some("bags"));
        assert_eq!(parts.next(), Some("contain"));

        let mut next = parts.next();
        if next == Some("no") {
            return Input {
                outer,
                inner: vec![],
            };
        }

        let mut inner = Vec::new();
        loop {
            if let Some(part) = next {
                let number = part.parse::<usize>().unwrap();
                let c1 = parts.next().unwrap();
                let c2 = parts.next().unwrap();
                let color = format!("{} {}", c1, c2);
                inner.push(NumberedBag { number, color });
                parts.next().unwrap();
                next = parts.next();
            } else {
                break;
            }
        }

        Input { outer, inner }
    }
}

#[derive(Debug, Clone)]
struct NumberedBag {
    number: usize,
    color: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_1() {
        let input =
            "light red bags contain 1 bright white bag, 2 muted yellow bags, 3 bright orange bags.";
        let input = Input::parse(input);
        assert_eq!(input.outer, "light red");
        assert_eq!(input.inner.len(), 3);
        assert_eq!(input.inner[0].number, 1);
        assert_eq!(input.inner[0].color, "bright white");
    }

    #[test]
    fn input_2() {
        let input = "faded blue bags contain no other bags.";
        let input = Input::parse(input);
        assert_eq!(input.outer, "faded blue");
        assert_eq!(input.inner.len(), 0);
    }
}
