use std::{
    collections::{HashMap, HashSet},
    fs,
};

type BoardType = HashMap<u32, (u32, u32)>;
type InputType = (Vec<u32>, Vec<BoardType>);
const BOARD_SIZE: u32 = 5;

fn read_input() -> InputType {
    let file = fs::read_to_string("./inputs/day04.txt").unwrap();

    let (drawn, boards) = file.split_once("\n\n").unwrap();

    let drawn: Vec<_> = drawn.split(',').map(|s| s.parse().unwrap()).collect();

    let boards: Vec<BoardType> = boards
        .split("\n\n")
        .map(|b| {
            b.split_ascii_whitespace()
                .enumerate()
                .map(|(i, n)| (n.parse().unwrap(), ((i % 5) as u32, (i / 5) as u32)))
                .collect()
        })
        .collect();

    (drawn, boards)
}

fn gen_row(row_idx: u32) -> HashSet<(u32, u32)> {
    HashSet::from_iter((0..BOARD_SIZE).map(|i| (row_idx, i as u32)))
}

fn gen_col(col_idx: u32) -> HashSet<(u32, u32)> {
    HashSet::from_iter((0..BOARD_SIZE).map(|i| (i as u32, col_idx)))
}

fn check_bingo(indexes: &HashSet<(u32, u32)>) -> bool {
    for i in 0..BOARD_SIZE {
        if indexes.is_superset(&gen_row(i)) || indexes.is_superset(&gen_col(i)) {
            return true;
        }
    }

    false
}

fn calc_numbers_to_win(drawn: &[u32], board: &BoardType) -> (u32, u32, u32) {
    let mut matched_index = HashSet::<(u32, u32)>::new();
    let mut board_mut = board.clone();

    for (i, n) in drawn.iter().enumerate() {
        if let Some(index) = board_mut.remove(n) {
            matched_index.insert(index);

            if check_bingo(&matched_index) {
                return (i as u32, board_mut.keys().sum(), *n);
            }
        }
    }

    unreachable!()
}

fn part1((drawn, boards): &InputType) -> u32 {
    let mut min_steps = u32::MAX;
    let mut ans = 0;
    for board in boards {
        let (steps, v, last_num) = calc_numbers_to_win(drawn, board);
        if steps < min_steps {
            min_steps = steps;

            ans = v * last_num;
        }
    }

    ans
}

fn part2((drawn, boards): &InputType) -> u32 {
    let mut max_steps = 0;
    let mut ans = 0;
    for board in boards {
        let (steps, v, last_num) = calc_numbers_to_win(drawn, board);
        if steps > max_steps {
            max_steps = steps;

            ans = v * last_num;
        }
    }

    ans
}

pub fn run() {
    let input = read_input();

    println!("--- Day 04 ---");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
