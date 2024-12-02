use std::iter::zip;

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();

    let mut safe_reports = 0;
    for line in lines {
        let levels = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let is_increasing = levels[0] < levels[1];

        let prev = levels.iter();
        let next = levels[1..].iter();

        let is_safe = zip(
            prev,
            next,
        ).all(|(prev, next)| {
            if is_increasing {
                return *prev < *next && *prev + 3 >= *next;
            } else {
                return *prev > *next && *prev <= *next + 3;
            }
        });

        safe_reports += is_safe as i32
    }

    println!("{}", safe_reports);
}
