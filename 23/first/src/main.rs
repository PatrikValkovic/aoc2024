use std::collections::HashMap;

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines().map(|line| {
        let mut parts = line.split("-");
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        return (a, b);
    });

    let mut nodes_indexes = HashMap::new();
    let mut node_index_to_name = vec![];
    let mut current_index = 0;
    for (a, b) in lines.clone() {
        nodes_indexes.entry(a).or_insert_with(|| {
            let tmp = current_index;
            current_index += 1;
            node_index_to_name.push(a);
            return tmp;
        });
        nodes_indexes.entry(b).or_insert_with(|| {
            let tmp = current_index;
            current_index += 1;
            node_index_to_name.push(b);
            return tmp;
        });
    }
    let n_nodes = nodes_indexes.len();

    let mut matrix = vec![0; n_nodes * n_nodes];
    for (a, b) in lines.clone() {
        let a = nodes_indexes.get(a).unwrap();
        let b = nodes_indexes.get(b).unwrap();

        matrix[*a * n_nodes + *b] = 1;
        matrix[*b * n_nodes + *a] = 1;
    }

    let mut sum = 0;

    for i in 0..n_nodes {
        for j in 0..i {
            for k in 0..j {
                if !node_index_to_name[i].starts_with("t")
                    && !node_index_to_name[j].starts_with("t")
                    && !node_index_to_name[k].starts_with("t")
                {
                    continue;
                }

                if matrix[i * n_nodes + j] == 1
                    && matrix[j * n_nodes + k] == 1
                    && matrix[k * n_nodes + i] == 1
                {
                    sum += 1;
                }
            }
        }
    }

    println!("{}", sum);
}
