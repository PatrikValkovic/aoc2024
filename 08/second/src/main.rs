fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines.iter().flat_map(|x| x.chars()).collect::<Vec<char>>();
    let mut signals = vec![false; width * height];

    // create dictionary from antennas
    let mut antennas = std::collections::HashMap::new();
    for i in 0..width*height {
        let c = grid[i];
        if c == '.' {
            continue
        }
        if !antennas.contains_key(&c) {
            antennas.insert(c, Vec::new());
        }
        antennas.get_mut(&c).unwrap().push(i);
    }

    // calculate antinodes
    for (_, v) in antennas.iter() {
        for i in 1..v.len() {
            for j in 0..i {
                let i1 = v[i];
                let i2 = v[j];
                let y1 = (i1 / width) as f32;
                let x1 = (i1 % width) as f32;
                let y2 = (i2 / width) as f32;
                let x2 = (i2 % width) as f32;
                let a = (y2 - y1) / (x2 - x1);
                let b = y1 - a * x1;
                let d = f32::abs(x2 - x1);
                let start_x = f32::min(x1, x2);
                let min_index = -f32::floor(start_x / d) as i32;
                let max_index = f32::ceil((width as f32 - start_x) / d) as i32;
                for step_size in min_index..max_index {
                    let x = (start_x + step_size as f32 * d).round();
                    let y = (a * x + b).round();
                    if x >= 0.0 && x < width as f32 && y >= 0.0 && y < height as f32 {
                        let left_x = x as usize;
                        let left_y = y as usize;
                        signals[left_y * width + left_x] = true;
                    }
                }
            }
        }
    }

    // count signals
    let count = signals.iter().filter(|&&x| x).count();

    println!("{}", count);
}
