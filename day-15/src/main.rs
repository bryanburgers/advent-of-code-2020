use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input = input
        .trim()
        .split(',')
        .map(|part| part.parse())
        .collect::<Result<Vec<usize>, _>>()
        .unwrap();

    let mut last_turn = HashMap::new();
    let mut current_turn = 1;
    let mut last_spoken_age = 0;
    let mut deque: VecDeque<usize> = input.iter().map(|r| *r).collect();
    let result = loop {
        let value = if let Some(start_value) = deque.pop_front() {
            start_value
        } else {
            last_spoken_age
        };

        let prev = last_turn.insert(value, current_turn);
        last_spoken_age = match prev {
            None => 0,
            Some(prev) => current_turn - prev,
        };

        if current_turn == 2020 {
            break value;
        }

        current_turn += 1;
    };

    println!("{}", result);
}
