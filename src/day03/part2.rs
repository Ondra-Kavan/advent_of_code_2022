// https://github.com/LinAGKar/advent-of-code-2022-rust/blob/main/day3b/src/main.rs

use crate::day03::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let lines: Vec<_> = input.lines().collect();
    Output::U64(
        lines
            .chunks(3)
            .map(|groups| {
                let mut in_sacks = [[false; 3]; 53];

                for (n, &sack) in groups.iter().enumerate() {
                    for c in sack.chars() {
                        let priority = if c.is_ascii_lowercase() {
                            c as u32 - 'a' as u32 + 1
                        } else if c.is_ascii_uppercase() {
                            c as u32 - 'A' as u32 + 27
                        } else {
                            panic!("{} was not ASCII letter", c)
                        } as usize;
                        in_sacks[priority][n] = true;
                    }
                }

                in_sacks
                    .into_iter()
                    .enumerate()
                    .find_map(|(priority, sacks)| {
                        if sacks.into_iter().all(|x| x) {
                            Some(priority)
                        } else {
                            None
                        }
                    })
                    .unwrap()
            })
            .sum::<usize>() as u64,
    )
}
