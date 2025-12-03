fn main() {
    let path = "input/02.txt";
    part1(path);
    part2(path);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut output = 0;
    for pair_str in input.split(",") {
        let (start, end) = pair_str.split_once("-").unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();
        for id in start..end + 1 {
            let id_str = id.to_string();
            if id_str.len() % 2 == 0 {
                let (p1, p2) = id_str.split_at(id_str.len() / 2);
                if p1 == p2 {
                    output += id;
                }
            }
        }
    }
    println!("part1:\n{output}");
}

fn part2(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut output = 0;
    for pair_str in input.split(",") {
        let (start, end) = pair_str.split_once("-").unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();
        for id in start..end + 1 {
            let id_str = id.to_string();
            for width in 1..(id_str.len() / 2) + 1 {
                if id_str.len() % width != 0 {
                    continue;
                }
                let pattern = &id_str[0..width];
                let mut start_idx = width;
                let mut matches = true;
                while matches {
                    let chunk = &id_str[start_idx..start_idx + width];
                    matches &= pattern == chunk;
                    start_idx += width;
                    if start_idx + width > id_str.len() {
                        break
                    }
                }
                if matches {
                    output += id;
                    break
                }
            }
        }
    }
    println!("part2:\n{output}");
}
