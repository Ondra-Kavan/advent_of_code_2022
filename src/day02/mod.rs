pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

pub type Input = Vec<(char, char)>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
        assert_eq!(result, 11841);
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
        assert_eq!(result, 13022);
    }
}
