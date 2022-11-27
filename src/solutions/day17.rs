use itertools::Itertools;
use std::fs;

type InputType = Vec<String>;

fn read_input() -> InputType {
    fs::read_to_string("./inputs/day17.txt")
        .unwrap()
        .trim()
        .lines()
        .map_into()
        .collect()
}

fn part1(_input: InputType) -> i64 {
    0
}

fn part2(_input: InputType) -> i64 {
    0
}

pub fn run() {
    let input = read_input();

    println!("--- Day 17 ---");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}
