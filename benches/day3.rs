use aoc_2025::base::Day;
use aoc_2025::days::day3::Day3;
use criterion::{criterion_group, criterion_main, Criterion};

fn day1_part1(c: &mut Criterion) {
    let day = Day3;
    let data = day.prepare();
    match data {
        Ok(data) => {
            c.bench_function("day3_part1", |b| {
                b.iter(|| {
                    day.solve_part_one(&&data);
                })
            });
        }
        Err(_) => eprintln!("Failed to evaluate day {:?}", Day3.day_number()),
    }
}

fn day1_part2(c: &mut Criterion) {
    let day = Day3;
    let data = day.prepare();
    match data {
        Ok(data) => {
            c.bench_function("day3_part2", |b| {
                b.iter(|| {
                    day.solve_part_two(&&data);
                })
            });
        }
        Err(_) => eprintln!("Failed to evaluate day {:?}", Day3.day_number()),
    }
}

criterion_group!(part1, day1_part1);
criterion_group!(part2, day1_part2);
criterion_main!(part1, part2);
