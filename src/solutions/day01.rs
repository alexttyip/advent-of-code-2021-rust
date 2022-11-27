use itertools::Itertools;
use std::fs;

type InputType = Vec<u32>;

fn read_input() -> InputType {
    fs::read_to_string("./inputs/day01.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(input: InputType) -> usize {
    input.iter().tuple_windows().filter(|(a, b)| a < b).count()
}

fn part2(input: InputType) -> usize {
    input
        .iter()
        .tuple_windows()
        .filter(|(a, _, _, d)| a < d)
        .count()
}

pub fn run() {
    let input = read_input();

    println!("--- Day 01 ---");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}
