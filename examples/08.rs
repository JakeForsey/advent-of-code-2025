use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let path = "input/08.txt";
    part1(path);
    part2(path);
}

fn part1(path: &str) {
    let n = 1000;

    // Load input
    let input = std::fs::read_to_string(path).unwrap();

    // Parse input to coords
    let mut coords = Vec::new();
    for line in input.split("\n") {
        let (x, yz) = line.split_once(",").unwrap();
        let (y, z) = yz.split_once(",").unwrap();
        let x: i64 = x.parse().unwrap();
        let y: i64 = y.parse().unwrap();
        let z: i64 = z.parse().unwrap();
        coords.push((x, y, z));
    }

    // Compute pairwise distances
    let mut distance_matrix = Vec::new();
    for (i, (x0, y0, z0)) in coords.iter().enumerate() {
        for (j, (x1, y1, z1)) in coords.iter().enumerate().skip(i + 1) {
            let dx = x0 - x1;
            let dy = y0 - y1;
            let dz = z0 - z1;
            let d = dx.pow(2) + dy.pow(2) + dz.pow(2);
            distance_matrix.push((d, i, j));
        }
    }
    distance_matrix.sort();

    // Build adjacency matrix by connecting the n closest nodes
    let mut nodes = HashSet::new();
    let mut adj = HashMap::new();
    for (_, i, j) in distance_matrix.iter().take(n) {
        nodes.insert(i);
        nodes.insert(j);
        adj.entry(i).or_insert(Vec::new()).push(j);
        adj.entry(j).or_insert(Vec::new()).push(i);
    }

    // Find connected components
    let mut components: HashMap<usize, i32> = HashMap::new();
    let mut component_id = 0;
    for seed_node in nodes {
        if components.contains_key(seed_node) {
            // Already found it...
        } else {
            let mut queue = VecDeque::new();
            queue.push_back(seed_node);
            while !queue.is_empty() {
                let i = queue.pop_front().unwrap();
                components.insert(*i, component_id);

                if let Some(neighbours) = adj.get(i) {
                    for neighbour in neighbours {
                        if components.contains_key(neighbour) {
                            continue;
                        }
                        components.insert(**neighbour, component_id);
                        queue.push_back(neighbour);
                    }
                }
            }
            component_id += 1;
        }
    }

    // Compute the size of each component
    let mut component_counts: HashMap<i32, i32> = HashMap::new();
    for (_node, id) in &components {
        let count = component_counts.entry(*id).or_default();
        *count += 1;
    }

    let mut counts: Vec<i32> = component_counts.iter().map(|x| *x.1).collect();
    counts.sort();
    let output = counts.iter().rev().take(3).product::<i32>();
    println!("part1:\n{output}");
}

fn part2(path: &str) {
    // Load input
    let input = std::fs::read_to_string(path).unwrap();

    // Parse input to coords
    let mut coords = Vec::new();
    for line in input.split("\n") {
        let (x, yz) = line.split_once(",").unwrap();
        let (y, z) = yz.split_once(",").unwrap();
        let x: i64 = x.parse().unwrap();
        let y: i64 = y.parse().unwrap();
        let z: i64 = z.parse().unwrap();
        coords.push((x, y, z));
    }

    // Compute pairwise distances
    let mut distance_matrix = Vec::new();
    for (i, (x0, y0, z0)) in coords.iter().enumerate() {
        for (j, (x1, y1, z1)) in coords.iter().enumerate().skip(i + 1) {
            let dx = x0 - x1;
            let dy = y0 - y1;
            let dz = z0 - z1;
            let d = dx.pow(2) + dy.pow(2) + dz.pow(2);
            distance_matrix.push((d, i, j));
        }
    }
    distance_matrix.sort();

    // Build adjacency matrix by connecting the n closest nodes
    let mut nodes = HashSet::new();
    for (_, i, j) in distance_matrix.iter() {
        nodes.insert(i);
        nodes.insert(j);
    }

    let mut adj = HashMap::new();
    let mut output = 0;
    for (_, i, j) in distance_matrix.iter() {
        adj.entry(i).or_insert(Vec::new()).push(j);
        adj.entry(j).or_insert(Vec::new()).push(i);

        if all_connected(nodes.clone(), adj.clone()) {
            let x0 = coords.get(*i).unwrap().0;
            let x1 = coords.get(*j).unwrap().0;
            output = x0 * x1;
            break;
        }
    }
    println!("part2:\n{output}");
}

fn all_connected(nodes: HashSet<&usize>, adj: HashMap<&usize, Vec<&usize>>) -> bool {
    let mut components: HashMap<usize, i32> = HashMap::new();
    let mut component_id = 0;
    for seed_node in nodes {
        if components.contains_key(seed_node) {
            // Already found it...
        } else {
            let mut queue = VecDeque::new();
            queue.push_back(seed_node);
            while !queue.is_empty() {
                let i = queue.pop_front().unwrap();
                components.insert(*i, component_id);

                if let Some(neighbours) = adj.get(i) {
                    for neighbour in neighbours {
                        if components.contains_key(neighbour) {
                            continue;
                        }
                        components.insert(**neighbour, component_id);
                        queue.push_back(neighbour);
                    }
                }
            }
            component_id += 1;
        }
    }
    component_id == 1
}
