
fn main() {
    let path = "input/03.txt";
    part1(path);
    part2(path);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut output = 0;
    for line in input.split("\n") {
        let values: Vec<u32> = line.chars().into_iter().map(|c| c.to_digit(10).unwrap()).collect();
        let mut max_value = 0;
        for i in 0..values.len() {
            for j in i + 1..values.len() {
                let value = values[i] * 10 + values[j];
                if value > max_value {
                    max_value = value;
                }
            }
        }
        output += max_value;
    }
    println!("part1:\n{output}");
}

fn part2(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut output = 0;
    for line in input.split("\n") {
        let values: Vec<i64> = line.chars().into_iter().map(|c| c.to_digit(10).unwrap() as i64).collect();
        let mut best_value = 0;
        let mut consumed = 0;
        for mut d in 0..12 {
            d = 12 - d;
            let remaining = &values[consumed..values.len() - d + 1];            
            let max = remaining.iter().max().unwrap();
            consumed += 1 + remaining.iter().position(|e| e == max).unwrap().clone() as usize;
            let m = i64::pow(10, (d - 1) as u32);
            best_value += max * m;
        }
        output += best_value;
    }
    println!("part2:\n{output}");
}
