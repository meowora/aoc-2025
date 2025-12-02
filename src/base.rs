use std::fmt::Debug;

pub trait Day<T : 'static, E : Debug + 'static> {
    fn prepare(&self) -> Result<T, E>;
    fn day_number(&self) -> i8;

    fn solve_part_one(&self, input: &&T) -> impl ToString;
    fn solve_part_two(&self, input: &&T) -> impl ToString;
}