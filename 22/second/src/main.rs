use std::collections::{HashMap, HashSet};

type SecretNumberType = u64;

const STEPS: usize = 2000;
const PRUNE: SecretNumberType = 16777216;

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();
    let mut secret_numbers = lines
        .map(|line| line.parse::<SecretNumberType>().unwrap())
        .collect::<Vec<_>>();
    let sellers = secret_numbers.len();
    let mut selling = vec![0; sellers * (STEPS + 1)];
    let mut diff = vec![0; sellers * STEPS];

    for i in 0..sellers {
        selling[i] = secret_numbers[i] % 10;
    }

    for step in 0..STEPS {
        secret_numbers = secret_numbers
            .iter()
            .map(|n| {
                let mut tmp = *n;
                // first
                tmp = ((tmp << 6) ^ tmp) % PRUNE;
                // second
                tmp = ((tmp >> 5) ^ tmp) % PRUNE;
                // third
                tmp = ((tmp << 11) ^ tmp) % PRUNE;
                return tmp;
            })
            .collect();

        for seller in 0..secret_numbers.len() {
            selling[(step +1)*sellers + seller] = secret_numbers[seller] % 10;
            diff[step *sellers + seller] = selling[(step +1)*sellers + seller] as i8 - selling[step *sellers + seller] as i8;
        }
    }
    secret_numbers.clear();
    secret_numbers.shrink_to_fit();

    let mut prices_per_combination = HashMap::new();
    let mut visited_for_seller = HashSet::new();
    for seller in 0..sellers {
        visited_for_seller.clear();
        for step in 3..STEPS {
            let combination = (
                diff[(step - 3) * sellers + seller],
                diff[(step - 2) * sellers + seller],
                diff[(step - 1) * sellers + seller],
                diff[(step - 0) * sellers + seller],
            );
            let price = selling[(step+1) * sellers + seller];

            if visited_for_seller.contains(&combination) {
                continue;
            }

            visited_for_seller.insert(combination);
            *prices_per_combination.entry(combination).or_insert(0) += price;
        }
    }

    let max_entry = prices_per_combination.iter().max_by_key(|x| *x.1).unwrap();
    println!("{:?}: {}", max_entry.0, max_entry.1);
}
