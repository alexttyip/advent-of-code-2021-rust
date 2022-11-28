use regex::Regex;
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
    fs,
};

type InputType = Vec<((i16, i16), (i16, i16))>;

fn read_input() -> InputType {
    let re: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let file = fs::read_to_string("./inputs/day05.txt").unwrap();

    file.lines()
        .map(|line| re.captures(line).unwrap())
        .map(|cap| {
            (
                (
                    cap.get(1).unwrap().as_str().parse().unwrap(),
                    cap.get(2).unwrap().as_str().parse().unwrap(),
                ),
                (
                    cap.get(3).unwrap().as_str().parse().unwrap(),
                    cap.get(4).unwrap().as_str().parse().unwrap(),
                ),
            )
        })
        .collect()
}

fn part1(input: InputType) -> usize {
    let mut grid = HashMap::<(i16, i16), i16>::new();
    for ((x1, y1), (x2, y2)) in input {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                *grid.entry((x1, y)).or_insert(0) += 1;
            }
        }

        if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                *grid.entry((x, y1)).or_insert(0) += 1;
            }
        }
    }

    grid.values().filter(|&n| n >= &2).count()
}

fn part2(input: InputType) -> usize {
    let mut grid = HashMap::<(i16, i16), i16>::new();
    for ((x1, y1), (x2, y2)) in input {
        let mut x = x1;
        let mut y = y1;
        let dx = match x1.cmp(&x2) {
            Less => 1,
            Equal => 0,
            Greater => -1,
        };
        let dy = match y1.cmp(&y2) {
            Less => 1,
            Equal => 0,
            Greater => -1,
        };

        loop {
            *grid.entry((x, y)).or_insert(0) += 1;

            if x == x2 && y == y2 {
                break;
            }

            x += dx;
            y += dy;
        }
    }

    grid.values().filter(|&n| n >= &2).count()
}

pub fn run() {
    let input = read_input();

    println!("--- Day 05 ---");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}
