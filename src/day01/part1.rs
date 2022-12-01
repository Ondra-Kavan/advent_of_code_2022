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
    // println!("inter length: {}", inter.len());
    // for i in 0..inter.len() {
    //     println!("o: {:?}", inter[i]);
    // }
    let out = inter.iter().max().unwrap();
    Output::U32(*out)
}
