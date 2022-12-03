// I was really stuck on this one
// CREDIT: https://github.com/LinAGKar/advent-of-code-2022-rust/blob/main/day3a/src/main.rs

use std::str::Chars;

use crate::day03::{Input, Output};

const INPUT: &str = include_str!("../../input/03/input.txt");
pub fn solve(input: &Input) -> Output {
    Output::U16(
        input
            .lines()
            .map(|line| {
                let mut sack = [0; 48];
                let mut sack_len = 0;

                for (m, c) in line.chars().enumerate() {
                    let priority = if c.is_ascii_lowercase() {
                        c as u32 - 'a' as u32 + 1
                    } else if c.is_ascii_uppercase() {
                        c as u32 - 'A' as u32 + 27
                    } else {
                        panic!("{} was not ASCII letter", c)
                    } as u8;
                    sack[m] = priority;
                    sack_len = m + 1;
                }

                let mut in_first = [false; 53];
                for &priority in sack.iter().take(sack_len / 2) {
                    in_first[priority as usize] = true;
                }

                for &priority in sack.iter().skip(sack_len / 2) {
                    if in_first[priority as usize] {
                        return priority as u16;
                    }
                }

                panic!("Found no duplicate")
            })
            .sum::<u16>(),
    )
}
