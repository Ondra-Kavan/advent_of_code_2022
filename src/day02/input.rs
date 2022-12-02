use crate::day02::Input;

const INPUT: &str = include_str!("../../input/02/input.txt");

pub fn read() -> Input {
    let input_lines: Vec<Vec<char>> = INPUT.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.replace(" ", ""))
        .map(|s| s.chars().collect())
        .collect();

    let mut tuple_vec: Vec<(char, char)> = Vec::with_capacity(256);
    for i in 0..input_lines.len() {
        tuple_vec.push((input_lines[i][0],input_lines[i][1]));
    }
    tuple_vec
}
