use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut parts = input.trim().split("\n\n");
    let hand1 = parts.next().unwrap();
    let hand2 = parts.next().unwrap();

    let mut hand1 = hand1.lines().skip(1).map(|l| l.parse()).collect::<Result<VecDeque<usize>, _>>().unwrap();
    let mut hand2 = hand2.lines().skip(1).map(|l| l.parse()).collect::<Result<VecDeque<usize>, _>>().unwrap();

    let mut rounds = 0;
    loop {
        if hand1.is_empty() || hand2.is_empty() {
            break;
        }

        rounds += 1;
        let v1 = hand1.pop_front().unwrap();
        let v2 = hand2.pop_front().unwrap();
        if v1 > v2 {
            hand1.push_back(v1);
            hand1.push_back(v2);
        } else {
            hand2.push_back(v2);
            hand2.push_back(v1);
        }
    }
    println!("{}", rounds);
    println!("{}", score_hand(hand1));
    println!("{}", score_hand(hand2));
}

fn score_hand(mut i: VecDeque<usize>) -> usize {
    let mut sum = 0;
    let mut idx = 1;
    while let Some(v) = i.pop_back() {
        sum += idx * v;
        idx += 1;
    }
    sum
}
