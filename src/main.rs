#![allow(dead_code)]

mod day1;
mod day2;

use crate::day2::day2::do_it;
use std::fs::read_to_string;

fn main() {
    do_it();
}

fn input_to_vec(day: u8) -> Vec<String> {
    read_to_string(format!("inputs/day{day}.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
