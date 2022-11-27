use std::fs;

type InputType = Vec<u32>;
const NUM_BITS: usize = 12;

fn read_input() -> InputType {
    fs::read_to_string("./inputs/day03.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect()
}

fn part1(input: &InputType) -> u32 {
    let mut gamma = 0b0;

    for i in 0..NUM_BITS {
        let pos_count = input
            .iter()
            .filter(|&v| *v & (1 << (NUM_BITS - i - 1)) > 0)
            .count();

        gamma <<= 1;
        gamma += u32::from(pos_count > input.len() / 2);
    }

    let filter = (0b1 << NUM_BITS) - 1;

    gamma * (gamma ^ filter)
}

fn reduce_values(values: &Vec<u32>, i: usize, is_co2: bool) -> Vec<u32> {
    if values.len() == 1 {
        return values.to_vec();
    }

    let (ones, zeros): (Vec<u32>, Vec<u32>) = values
        .iter()
        .partition(|&v| *v & (1 << (NUM_BITS - i - 1)) > 0);

    reduce_values(
        if (ones.len() >= zeros.len()) ^ is_co2 {
            &ones
        } else {
            &zeros
        },
        i + 1,
        is_co2,
    )
}

fn part2(inputs: &InputType) -> u32 {
    let oxygen = reduce_values(inputs, 0, false);
    let co2 = reduce_values(inputs, 0, true);

    oxygen.first().unwrap() * co2.first().unwrap()
}

pub fn run() {
    let input = read_input();

    println!("--- Day 03 ---");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
