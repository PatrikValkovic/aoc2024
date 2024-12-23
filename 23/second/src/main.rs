use std::collections::{HashMap, HashSet};
use std::ops::Sub;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let edges = input
        .lines()
        .map(|line| {
            let mut parts = line.split("-");
            let a = parts.next().unwrap();
            let b = parts.next().unwrap();
            return (a, b);
        })
        .collect::<Vec<_>>();

    let (nodes_to_index, node_index_to_name) = map_nodes_to_indexes(&edges);
    let n_nodes = nodes_to_index.len();

    let matrix = create_adjacency_matrix(&edges, nodes_to_index, n_nodes);

    let r = HashSet::with_capacity(n_nodes * 2);
    let p = (0..n_nodes).collect::<HashSet<_>>();
    let x = HashSet::with_capacity(n_nodes * 2);
    let max_clique = bron_kerbosch(r, p, x, n_nodes, &matrix);

    let mut station_names = max_clique
        .iter()
        .map(|&i| node_index_to_name[i])
        .collect::<Vec<_>>();
    station_names.sort();
    let password = station_names.join(",");
    println!("{}", password);
}

fn map_nodes_to_indexes<'a>(edges: &'a Vec<(&'a str, &'a str)>) -> (HashMap<&'a str, usize>, Vec<&'a str>) {
    let mut node_to_index = HashMap::new();
    let mut node_index_to_name = vec![];

    let mut current_index = 0;
    for (a, b) in edges.clone() {
        node_to_index.entry(a).or_insert_with(|| {
            let tmp = current_index;
            current_index += 1;
            node_index_to_name.push(a);
            return tmp;
        });
        node_to_index.entry(b).or_insert_with(|| {
            let tmp = current_index;
            current_index += 1;
            node_index_to_name.push(b);
            return tmp;
        });
    }

    (node_to_index, node_index_to_name)
}

fn create_adjacency_matrix(
    edges: &Vec<(&str, &str)>,
    nodes_indexes: HashMap<&str, usize>,
    n_nodes: usize,
) -> Vec<i32> {
    let mut matrix = vec![0; n_nodes * n_nodes];
    for (a, b) in edges.iter() {
        let a = *nodes_indexes.get(*a).unwrap();
        let b = *nodes_indexes.get(*b).unwrap();

        matrix[a * n_nodes + b] = 1;
        matrix[b * n_nodes + a] = 1;
    }
    matrix
}

fn bron_kerbosch(
    r: HashSet<usize>,
    mut p: HashSet<usize>,
    mut x: HashSet<usize>,
    n_nodes: usize,
    matrix: &Vec<i32>,
) -> HashSet<usize> {
    if p.is_empty() && x.is_empty() {
        return r;
    }

    let mut max_clique = HashSet::new();

    let pivot = p.union(&x).cloned().into_iter().next().unwrap();
    let pivot_neighbors = neighbors(pivot, matrix, n_nodes);

    for &v in p.sub(&pivot_neighbors).iter() {
        let mut new_r = r.clone();
        new_r.insert(v);
        let v_neighbors = neighbors(v, matrix, n_nodes);
        let new_p = set_intersection(&p, &v_neighbors);
        let new_x = set_intersection(&x, &v_neighbors);
        let clique = bron_kerbosch(new_r, new_p, new_x, n_nodes, matrix);
        p.remove(&v);
        x.insert(v);

        if clique.len() > max_clique.len() {
            max_clique = clique;
        }
    }

    max_clique
}

fn set_intersection(set: &HashSet<usize>, intersect: &HashSet<usize>) -> HashSet<usize> {
    set
        .intersection(&intersect)
        .cloned()
        .collect::<HashSet<_>>()
}

fn neighbors(node: usize, matrix: &Vec<i32>, n_nodes: usize) -> HashSet<usize> {
    matrix[node * n_nodes..node * n_nodes + n_nodes]
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if x == 1 { Some(i) } else { None })
        .collect::<HashSet<_>>()
}
