use crate::day01::Input;

const INPUT: &str = include_str!("../../input/01/input.txt");

/// Read and parse the input
pub fn read() -> Input {
    let i: Vec<&str> = INPUT.split("\n\n").collect();
    let mut o: Vec<Vec<u32>> = vec![vec![]];

    for x in 0..i.len() {
        let out: Vec<u32> = i[x].split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        o.push(out);
    }

    let mut inter: Vec<u32> = Vec::with_capacity(256);
    for i in 0..o.len() {
        let mut x = 0;
        for j in 0..o[i].len() {
            x += o[i][j];
        }
        inter.push(x);
    }
    inter
}
