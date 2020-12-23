use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut parts = input.trim().split("\n\n");
    let hand1 = parts.next().unwrap();
    let hand2 = parts.next().unwrap();

    let mut hand1 = hand1
        .lines()
        .skip(1)
        .map(|l| l.parse())
        .collect::<Result<VecDeque<usize>, _>>()
        .unwrap();
    let mut hand2 = hand2
        .lines()
        .skip(1)
        .map(|l| l.parse())
        .collect::<Result<VecDeque<usize>, _>>()
        .unwrap();
    let hand1_partb = hand1.clone();
    let hand2_partb = hand2.clone();

    loop {
        if hand1.is_empty() || hand2.is_empty() {
            break;
        }

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
    println!("{} {}", score_hand(hand1), score_hand(hand2));

    let (score1, score2) = game_rec(hand1_partb, hand2_partb);
    println!("{} {}", score1, score2);
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

fn hash_hands(hand1: &VecDeque<usize>, hand2: &VecDeque<usize>) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;

    let mut hasher = DefaultHasher::new();

    hand1.hash(&mut hasher);
    hand2.hash(&mut hasher);

    hasher.finish()
}

fn game_rec(mut hand1: VecDeque<usize>, mut hand2: VecDeque<usize>) -> (usize, usize) {
    let mut states_seen = HashSet::new();

    loop {
        if hand1.is_empty() || hand2.is_empty() {
            break;
        }
        let game_state = hash_hands(&hand1, &hand2);
        if !states_seen.insert(game_state) {
            // Player 1 wins.
            return (1, 0);
        }

        let v1 = hand1.pop_front().unwrap();
        let v2 = hand2.pop_front().unwrap();

        let player1_wins;
        if hand1.len() >= v1 && hand2.len() >= v2 {
            // Recurse!
            let hand1_sub = hand1.iter().take(v1).copied().collect();
            let hand2_sub = hand2.iter().take(v2).copied().collect();
            let (s1, s2) = game_rec(hand1_sub, hand2_sub);
            player1_wins = s1 > s2;
        } else {
            player1_wins = v1 > v2;
        }

        if player1_wins {
            hand1.push_back(v1);
            hand1.push_back(v2);
        } else {
            hand2.push_back(v2);
            hand2.push_back(v1);
        }
    }

    (score_hand(hand1), score_hand(hand2))
}
