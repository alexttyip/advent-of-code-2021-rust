use std::fs;

type InputType = Vec<usize>;

fn read_input() -> InputType {
    fs::read_to_string("./inputs/day06.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect()
}

fn solve(input: &InputType, days: u16) -> u64 {
    let mut fishes = [0; 9];

    for i in input {
        fishes[*i] += 1;
    }

    for _ in 0..days {
        for i in 0..8 {
            fishes.swap(i, i + 1);
        }

        fishes[6] += fishes[8];
    }

    fishes.iter().sum()
}

fn part1(input: &InputType) -> u64 {
    solve(input, 80)
}

fn part2(input: &InputType) -> u64 {
    solve(input, 256)
}

pub fn run() {
    let input = read_input();

    println!("--- Day 06 ---");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
