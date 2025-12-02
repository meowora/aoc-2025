use crate::base::Day;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;
use std::{fs, io};

pub struct Day2;

impl Day<String, io::Error> for Day2 {
    fn prepare(&self) -> Result<String, io::Error> {
        fs::read_to_string("inputs/day2.txt")
    }

    fn day_number(&self) -> i8 {
        2
    }

    fn solve_part_one(&self, input: &&String) -> impl ToString {
        solve_part_one(input)
    }

    fn solve_part_two(&self, input: &&String) -> impl ToString {
        solve_part_two(input)
    }
}

fn solve_part_one(input: &&String) -> i64 {
    input
        .split(",")
        .flat_map(range_to_vec)
        .collect::<Vec<i64>>()
        .into_par_iter()
        .filter(test_first_id)
        .sum()
}

fn solve_part_two(input: &&String) -> i64 {
    input
        .split(",")
        .flat_map(range_to_vec)
        .collect::<Vec<i64>>()
        .into_par_iter()
        .filter(test_second_id)
        .sum()
}

fn test_first_id(id: &i64) -> bool {
    let string = id.to_string();
    let half = string.len() / 2;
    string[..half] == string[half..]
}

fn test_second_id(id: &i64) -> bool {
    let string = id.to_string();
    let half = string.len() / 2;
    for x in 1..half+1 {
        let set = (0..string.len() / x)
            .map(|index| string[index * x..(index + 1) * x].to_string())
            .collect::<HashSet<String>>();
        if set.len() == 1 && string.len() % x == 0 {
            return true;
        }
    }
    false
}

fn range_to_vec(range: &str) -> impl Iterator<Item = i64> {
    let range = range
        .split("-")
        .map(|number| number.parse::<i64>())
        .map(|it| it.unwrap())
        .collect::<Vec<i64>>();

    range[0]..range[1] + 1
}
