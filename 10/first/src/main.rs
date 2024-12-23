/*
The topographic map indicates the height at each position using a scale from 0 (lowest) to 9 (highest). For example:

0123
1234
8765
9876
Based on un-scorched scraps of the book, you determine that a good hiking trail is as long as possible and has an even, gradual, uphill slope. For all practical purposes, this means that a hiking trail is any path that starts at height 0, ends at height 9, and always increases by a height of exactly 1 at each step. Hiking trails never include diagonal steps - only up, down, left, or right (from the perspective of the map).

You look up from the map and notice that the reindeer has helpfully begun to construct a small pile of pencils, markers, rulers, compasses, stickers, and other equipment you might need to update the map with hiking trails.

A trailhead is any position that starts one or more hiking trails - here, these positions will always have height 0. Assembling more fragments of pages, you establish that a trailhead's score is the number of 9-height positions reachable from that trailhead via a hiking trail. In the above example, the single trailhead in the top left corner has a score of 1 because it can reach a single 9 (the one in the bottom left).

This trailhead has a score of 2:

...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
(The positions marked . are impassable tiles to simplify these examples; they do not appear on your actual topographic map.)

This trailhead has a score of 4 because every 9 is reachable via a hiking trail except the one immediately to the left of the trailhead:

..90..9
...1.98
...2..7
6543456
765.987
876....
987....
This topographic map contains two trailheads; the trailhead at the top has a score of 1, while the trailhead at the bottom has a score of 2:

10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
Here's a larger example:

89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
This larger example has 9 trailheads. Considering the trailheads in reading order, they have scores of 5, 6, 5, 3, 1, 3, 5, 3, and 5. Adding these scores together, the sum of the scores of all trailheads is 36.

The reindeer gleefully carries over a protractor and adds it to the pile. What is the sum of the scores of all trailheads on your topographic map?


 */

fn main() {
    // read input
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut map: Vec<i8> = vec![0; height * width];
    for i in 0..height {
        for j in 0..width {
            let c = lines[i].chars().nth(j).unwrap();
            map[i * width + j] = c.to_digit(10).unwrap() as i8;
        }
    }

    let get_score = |start_x: usize, start_y: usize| -> usize {
        let mut visited = vec![false; height * width];
        visited[start_y * width + start_x] = true;

        let mut stack = vec![(start_x, start_y)];
        let mut score = 0;
        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();
            let h = map[y * width + x];
            if h == 9 {
                score += 1;
            }
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                let nh = map[ny * width + nx];
                if nh == h + 1 && !visited[ny * width + nx] {
                    visited[ny * width + nx] = true;
                    stack.push((nx, ny));
                }
            }
        };

        return score;
    };

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y * width + x] == 0 {
                sum += get_score(x, y);
            }
        }
    }

    println!("{}", sum);
}
