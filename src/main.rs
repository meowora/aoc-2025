use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use std::fmt::Debug;

mod base;
mod days;

fn main() {
    evaluate_day(Day1);
    evaluate_day(Day2);
    evaluate_day(Day3);
}

fn evaluate_day<T: 'static, E: Debug + 'static>(day: impl base::Day<T, E> + 'static) {
    let data = day.prepare();
    match data {
        Ok(data) => {
            run_and_log(
                "Day ".to_string() + &day.day_number().to_string() + " / Part 1",
                day.solve_part_one(&&data).to_string(),
            );
            run_and_log(
                "Day ".to_string() + &day.day_number().to_string() + " / Part 2",
                day.solve_part_two(&&data).to_string(),
            );
        }
        Err(_) => eprintln!("Failed to evaluate day {:?}", day.day_number()),
    }
}

fn run_and_log(name: String, data: String) {
    let output = data.to_string();
    println!("{name} -> {output}")
}
