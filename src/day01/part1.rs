use crate::day01::{Input, Output};

pub fn solve(input: &mut Input) -> Output {
    input.sort_unstable();
    let out = input[input.len() -1];
    Output::U32(out)
}
