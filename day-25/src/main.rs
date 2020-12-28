use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let public_key_card = lines.next().unwrap().parse().unwrap();
    let public_key_door = lines.next().unwrap().parse().unwrap();

    let iterator = LoopSizeCalculator::new(7);

    let mut loop_size_card = None;
    let mut loop_size_door = None;
    for (idx, v) in iterator.enumerate() {
        if v == public_key_card {
            loop_size_card = Some(idx);
        }
        if v == public_key_door {
            loop_size_door = Some(idx);
        }
        if loop_size_card.is_some() && loop_size_door.is_some() {
            break;
        }
    }
    let loop_size_card = loop_size_card.unwrap();
    let loop_size_door = loop_size_door.unwrap();

    println!(
        "card. public_key = {}, loop_size = {}",
        public_key_card, loop_size_card
    );
    println!(
        "door. public_key = {}, loop_size = {}",
        public_key_door, loop_size_door
    );

    println!(
        "encryption key = {}",
        LoopSizeCalculator::run(public_key_card, loop_size_door)
    );
    println!(
        "encryption key = {}",
        LoopSizeCalculator::run(public_key_door, loop_size_card)
    );
}

struct LoopSizeCalculator {
    value: u32,
    subject: u32,
}

impl LoopSizeCalculator {
    fn new(subject: u32) -> Self {
        LoopSizeCalculator { value: 1, subject }
    }

    fn run(subject: u32, loop_size: usize) -> u32 {
        let calc = LoopSizeCalculator::new(subject);
        calc.skip(loop_size).next().unwrap()
    }
}

impl Iterator for LoopSizeCalculator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.value;
        let v_64 = self.value as u64;
        let s_64 = self.subject as u64;
        let v_next = (v_64 * s_64) % 20201227;
        self.value = v_next as u32;
        Some(v)
    }
}
