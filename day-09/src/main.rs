use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let prelude_size = std::env::args()
        .skip(1)
        .next()
        .and_then(|n| n.parse::<usize>().ok())
        .unwrap_or(25);

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let numbers = input
        .trim()
        .split('\n')
        .map(|line| line.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut buffer = VecDeque::new();
    for number in numbers {
        if buffer.len() < prelude_size {
            buffer.push_back(number);
            continue;
        }

        let valid = is_valid(number, prelude_size, &buffer);
        if !valid {
            println!("{}", number);
            break;
        } else {
            buffer.pop_front();
            buffer.push_back(number);
        }
    }
}

fn is_valid(n: i64, size: usize, deque: &VecDeque<i64>) -> bool {
    let mut set = HashSet::new();
    for m in deque {
        if set.contains(&(n - m)) {
            return true;
        }
        set.insert(m);
    }

    false
}
