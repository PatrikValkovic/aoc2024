type SecretNumberType = u64;

const STEPS: usize = 2000;
const PRUNE: SecretNumberType = 16777216;

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();
    let mut secret_numbers = lines.map(|line| line.parse::<SecretNumberType>().unwrap()).collect::<Vec<_>>();

    for _ in 0..STEPS {
        secret_numbers = secret_numbers.iter().map(|n| {
            let mut tmp = *n;
            // first
            tmp = ((tmp << 6) ^ tmp) % PRUNE;
            // second
            tmp = ((tmp >> 5) ^ tmp) % PRUNE;
            // third
            tmp = ((tmp << 11) ^ tmp) % PRUNE;
            return tmp;
        }).collect();
    }

    println!("{}", secret_numbers.iter().sum::<SecretNumberType>());
}
