use std::collections::{HashMap, HashSet};


fn main() {
    let path = "input/04.txt";
    part1(path);
    part2(path);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut map = HashSet::new();
    for (i, line) in input.split("\n").enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '@' {
                map.insert((i as i32, j as i32));
            }
        }
    }
    let mut accessible_rolls = 0;
    for (i, j) in map.iter() {
        let mut adj_count = 0;
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if (dx == 0) & (dy == 0) {
                    continue;
                }
                let ii = i + dx;
                let jj = j + dy;
                if map.contains(&(ii, jj)) {
                    adj_count += 1;
                }
            }
        }
        if adj_count < 4 {
            accessible_rolls += 1;
        }
    }
    println!("part1:\n{accessible_rolls}");
}

fn part2(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut map = HashSet::new();
    for (i, line) in input.split("\n").enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '@' {
                map.insert((i as i32, j as i32));
            }
        }
    }

    let mut total_accessible = 0;
    loop {
        let mut accessible_rolls = HashSet::new();
        for (i, j) in map.iter() {
            let mut adj_count = 0;
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if (dx == 0) & (dy == 0) {
                        continue;
                    }
                    let ii = i + dx;
                    let jj = j + dy;
                    if map.contains(&(ii, jj)) {
                        adj_count += 1;
                    }
                }
            }
            if adj_count < 4 {
                accessible_rolls.insert((i.clone(), j.clone()));
            }
        }
        
        if accessible_rolls.is_empty() {
            break;
        } else {
            for roll in accessible_rolls {
                map.remove(&roll);
                total_accessible += 1;
            }
        }
    }
    println!("part2:\n{total_accessible}");
}

