use crate::day01::{Input, Output};

pub fn solve(input: &mut Input) -> Output {
    input.sort_unstable();
    let mut out: u32 = 0;
    for i in 0..3 {
        out += input[input.len()-(i+1)];
    }
    Output::U32(out)
}
