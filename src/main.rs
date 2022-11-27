use dotenv::dotenv;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use reqwest::{blocking, header::COOKIE};

use crate::solutions::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
};

mod solutions;

const DAY_FNS: [fn(); 25] = [
    day01::run,
    day02::run,
    day03::run,
    day04::run,
    day05::run,
    day06::run,
    day07::run,
    day08::run,
    day09::run,
    day10::run,
    day11::run,
    day12::run,
    day13::run,
    day14::run,
    day15::run,
    day16::run,
    day17::run,
    day18::run,
    day19::run,
    day20::run,
    day21::run,
    day22::run,
    day23::run,
    day24::run,
    day25::run,
];

fn scrape_input(day: usize) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("./inputs/day{day:02}.txt"))
        .expect("Unable to open inputs dir or input file already exists");

    let url = format!("https://adventofcode.com/2019/day/{day}/input");

    let session = env::var("SESSION").expect("AoC session ID must be set");

    let client = blocking::Client::new();
    let input = client
        .get(url)
        .header(COOKIE, format!("session={}", session))
        .send()
        .and_then(|res| res.text())
        .expect("Can't download input from AoC");

    write!(file, "{}", input).expect("File writting failed");
}

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if let Some(day_str) = args.get(1) {
        if let Ok(day) = day_str.parse::<usize>() {
            if let Some(day_fn) = DAY_FNS.get(day - 1) {
                scrape_input(day);
                day_fn();
            }
        }
    }
}
