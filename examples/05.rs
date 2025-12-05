use std::collections::VecDeque;

fn main() {
    let path = "input/05.txt";
    part1(path);
    part2(path);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();
    let ranges: Vec<(u64, u64)> = ranges_str
        .split("\n")
        .map(|r| {
            let (start_str, end_str) = r.split_once("-").unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            (start, end)
        })
        .collect();

    let mut output = 0;
    for id_str in ids_str.split("\n") {
        let id: u64 = id_str.parse().unwrap();
        for (start, end) in &ranges {
            if (*start <= id) & (id <= *end) {
                output += 1;
                break;
            }
        }
    }
    println!("part1:\n{output}");
}

fn part2(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let (ranges_str, _) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<(u64, u64)> = ranges_str
        .split("\n")
        .map(|r| {
            let (start_str, end_str) = r.split_once("-").unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            (start, end)
        })
        .collect();
    ranges.sort();

    let mut queue = VecDeque::from(ranges);
    let mut merged = Vec::new();
    while !queue.is_empty() {
        let a = queue.pop_front();
        let b = queue.pop_front();
        match (a, b) {
            (Some(a), Some(b)) => {
                if b.0 <= a.1 {
                    let a1 = if a.1 > b.1 { a.1 } else { b.1 };
                    let new = (a.0, a1);
                    queue.push_front(new);
                } else {
                    merged.push(a);
                    queue.push_front(b);
                }
            }
            (None, Some(b)) => {
                unreachable!()
            }
            (Some(a), None) => {
                merged.push(a);
            }
            (None, None) => {
                unreachable!()
            }
        }
    }
    let output = merged.iter().map(|r| (r.1 - r.0) + 1).sum::<u64>();
    println!("part2:\n{output}");
}
