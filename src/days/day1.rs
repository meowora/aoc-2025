use std::{fs, io};
use crate::base::Day;

pub struct Day1;

impl Day<String, io::Error> for Day1 {
    fn prepare(&self) -> Result<String, io::Error> {
        fs::read_to_string("inputs/day1.txt")
    }

    fn day_number(&self) -> i8 {
        1
    }

    fn solve_part_one(&self, input: &&String) -> impl ToString {
        solve_part1(input)
    }

    fn solve_part_two(&self, input: &&String) -> impl ToString  {
        solve_part2(input)
    }
}

pub fn solve_part2(input: &String) -> i32 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut zeros = 0;
    let mut dial_index = 50;
    lines
        .iter()
        .map(|&line| line.to_string())
        .for_each(|mut line| {
            let char = line.remove(0);
            let amount = match line.parse::<i32>() {
                Ok(number) => number,
                Err(_) => panic!("Failed to parse int from input {line}"),
            };

            let number: i32;
            match char {
                'L' => number = -1,
                'R' => number = 1,
                default => panic!("Unmatched char {default}"),
            };

            for _ in 0..amount {
                dial_index += number;
                if dial_index < 0 {
                    dial_index += 100
                } else if dial_index > 99 {
                    dial_index -= 100
                }
                if dial_index == 0 {
                    zeros += 1;
                }
            }
        });

    zeros
}

pub fn solve_part1(input: &String) -> i32 {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut zeros = 0;
    let mut dial_index = 50;
    lines
        .iter()
        .map(|&line| line.to_string())
        .for_each(|mut line| {
            let char = line.remove(0);
            let amount = match line.parse::<i32>() {
                Ok(number) => number,
                Err(_) => panic!("Failed to parse int from input {line}"),
            };

            //let current_index = dial_index;
            match char {
                'L' => dial_index -= amount,
                'R' => dial_index += amount,
                default => panic!("Unmatched char {default}"),
            };

            while dial_index < 0 {
                dial_index += 100;
            }
            while dial_index > 99 {
                dial_index -= 100;
            }
            if dial_index == 0 {
                zeros += 1;
            }
            //println!("Turned {char} from {current_index} to {dial_index}; total zeros {zeros}");
        });

    zeros
}
