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
    let mut answer = None;
    for number in &numbers {
        let number = *number;
        if buffer.len() < prelude_size {
            buffer.push_back(number);
            continue;
        }

        let valid = is_valid(number, &buffer);
        if !valid {
            println!("{}", number);
            answer = Some(number);
            break;
        } else {
            buffer.pop_front();
            buffer.push_back(number);
        }
    }

    if let Some(answer) = answer {
        if let Some(r) = find_weakness(answer, &numbers) {
            println!("{}", r);
        }
    }
}

fn is_valid(n: i64, deque: &VecDeque<i64>) -> bool {
    let mut set = HashSet::new();
    for m in deque {
        if set.contains(&(n - m)) {
            return true;
        }
        set.insert(m);
    }

    false
}

fn find_weakness(n: i64, numbers: &[i64]) -> Option<i64> {
    let len = numbers.len();
    let mut start_index = 0;
    let mut end_index = 0;
    let mut sum = numbers[0];

    loop {
        if sum == n {
            let mut smallest = std::i64::MAX;
            let mut largest = std::i64::MIN;
            for number in &numbers[start_index..=end_index] {
                if *number < smallest {
                    smallest = *number;
                }
                if *number > largest {
                    largest = *number;
                }
            }
            return Some(smallest + largest);
        }
        if sum < n {
            end_index += 1;
            if end_index >= len {
                break;
            }
            sum += numbers[end_index];
        } else if sum > n {
            sum -= numbers[start_index];
            start_index += 1;
            if start_index > end_index {
                break;
            }
        }
    }

    None
}
