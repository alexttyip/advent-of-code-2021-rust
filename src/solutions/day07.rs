use std::fs;

type InputType = Vec<u32>;

fn read_input() -> InputType {
    fs::read_to_string("./inputs/day07.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect()
}

fn part1(input: InputType) -> u32 {
    let mut sorted = input;
    sorted.sort_unstable();

    let median = sorted.get(sorted.len() / 2).unwrap();

    sorted.iter().map(|n| n.abs_diff(*median)).sum()
}

fn part2(input: InputType) -> u32 {
    let min: u32 = *input.iter().min().unwrap();
    let max: u32 = *input.iter().max().unwrap();

    let mut min_diff = u32::MAX;

    for i in min..=max {
        let diff = input
            .iter()
            .map(|n| i.abs_diff(*n))
            .map(|dist| dist * (dist + 1) / 2)
            .sum();

        if diff < min_diff {
            min_diff = diff;
        }
    }

    min_diff
}

pub fn run() {
    let input = read_input();

    println!("--- Day 07 ---");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}
