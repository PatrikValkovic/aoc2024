use std::iter::zip;

fn brute_force() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();

    let mut safe_reports = 0;

    for line in lines {
        let original_levels = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // decide if sequence is increasing or decreasing
        let prev_iter = original_levels.iter();
        let next_iter = original_levels[1..].iter();
        let increase_steps = zip(prev_iter.clone(), next_iter.clone())
            .map(|(prev, next)| return (*next - *prev > 0) as i32)
            .sum::<i32>();
        let decreasing_steps = zip(prev_iter.clone(), next_iter.clone())
            .map(|(prev, next)| return (*next - *prev < 0) as i32)
            .sum::<i32>();
        let is_increasing = increase_steps > decreasing_steps;

        // check if combination is safe
        let is_safe_combination = |prev: i32, next: i32| {
            return (is_increasing && prev < next && prev + 3 >= next) ||
                (!is_increasing && prev > next && prev - 3 <= next);
        };

        let mut is_safe = true;
        let mut dumpened_index = usize::MAX;
        for i in 1..original_levels.len() {
            let prev_index = if i - 1 == dumpened_index {
                i - 2
            } else {
                i - 1
            };
            let prev = original_levels[prev_index];
            let next = original_levels[i];
            if !is_safe_combination(prev, next) {
                if i == 1 {
                    let follow = original_levels[i + 1];
                    if is_safe_combination(prev, follow) {
                        // dump second number
                        dumpened_index = 1;
                    } else {
                        // dump beginning number
                        dumpened_index = 0;
                    }
                    continue
                }
                if dumpened_index != usize::MAX {
                    is_safe = false;
                    break;
                }
                let before = original_levels[i - 2];
                if is_safe_combination(before, next) {
                    // dump prev number
                    dumpened_index = i - 1;
                }
                else {
                    // dump current number
                    dumpened_index = i;
                }
            }
        }

        for remove_index in -1..original_levels.len() as i32 {
            let mut levels = original_levels.clone();
            if remove_index >= 0 {
                levels.remove(remove_index as usize);
            }
            let mut isSafe = true;
            let isIncreasing = levels[0] < levels[1];
            let mut i = 1;
            while i < levels.len() {
                if (isIncreasing && levels[i] <= levels[i - 1])
                    || (!isIncreasing && levels[i] >= levels[i - 1])
                {
                    isSafe = false;
                    break;
                }
                if levels[i] - levels[i - 1] > 3 || levels[i] - levels[i - 1] < -3 {
                    isSafe = false;
                    break;
                }
                i += 1;
            }
            if isSafe {
                if isSafe != is_safe {
                    println!("Mismatch ({}): {}", is_safe, original_levels.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
                }
                safe_reports += 1;
                break;
            }
        }
    }

    println!("{}", safe_reports);
}