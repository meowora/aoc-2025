use crate::base::Day;
use std::{fs, io};

pub struct Day3;

impl Day<Vec<String>, io::Error> for Day3 {
    fn prepare(&self) -> Result<Vec<String>, io::Error> {
        fs::read_to_string("inputs/day3.txt").map(|s| s.lines().into_iter().map(|x| x.to_string()).collect())
    }

    fn day_number(&self) -> i8 {
        3
    }

    fn solve_part_one(&self, input: &&Vec<String>) -> impl ToString {
        solve_part_one(input)
    }

    fn solve_part_two(&self, input: &&Vec<String>) -> impl ToString {
        solve_part_two(input)
    }
}

fn solve_part_one(input: &Vec<String>) -> u64 {
    let mut sum = 0;
    for line in input {
        let chars = line.as_bytes();

        let (start_after, first) = find_biggest(&&chars, 0, 1);
        let (_, second) = find_biggest(&&chars, start_after + 1, 0);

        sum += to_int(first) * 10 + to_int(second)
    }
    sum
}

fn to_int(c: char) -> u64 {
    match c {
        '1' => 1u64,
        '2' => 2u64,
        '3' => 3u64,
        '4' => 4u64,
        '5' => 5u64,
        '6' => 6u64,
        '7' => 7u64,
        '8' => 8u64,
        '9' => 9u64,
        _ => 0,
    }
}

fn get_offset(i: usize) -> u64 {
    match i {
        1 => 10,
        2 => 100,
        3 => 1_000,
        4 => 10_000,
        5 => 100_000,
        6 => 1_000_000,
        7 => 10_000_000,
        8 => 100_000_000,
        9 => 1_000_000_000,
        10 => 10_000_000_000,
        11 => 100_000_000_000,
        _ => 1,
    }
}

fn find_biggest(chars: &&[u8], start_after: usize, skip: usize) -> (usize, char) {
    let mut max_index = (start_after, chars[start_after] as char);

    for index in start_after..(chars.len() - skip) {
        let char = chars[index] as char;
        if char > max_index.1 {
            max_index = (index, char)
        }
    }

    max_index
}

fn solve_part_two(input: &Vec<String>) -> u64 {
    let mut sum = 0u64;

    for line in input {
        let chars = line.as_bytes();

        let mut meow: u64 = 0;
        let mut search_after = 0;
        for i in 0..12 {
            let (new_start, biggest) = find_biggest(&&chars, search_after, 11 - i);

            let literal = to_int(biggest);
            meow += literal * get_offset(11 - i);

            search_after = new_start + 1;
        }

        sum += meow
    }
    sum
}
