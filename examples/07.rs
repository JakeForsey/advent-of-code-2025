use std::collections::{HashMap, HashSet};

fn main() {
    let path = "input/07.txt";
    part1(path);
    part2(path);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let line0 = lines.first().unwrap();
    let initial_beam = line0.chars().position(|c| c == 'S').unwrap();

    let mut beams = HashSet::new();
    beams.insert(initial_beam);

    let mut splits = 0;
    for line in lines.iter().skip(1) {
        for (i, char) in line.chars().enumerate() {
            if char == '^' {
                if beams.contains(&i) {
                    beams.remove(&i);
                    beams.insert(i - 1);
                    beams.insert(i + 1);
                    splits += 1;
                }
            }
        }
    }
    println!("part1:\n{splits}");
}

fn part2(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let line0 = lines.first().unwrap();
    let initial_beam = line0.chars().position(|c| c == 'S').unwrap();

    let mut beams: HashMap<usize, i64> = HashMap::new();
    beams.insert(initial_beam, 1);

    for line in lines.iter().skip(1) {
        for (i, char) in line.chars().enumerate() {
            if char == '^' {
                if let Some(count) = beams.remove(&i) {
                    beams
                        .entry(i - 1)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                    beams
                        .entry(i + 1)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
            }
        }
    }
    let timelines = beams.values().into_iter().sum::<i64>();
    println!("part2:\n{timelines}");
}
