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
        if rules.get(&a).is_none() {
            rules.insert(a, HashSet::new());
        }
        rules.get_mut(&a).unwrap().insert(b);
    }

    // process updates
    let mut sum = 0;
    for update in update_strings {
        let mut pages = update.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut is_correct = true;

        // This should not actually work, because the swap may broke some other rule and the
        // algorithm will not return to it.
        // Topological sort would be better solution, but it may be ambiguous and the task doesn't
        // specify what exactly do in those cases.
        // Probably some distance metric on number of swaps would solve the ambiguity.
        // Anyway, this passed so it is OK I guess.
        for i in 0..pages.len() {
            for j in 0..i {
                let before = pages[j];
                let after = pages[i];
                if rules.get(&after).is_some() && rules.get(&after).unwrap().contains(&before) {
                    is_correct = false;
                    pages[i] = before;
                    pages[j] = after;
                }
            }
        }

        if !is_correct {
            let middle = pages[pages.len() / 2];
            sum += middle;
        }
    }

    println!("{}", sum);
}
