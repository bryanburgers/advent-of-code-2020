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

    let mut all_seen: HashSet<&str> = HashSet::new();
    let mut contains: HashMap<&str, Vec<&str>> = HashMap::new();

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
        if check_contains(item, "shiny gold", &contains) {
            total_seen += 1;
        }
    }

    println!("{}", total_seen);

    let mut contains: HashMap<&str, Vec<NumberedBag<'_>>> = HashMap::new();

    for input in &inputs {
        contains.insert(input.outer.clone(), input.inner.clone());
    }

    let r = inside_bag("shiny gold", &contains);
    println!("{}", r);
}

fn check_contains(color: &str, target: &str, map: &HashMap<&str, Vec<&str>>) -> bool {
    if let Some(colors) = map.get(color) {
        if colors.contains(&target) {
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

fn inside_bag(color: &str, map: &HashMap<&str, Vec<NumberedBag<'_>>>) -> usize {
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
struct Input<'a> {
    outer: &'a str,
    inner: Vec<NumberedBag<'a>>,
}

fn one_word<'a>(input: &'a str) -> (&'a str, Option<&'a str>) {
    if let Some(whitespace) = input.find(' ') {
        (&input[..whitespace], Some(&input[(whitespace + 1)..]))
    } else {
        (input, None)
    }
}

fn two_words<'a>(input: &'a str) -> (&'a str, Option<&'a str>) {
    let (first_word, rest) = one_word(input);
    if let Some(rest) = rest {
        let (second_word, rest) = one_word(rest);
        (&input[..(first_word.len() + 1 + second_word.len())], rest)
    } else {
        (first_word, rest)
    }
}

impl<'a> Input<'a> {
    fn parse(input: &'a str) -> Self {
        let (outer, rest) = two_words(input);
        let (_bags, rest) = one_word(rest.unwrap());
        let (_contain, rest) = one_word(rest.unwrap());

        let (next_word, _) = one_word(rest.unwrap());

        if next_word == "no" {
            return Input {
                outer,
                inner: vec![],
            };
        }

        let mut inner = Vec::new();
        let mut outer_rest = rest;
        while let Some(rest) = outer_rest {
            let (number, rest) = one_word(rest);
            let number = number.parse::<usize>().unwrap();
            let (color, rest) = two_words(rest.unwrap());
            let (_bag, rest) = one_word(rest.unwrap());
            outer_rest = rest;
            inner.push(NumberedBag { number, color });
        }

        Input { outer, inner }
    }
}

#[derive(Copy, Clone, Debug)]
struct NumberedBag<'a> {
    number: usize,
    color: &'a str,
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
