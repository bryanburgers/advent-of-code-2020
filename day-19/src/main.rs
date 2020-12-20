use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.trim().split("\n\n");
    let rules = iter.next().unwrap();
    let messages = iter.next().unwrap();

    let mut rules = Rules::parse(rules.trim()).unwrap();
    let r = messages
        .lines()
        .filter(|message| rules.is_match(message))
        .count();
    println!("{}", r);

    rules.r.insert(8, Rule::parse("42 | 42 8").unwrap());
    rules.r.insert(11, Rule::parse("42 31 | 42 11 31").unwrap());

    let r = messages
        .lines()
        .filter(|message| rules.is_match(message))
        .count();
    println!("{}", r);
}

#[derive(Clone, Debug)]
enum Rule<'a> {
    Terminal(&'a str),
    Ref(usize),
    Cons(Box<Rule<'a>>, Box<Rule<'a>>),
    Alt(Box<Rule<'a>>, Box<Rule<'a>>),
}

impl<'a> Rule<'a> {
    fn consume(&self, rules: &Rules, input: &str) -> Vec<usize> {
        let consumed = match self {
            Rule::Terminal(t) => {
                if t.len() > input.len() {
                    vec![]
                } else if *t == &input[0..t.len()] {
                    vec![t.len()]
                } else {
                    vec![]
                }
            }
            Rule::Ref(r) => {
                if let Some(rule) = rules.get_rule(*r) {
                    rule.consume(rules, input)
                } else {
                    vec![]
                }
            }
            Rule::Cons(r1, r2) => {
                let mut possibilities = Vec::new();

                for r1_consumed in r1.consume(rules, input) {
                    for r2_consumed in r2.consume(rules, &input[r1_consumed..]) {
                        possibilities.push(r1_consumed + r2_consumed);
                    }
                }

                possibilities.sort();
                possibilities.dedup();
                possibilities
            }
            Rule::Alt(r1, r2) => {
                let mut r1 = r1.consume(rules, input);
                let mut r2 = r2.consume(rules, input);
                r1.append(&mut r2);
                r1.sort();
                r1.dedup();
                r1
            }
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
            let mut refs = std::collections::VecDeque::new();
            for n in s.trim().split_whitespace() {
                let n = n
                    .parse::<usize>()
                    .map_err(|_| "Failed to parse reference")?;
                let r = Rule::Ref(n);
                refs.push_back(r);
            }

            let mut r = refs.pop_back().unwrap();
            while !refs.is_empty() {
                let n = refs.pop_back().unwrap();
                r = Rule::Cons(Box::new(n), Box::new(r));
            }

            Ok(r)
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

        let possibilities = rule.consume(&self, input);
        possibilities.iter().any(|p| *p == input.len())
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
        map.insert(
            0,
            Rule::Cons(Box::new(Rule::Ref(1)), Box::new(Rule::Ref(2))),
        );
        map.insert(1, Rule::Terminal("a"));
        map.insert(
            2,
            Rule::Alt(
                Box::new(Rule::Cons(Box::new(Rule::Ref(1)), Box::new(Rule::Ref(3)))),
                Box::new(Rule::Cons(Box::new(Rule::Ref(3)), Box::new(Rule::Ref(1)))),
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
                Box::new(Rule::Cons(Box::new(Rule::Ref(a)), Box::new(Rule::Ref(b)))),
                Box::new(Rule::Cons(Box::new(Rule::Ref(c)), Box::new(Rule::Ref(d)))),
            )
        }

        let mut map = HashMap::new();
        map.insert(
            0,
            Rule::Cons(
                Box::new(Rule::Ref(4)),
                Box::new(Rule::Cons(Box::new(Rule::Ref(1)), Box::new(Rule::Ref(5)))),
            ),
        );
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

    #[test]
    fn part_b_example() {
        let rules = r#"
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1
8: 42 | 42 8
11: 42 31 | 42 11 31
"#;
        let rules = Rules::parse(rules).unwrap();

        assert_eq!(
            rules.is_match("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa"),
            false
        );
        assert_eq!(rules.is_match("bbabbbbaabaabba"), true);
        assert_eq!(rules.is_match("babbbbaabbbbbabbbbbbaabaaabaaa"), true);
        assert_eq!(
            rules.is_match("aaabbbbbbaaaabaababaabababbabaaabbababababaaa"),
            true
        );
        assert_eq!(rules.is_match("bbbbbbbaaaabbbbaaabbabaaa"), true);
        assert_eq!(rules.is_match("bbbababbbbaaaaaaaabbababaaababaabab"), true);
        assert_eq!(rules.is_match("ababaaaaaabaaab"), true);
        assert_eq!(rules.is_match("ababaaaaabbbaba"), true);
        assert_eq!(rules.is_match("baabbaaaabbaaaababbaababb"), true);
        assert_eq!(rules.is_match("abbbbabbbbaaaababbbbbbaaaababb"), true);
        assert_eq!(rules.is_match("aaaaabbaabaaaaababaa"), true);
        assert_eq!(rules.is_match("aaaabbaaaabbaaa"), false);
        assert_eq!(rules.is_match("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"), true);
        assert_eq!(rules.is_match("babaaabbbaaabaababbaabababaaab"), false);
        assert_eq!(
            rules.is_match("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"),
            true
        );
    }
}
