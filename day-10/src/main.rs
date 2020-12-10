use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut joltages = input
        .trim()
        .split('\n')
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    joltages.sort();

    let device_joltage = *joltages.last().unwrap_or(&0) + 3;

    let mut last_joltage = 0;
    let mut count_ones = 0;
    let mut count_threes = 0;
    for joltage in joltages {
        let jump = joltage - last_joltage;
        match jump {
            1 => {
                count_ones += 1;
            }
            2 => {}
            3 => {
                count_threes += 1;
            }
            _ => {
                panic!("jump from {} to {} was unexpected!", last_joltage, joltage)
            }
        }

        last_joltage = joltage;
    }
    // Jump from last voltage to device joltage is always 3 by definition
    count_threes += 1;
    println!(
        "ones={}, threes={}, result={}",
        count_ones,
        count_threes,
        count_ones * count_threes
    );
}
