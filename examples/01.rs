fn main() {
    let path = "input/01.txt";
    part1(path);
    part2(path);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut position = 50;
    let mut password: i32 = 0;
    for line in input.split("\n") {
        let (direction, size_str) = line.split_at(1);
        let size: i32 = size_str.parse().unwrap();
        if direction == "L" {
            position -= size;
        } else {
            position += size;
        }
        position %= 100;
        if position == 0 {
            password += 1;
        }
    }
    println!("part1:\n{password}");
}

fn part2(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut position = 50;
    let mut password: i32 = 0;
    for line in input.split("\n") {
        let (direction, size_str) = line.split_at(1);
        let size: i32 = size_str.parse().unwrap();
        for _ in 0..size {
            if direction == "L" {
                position -= 1;
            } else {
                position += 1;
            }
            if position == -1 {
                position = 99;
            } else if position == 100 {
                position = 0;
            }
            if position == 0 {
                password += 1;
            }
        }
    }
    println!("part2:\n{password}");
}
