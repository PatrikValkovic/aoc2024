use std::cmp::min;
use std::collections::{HashMap, HashSet};

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(",").map(|s| s.trim()).collect::<HashSet<_>>();
    let longest_pattern = patterns.iter().map(|s| s.len()).max().unwrap();
    lines.next();

    let mut cache = HashMap::new();
    cache.insert("", 1);

    let mut possible_patterns = 0;
    let mut pattern_variants = 0;
    for pattern in lines {
        let variants = is_possible(pattern, &patterns, longest_pattern, &mut cache);
        possible_patterns += (variants > 0) as u32;
        pattern_variants += variants;
    }

    println!("{}", possible_patterns);
    println!("{}", pattern_variants);
}

fn is_possible<'a>(pattern: &'a str, patterns: &HashSet<&str>, longest_pattern: usize, cache: &mut HashMap<&'a str, usize>) -> usize {
    let mut variants = 0;

    if cache.contains_key(pattern) {
        return cache[pattern];
    }

    let pattern_length = pattern.len();
    for split in 1..=min(pattern_length, longest_pattern) {
        let (left, right) = pattern.split_at(split);
        if !patterns.contains(left) {
            continue;
        }
        let right_side_variants = is_possible(right, patterns, longest_pattern, cache);
        variants += right_side_variants;
    }

    cache.insert(pattern, variants);
    return variants;
}