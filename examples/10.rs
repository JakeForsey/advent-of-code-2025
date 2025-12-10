use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

fn main() {
    let path = "input/10.txt";
    part1(path);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut output = 0;
    for line in input.split("\n") {
        let (machines_str, rest) = line.split_once(" ").unwrap();
        let mut target_machines: Vec<bool> = machines_str
            .chars()
            .into_iter()
            .skip(1)
            .map(|c| c == '#')
            .collect();
        target_machines.pop();

        let buttons: Vec<Vec<u32>> = rest
            .split(" ")
            .filter(|s| s.contains("("))
            .map(|chunk| {
                let b: Vec<u32> = chunk
                    .chars()
                    .into_iter()
                    .filter(|c| c.is_digit(10))
                    .map(|c| c.to_digit(10).unwrap())
                    .collect();
                b
            })
            .collect();

        let mut queue = VecDeque::new();
        queue.push_back((vec![false; target_machines.len()], 0));
        let mut min_steps: i64 = 999999999999;
        let mut seen: HashSet<Vec<bool>> = HashSet::new();
        while let Some((machines, steps)) = queue.pop_front() {
            seen.insert(machines.clone());
            if steps > min_steps {
                continue;
            }
            if machines == target_machines {
                min_steps = steps;
                continue;
            }
            for button in &buttons {
                let mut next_machines: Vec<bool> = machines.iter().copied().collect();
                for i in button {
                    next_machines[*i as usize] = !next_machines[*i as usize];
                }
                if seen.contains(&next_machines) {
                    continue;
                }
                queue.push_back((next_machines, steps + 1));
            }
        }
        output += min_steps;
    }
    println!("part1:\n{output}");
}
