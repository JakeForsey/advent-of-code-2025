use std::collections::HashMap;

fn main() {
    let path = "input/06.txt";
    part1(path);
    part2(path);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut ops: HashMap<i64, &str> = HashMap::new();
    let mut data: HashMap<i64, i64> = HashMap::new();
    for line in lines.into_iter().rev() {
        let mut i = 0;
        for part in line.split(" ") {
            if part == "" {
                continue;
            }
            if part == "*" {
                ops.insert(i, part);
                data.insert(i, 1);
            } else if part == "+" {
                ops.insert(i, part);
                data.insert(i, 0);
            } else {
                let value: i64 = part.parse().unwrap();
                let accum = data.get_mut(&i).unwrap();
                if *ops.get(&i).unwrap() == "*" {
                    *accum *= value;
                } else if *ops.get(&i).unwrap() == "+" {
                    *accum += value;
                }
            }
            i += 1;
        }
    }
    let output: i64 = data.values().sum();
    println!("part1:\n{output}");
}

fn part2(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let width = lines.first().unwrap().len();
    let height = lines.len();

    let mut transposed = vec![vec!['_'; height]; width];
    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            transposed[j][i] = char;
        }
    }

    let mut op = '_';
    let mut value = 0;
    let mut output = 0;
    for row in transposed {
        if row.last().unwrap() != &' ' {
            // Starting a new calc
            op = row.last().unwrap().clone();
            value = if op == '*' { 1 } else { 0 };
        }
        if row.iter().all(|c| *c == ' ') {
            // Finishing a cal
            output += value;
            continue;
        }
        let number_str: String = row.iter().filter(|c| c.is_numeric()).collect();
        let number: i64 = number_str.parse().unwrap();
        if op == '+' {
            value += number;
        } else {
            value *= number;
        }
    }
    // Remember the last value!
    output += value;
    println!("part2:\n{output}");
}
