use std::collections::HashMap;

const TOTAL_ITERATIONS: u32 = 25;

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let numbers: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // turn numbers int dictionary
    let mut counts: HashMap<u64, usize> = numbers
        .iter()
        .fold(HashMap::new(), |mut acc, &n| {
            *acc.entry(n).or_insert(0) += 1;
            acc
        });

    // run for given number of iterations
    for _ in 0..TOTAL_ITERATIONS {
        let mut new_counts = HashMap::new();
        for (number, count) in counts.iter() {
            let (number, count) = (*number, *count);
            // first rule
            if number == 0 {
                *new_counts.entry(1).or_insert(0) += count;
                continue;
            }
            // second rule
            let digits = number.ilog10() + 1;
            if digits % 2 == 0 {
                let half = digits / 2;
                let left = number / 10u64.pow(half);
                let right = number % 10u64.pow(half);
                *new_counts.entry(left).or_insert(0) += count;
                *new_counts.entry(right).or_insert(0) += count;
                continue;
            }
            // third rule
            *new_counts.entry(number * 2024).or_insert(0) += count;
        }
        counts = new_counts;
    }

    // sum counts
    let sum = counts.iter().fold(0, |acc, (_, count)| {
        acc + *count
    });

    println!("{}", sum);
}
