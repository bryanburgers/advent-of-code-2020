use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let numbers: Vec<_> = lines
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect();

    for n in &numbers {
        for m in &numbers {
            if n + m == 2020 {
                println!("n = {}, m = {}, n*m = {}", n, m, n * m);
            }
        }
    }

    for l in &numbers {
        for n in &numbers {
            for m in &numbers {
                if l + n + m == 2020 {
                    println!("l = {}, n = {}, m = {}, l*n*m = {}", l, n, m, l * n * m);
                }
            }
        }
    }
}
