use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut inter: Vec<u32> = Vec::with_capacity(256);
    for i in 0..input.len() {
        // println!("in: {:?}", input[i]);
        let mut x = 0;
        for j in 0..input[i].len() {
            x += input[i][j];
        }
        inter.push(x);
    }
    // for i in 0..inter.len() {
    //     println!("o: {:?}", inter[i]);
    // }
    inter.sort_unstable();
    let mut out: u32 = 0;
    for i in 0..3 {
        out += inter[inter.len()-(i+1)];
    }
    Output::U32(out)
}
