use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let path = "input/09.txt";
    part1(path);
    part2(path);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut max_area = 0;
    for line in input.split("\n") {
        let (x0, y0) = line.split_once(",").unwrap();
        let x0: i64 = x0.parse().unwrap();
        let y0: i64 = y0.parse().unwrap();
        for line in input.split("\n") {
            let (x1, y1) = line.split_once(",").unwrap();
            let x1: i64 = x1.parse().unwrap();
            let y1: i64 = y1.parse().unwrap();
            let area = ((x0 - x1).abs() + 1) * ((y0 - y1).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    println!("part1:\n{max_area}");
}

fn part2(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();

    // Parse input into coords
    let coords: Vec<(i64, i64)> = input
        .split("\n")
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            let x: i64 = x.parse().unwrap();
            let y: i64 = y.parse().unwrap();
            (x, y)
        })
        .collect();

    let min_x = coords.iter().map(|c| c.0).min().unwrap();
    let min_y = coords.iter().map(|c| c.1).min().unwrap();
    let max_x = coords.iter().map(|c| c.0).max().unwrap();
    let max_y = coords.iter().map(|c| c.1).max().unwrap();

    // Construct a boundary
    let mut boundary = HashSet::new();
    for i in 0..coords.len() - 1 {
        let c0 = coords[i];
        let c1 = coords[i + 1];
        add_edge(&mut boundary, c0, c1);
    }
    let c0 = coords[coords.len() - 1];
    let c1 = coords[0];
    add_edge(&mut boundary, c0, c1);

    // Expand the boundary by 1 cell
    let mut outside = HashSet::with_capacity(1_000_000_000);
    let mut queue = VecDeque::new();
    let start_pos = coords.iter().find(|c| c.1 == max_y).unwrap();
    queue.push_back((start_pos.0, start_pos.1 + 1));
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if boundary.contains(&(x, y)) {
            continue;
        }
        outside.insert((x, y));
        for dx in -1..2 {
            for dy in -1..2 {
                if (dx == 0) & (dy == 0) {
                    continue;
                }
                let nx = x + dy;
                let ny = y + dx;
                if (nx < min_x - 2) | (nx > max_x + 2) {
                    continue;
                }
                if (ny < min_y - 2) | (ny > max_y + 2) {
                    continue;
                }
                if outside.contains(&(nx, ny)) {
                    continue;
                }
                if queue.contains(&(nx, ny)) {
                    continue;
                }
                let mut adj_boundary = false;
                for dx in -1..2 {
                    for dy in -1..2 {
                        if (dx == 0) & (dy == 0) {
                            continue;
                        }
                        if boundary.contains(&(nx + dx, ny + dy)) {
                            adj_boundary = true;
                        }
                    }
                }
                if !adj_boundary {
                    continue;
                }
                queue.push_back((nx, ny));
            }
        }
    }

    let mut max_area = 0;
    let mut cache = HashMap::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let c0 = coords[i];
            let c1 = coords[j];

            let x0 = c0.0.min(c1.0);
            let x1 = c0.0.max(c1.0);
            let y0 = c0.1.min(c1.1);
            let y1 = c0.1.max(c1.1);
            let area = ((x0 - x1).abs() + 1) * ((y0 - y1).abs() + 1);
            if area < max_area {
                continue;
            }

            if !valid_edge(&mut cache, &outside, (x0, y0), (x0, y1)) {
                continue;
            }
            if !valid_edge(&mut cache, &outside, (x0, y1), (x1, y1)) {
                continue;
            }
            if !valid_edge(&mut cache, &outside, (x1, y1), (x1, y0)) {
                continue;
            }
            if !valid_edge(&mut cache, &outside, (x1, y0), (x0, y0)) {
                continue;
            }

            if area > max_area {
                max_area = area;
            }
        }
    }
    println!("part2:\n{max_area}");
}

fn valid_edge(
    cache: &mut HashMap<((i64, i64), (i64, i64)), bool>,
    outside: &HashSet<(i64, i64)>,
    p0: (i64, i64),
    p1: (i64, i64),
) -> bool {
    if let Some(x) = cache.get(&(p0, p1)) {
        return *x;
    }

    let min_x = p0.0.min(p1.0);
    let max_x = p0.0.max(p1.0);
    let min_y = p0.1.min(p1.1);
    let max_y = p0.1.max(p1.1);
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            if outside.contains(&(x, y)) {
                cache.insert((p0, p1), false);
                cache.insert((p1, p0), false);
                return false;
            }
        }
    }
    cache.insert((p0, p1), true);
    cache.insert((p1, p0), true);
    true
}

fn add_edge(boundary: &mut HashSet<(i64, i64)>, p0: (i64, i64), p1: (i64, i64)) {
    let min_x = p0.0.min(p1.0);
    let max_x = p0.0.max(p1.0);
    let min_y = p0.1.min(p1.1);
    let max_y = p0.1.max(p1.1);
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            boundary.insert((x, y));
        }
    }
}
