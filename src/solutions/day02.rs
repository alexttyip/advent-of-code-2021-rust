use std::fs;

type InputType = Vec<(String, i32)>;

fn read_input() -> InputType {
    fs::read_to_string("./inputs/day02.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.split(' '))
        // .map(|a| a.collect::<Vec<&str>>())
        .map(|mut a| {
            (
                a.next().unwrap().to_string(),
                a.next().map(|b| b.parse().unwrap()).unwrap(),
            )
        })
        .collect()
}

fn part1(input: InputType) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for (op, mag) in input {
        match op.as_str() {
            "forward" => horizontal += mag,
            "down" => depth += mag,
            "up" => depth -= mag,
            _ => (),
        }
    }

    horizontal * depth
}

fn part2(input: InputType) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (op, mag) in input {
        match op.as_str() {
            "forward" => {
                horizontal += mag;
                depth += aim * mag;
            }
            "down" => aim += mag,
            "up" => aim -= mag,
            _ => (),
        }
    }

    horizontal * depth
}

pub fn run() {
    let input = read_input();

    println!("--- Day 02 ---");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}
