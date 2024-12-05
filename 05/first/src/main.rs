mod bruteforce;

use std::collections::{HashMap, HashSet};

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();

    // split input
    let empty_line_index = lines.clone().position(|x| x.is_empty()).unwrap();
    let rules_strings = lines.clone().take(empty_line_index).collect::<Vec<&str>>();
    let update_strings = lines.skip(empty_line_index + 1).collect::<Vec<&str>>();

    // process rules
    let mut rules = HashMap::new();
    for rule in rules_strings {
        let parts = rule.split("|").collect::<Vec<&str>>();
        let a = parts[0].parse::<i32>().unwrap();
        let b = parts[1].parse::<i32>().unwrap();
        if rules.get(&b).is_none() {
            rules.insert(b, HashSet::new());
        }
        rules.get_mut(&b).unwrap().insert(a);
    }

    // process updates
    let mut sum = 0;
    for update in update_strings {
        let pages = update
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut is_correct = true;
        let mut must_be_before = HashSet::new();
        for symbol in pages.iter() {
            if must_be_before.contains(symbol) {
                is_correct = false;
                break;
            }
            if rules.get(symbol).is_some() {
                for after in rules.get(symbol).unwrap() {
                    must_be_before.insert(after);
                }
            }
        }

        if is_correct {
            let middle = pages[pages.len() / 2];
            sum += middle;
        }
    }

    println!("{}", sum);
}
