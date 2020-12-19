use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.trim().split("\n\n");
    let rules = iter.next().unwrap();
    let messages = iter.next().unwrap();

    let rules = Rules::parse(rules.trim()).unwrap();
    let r = messages
        .lines()
        .filter(|message| rules.is_match(message))
        .count();
    println!("{}", r);
}

#[derive(Debug)]
enum Rule<'a> {
    Terminal(&'a str),
    Ref(usize),
    Seq(Vec<Rule<'a>>),
    Alt(Box<Rule<'a>>, Box<Rule<'a>>),
}

impl<'a> Rule<'a> {
    fn consume(&self, rules: &Rules, input: &str) -> Option<usize> {
        let consumed = match self {
            Rule::Terminal(t) => {
                if t.len() > input.len() {
                    None
                } else if *t == &input[0..t.len()] {
                    Some(t.len())
                } else {
                    None
                }
            }
            Rule::Ref(r) => {
                if let Some(rule) = rules.get_rule(*r) {
                    rule.consume(rules, input)
                } else {
                    None
                }
            }
            Rule::Seq(seq) => {
                let mut consumed = 0;
                for rule in seq {
                    if let Some(c) = rule.consume(rules, &input[consumed..]) {
                        consumed += c;
                    } else {
                        return None;
                    }
                }
                Some(consumed)
            }
            Rule::Alt(r1, r2) => r1
                .consume(rules, input)
                .or_else(|| r2.consume(rules, input)),
        };
        consumed
    }
}

impl<'a> Rule<'a> {
    fn parse(s: &'a str) -> Result<Self, &'static str> {
        let s = s.trim();
        if s.len() == 0 {
            return Err("Zero-length");
        }
        if &s[0..1] == "\"" {
            let idx = (&s[1..])
                .find('"')
                .ok_or("Couldn't find second quotation mark")?
                + 1;
            let terminal = &s[1..idx];
            return Ok(Rule::Terminal(terminal));
        }

        fn parse_seq(s: &str) -> Result<Rule, &'static str> {
            let mut refs = Vec::new();
            for n in s.trim().split_whitespace() {
                let n = n
                    .parse::<usize>()
                    .map_err(|_| "Failed to parse reference")?;
                let r = Rule::Ref(n);
                refs.push(r);
            }

            Ok(Rule::Seq(refs))
        }

        if let Some(idx) = s.find("|") {
            let alt1 = &s[0..idx];
            let alt2 = &s[(idx + 1)..];
            let alt1 = parse_seq(alt1)?;
            let alt2 = parse_seq(alt2)?;
            Ok(Rule::Alt(Box::new(alt1), Box::new(alt2)))
        } else {
            parse_seq(s)
        }
    }
}

#[derive(Debug)]
struct Rules<'a> {
    r: HashMap<usize, Rule<'a>>,
}

impl<'a> Rules<'a> {
    fn is_match(&self, input: &str) -> bool {
        let rule = Rule::Ref(0);
        /*

        let rule_0 = self.get_rule(0);
        if rule_0.is_none() {
            return false;
        }
        let rule_0 = rule_0.unwrap();
        */
        if let Some(consumed) = rule.consume(&self, input) {
            consumed == input.len()
        } else {
            false
        }
    }

    fn get_rule(&self, idx: usize) -> Option<&Rule> {
        self.r.get(&idx)
    }
}

impl<'a> Rules<'a> {
    fn parse(s: &'a str) -> Result<Self, &'static str> {
        let mut map = HashMap::new();
        for line in s.trim().lines() {
            let mut parts = line.splitn(2, ": ");
            let rule_number = parts.next().ok_or("Missing rule number")?;
            let rule_number = rule_number
                .parse::<usize>()
                .map_err(|_| "Failed to parse rule number")?;
            let rule = parts.next().ok_or("Missing rule")?;
            let rule = Rule::parse(rule)?;
            map.insert(rule_number, rule);
        }

        Ok(Rules { r: map })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut map = HashMap::new();
        map.insert(0, Rule::Seq(vec![Rule::Ref(1), Rule::Ref(2)]));
        map.insert(1, Rule::Terminal("a"));
        map.insert(
            2,
            Rule::Alt(
                Box::new(Rule::Seq(vec![Rule::Ref(1), Rule::Ref(3)])),
                Box::new(Rule::Seq(vec![Rule::Ref(3), Rule::Ref(1)])),
            ),
        );
        map.insert(3, Rule::Terminal("b"));

        let rules = Rules { r: map };

        assert_eq!(rules.is_match("aab"), true);
        assert_eq!(rules.is_match("aba"), true);

        assert_eq!(rules.is_match("aaa"), false);
        assert_eq!(rules.is_match("abb"), false);
    }

    #[test]
    fn example_2() {
        /*
        0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"

        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb
        */

        fn alt_seq(a: usize, b: usize, c: usize, d: usize) -> Rule<'static> {
            Rule::Alt(
                Box::new(Rule::Seq(vec![Rule::Ref(a), Rule::Ref(b)])),
                Box::new(Rule::Seq(vec![Rule::Ref(c), Rule::Ref(d)])),
            )
        }

        let mut map = HashMap::new();
        map.insert(0, Rule::Seq(vec![Rule::Ref(4), Rule::Ref(1), Rule::Ref(5)]));
        map.insert(1, alt_seq(2, 3, 3, 2));
        map.insert(2, alt_seq(4, 4, 5, 5));
        map.insert(3, alt_seq(4, 5, 5, 4));
        map.insert(4, Rule::Terminal("a"));
        map.insert(5, Rule::Terminal("b"));

        let rules = Rules { r: map };

        assert_eq!(rules.is_match("ababbb"), true);
        assert_eq!(rules.is_match("bababa"), false);
        assert_eq!(rules.is_match("abbbab"), true);
        assert_eq!(rules.is_match("aaabbb"), false);
        assert_eq!(rules.is_match("aaaabbb"), false);
        assert_eq!(rules.is_match("ab"), false);
    }
}
