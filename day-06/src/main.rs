use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let groups = input
        .split("\n\n")
        .map(|block| block.parse())
        .collect::<Result<Vec<Group>, _>>()
        .unwrap();

    let r: usize = groups.iter().map(|group| group.total_yeses()).sum();
    println!("{}", r);

    let r: usize = groups.iter().map(|group| group.everyone_yeses()).sum();
    println!("{}", r);
}

#[derive(Clone, Debug)]
struct Group {
    forms: Vec<Form>,
}

impl Group {
    fn total_yeses(&self) -> usize {
        let all_yeses = self.forms.iter().fold(HashSet::new(), |hashset, form| {
            hashset.union(&form.answers).cloned().collect()
        });
        all_yeses.len()
    }

    fn everyone_yeses(&self) -> usize {
        let initial: HashSet<char> = ('a'..='z').collect();
        let everyone_yeses = self.forms.iter().fold(initial, |hashset, form| {
            hashset.intersection(&form.answers).cloned().collect()
        });
        everyone_yeses.len()
    }
}

impl std::str::FromStr for Group {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let forms = s
            .trim()
            .split('\n')
            .map(|form| form.parse())
            .collect::<Result<Vec<Form>, _>>()?;
        Ok(Self { forms })
    }
}

#[derive(Clone, Debug)]
struct Form {
    answers: HashSet<char>,
}

impl std::str::FromStr for Form {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let answers = s.trim().chars().collect();
        Ok(Self { answers })
    }
}
