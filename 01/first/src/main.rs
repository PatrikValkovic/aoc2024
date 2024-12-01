fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();

    // create two vectors to store the two lists
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    lines.for_each(|line| {
        let mut numbers = line.split("   ");
        let left = numbers.next().unwrap().parse::<i32>().unwrap();
        let right = numbers.next().unwrap().parse::<i32>().unwrap();
        left_list.push(left);
        right_list.push(right);
    });

    // sort each line
    left_list.sort();
    right_list.sort();

    // sum each diff
    let mut sum = 0;
    for i in 0..left_list.len() {
        sum += (left_list[i] - right_list[i]).abs();
    }

    println!("{}", sum);
}
