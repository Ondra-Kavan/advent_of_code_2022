pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

pub type Input = Vec<u32>;

pub fn run(part: Part) -> Output {
    let mut input = input::read();
    match part {
        Part::One => part1::solve(&mut input),
        Part::Two => part2::solve(&mut input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        // println!("{result}");
        assert_eq!(result, 72602);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        // println!("{result}");
        assert_eq!(result, 207410);
    }
}
