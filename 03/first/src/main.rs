fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();

    // define regex
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // find all the occurrences
    let mut sum = 0;
    for cap in re.captures_iter(&input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        sum += x * y;
    }

    println!("{}", sum);
}
