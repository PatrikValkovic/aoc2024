fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut lines = input.lines();

    let mut tokens_spend = 0;
    while true {
        // parse info for current claw machine
        let a_button_string = lines.next().unwrap();
        let b_button_string = lines.next().unwrap();
        let prize_string = lines.next().unwrap();
        let mut a_movements = a_button_string.split(":").nth(1).unwrap().split(",").map(|x| x.trim());
        let a_x_move: f64 = a_movements.next().unwrap().split('+').nth(1).unwrap().parse().unwrap();
        let a_y_move: f64 = a_movements.next().unwrap().split('+').nth(1).unwrap().parse().unwrap();
        let mut b_movements = b_button_string.split(":").nth(1).unwrap().split(",").map(|x| x.trim());
        let b_x_move: f64 = b_movements.next().unwrap().split('+').nth(1).unwrap().parse().unwrap();
        let b_y_move: f64 = b_movements.next().unwrap().split('+').nth(1).unwrap().parse().unwrap();
        let mut price_locations = prize_string.split(":").nth(1).unwrap().split(",").map(|x| x.trim());
        let price_x: f64 = price_locations.next().unwrap().split('=').nth(1).unwrap().parse().unwrap();
        let price_y: f64 = price_locations.next().unwrap().split('=').nth(1).unwrap().parse().unwrap();
        let price_x = price_x + 10000000000000_f64;
        let price_y = price_y + 10000000000000_f64;

        // solve equations
        let count_b =  (a_x_move*price_y - a_y_move * price_x)
            /
            (b_y_move * a_x_move - b_x_move * a_y_move);
        let count_a = (price_x - count_b * b_x_move) / a_x_move;
        let rounded_b = count_b.round();
        let rounded_a = count_a.round();

        // check results
        if count_b >= 0.0 &&
            count_b >= 0.0 &&
            f64::abs(count_a - rounded_a) <= f64::EPSILON &&
            f64::abs(count_b - rounded_b) <= f64::EPSILON
        {
            let count_b = count_b as u64;
            let count_a = count_a as u64;
            tokens_spend += count_a * 3 + count_b;
        }

        if let None = lines.next() {
            break;
        }
    }

    println!("{}", tokens_spend);
}
