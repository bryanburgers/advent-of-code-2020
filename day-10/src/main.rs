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
    joltages.push(device_joltage);

    let mut last_joltage = 0;
    let mut count_ones = 0;
    let mut count_threes = 0;
    let mut consecutive = Vec::new();
    let mut ways: usize = 1;
    for joltage in joltages {
        let jump = joltage - last_joltage;
        match jump {
            1 => {
                count_ones += 1;
                // We get multiple combinations of ways only when there are sequences of
                // consecutive jumps, or jumps by 1 joltage.
                // So keep track of the consecutive sequences.
                consecutive.push(last_joltage);
            }
            3 => {
                count_threes += 1;

                // This is no longer a consecutive sequence, so see if we did have a consecutive
                // sequence, and handle it.
                if !consecutive.is_empty() {
                    consecutive.push(last_joltage);
                    // By looking at the data (both the problem input and the two examples) the
                    // maximum number of consecutive items is 5. Manually figuring out how many
                    // different ways each consecutive list can get from the first to last yields
                    // this match.
                    //
                    // And since each consecutive list doesn't interact with any other, the total
                    // number of ways to get from 0 to the device input is the product of the
                    // number of ways for each individual consecutive run.
                    ways *= match consecutive.len() {
                        2 => 1,
                        3 => 2,
                        4 => 4,
                        5 => 7,
                        n => panic!("There's an unexpected span of {} consecutive jumps!", n),
                    };
                    consecutive.clear();
                }
            }
            _ => {
                panic!("jump from {} to {} was unexpected!", last_joltage, joltage)
            }
        }

        last_joltage = joltage;
    }

    println!(
        "ones={}, threes={}, result={}",
        count_ones,
        count_threes,
        count_ones * count_threes
    );
    println!("ways={}", ways);
}
