fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();

    let mut sum = 0;
    for line in lines {
        let first = line.chars().find(|c| c.is_digit(10)).unwrap();
        let last = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
        let value = format!("{}{}", first, last).parse::<i32>().unwrap();
        sum += value;
    }

    println!("{}", sum);
}
