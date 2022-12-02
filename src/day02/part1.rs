use crate::day02::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut scores: Vec<i32> = Vec::with_capacity(256);
    let mut left: Vec<i32> = Vec::with_capacity(256);
    let mut right: Vec<i32> = Vec::with_capacity(256);

    for i in 0..input.len() {
        left.push(match input[i] {
            ('A', _) => 1,
            ('B', _) => 2,
            ('C', _) => 3,
            _ => 0,
        });
        right.push(match input[i] {
            (_, 'X') => 1,
            (_, 'Y') => 2,
            (_, 'Z') => 3,
            _ => 0,
        });

        scores.push(match (&left[i], &right[i]) {
            (1,1) => 3+1,
            (1,2) => 6+2,
            (1,3) => 0+3,
            (2,1) => 0+1,
            (2,2) => 3+2,
            (2,3) => 6+3,
            (3,1) => 6+1,
            (3,2) => 0+2,
            (3,3) => 3+3,
            _ => 0,
        })
    }
    Output::I32(scores.iter().sum())

    // let mut left: Vec<Shape> = Vec::with_capacity(256);
    // let mut right: Vec<Shape> = Vec::with_capacity(256);
    // for i in 0..input.len() {
    //     left.push(match input[i] {
    //         ('A', _) => Shape::Rock,
    //         ('B', _) => Shape::Paper,
    //         ('C', _) => Shape::Scissors,
    //         _ => Shape::Rock,
    //     });
    //     right.push(match input[i] {
    //         (_, 'X') => Shape::Rock,
    //         (_, 'Y') => Shape::Paper,
    //         (_, 'Z') => Shape::Scissors,
    //         _ => Shape::Rock,
    //     });
    // }
    // let mut tuple_vec: Vec<(&Shape, &Shape)> = Vec::with_capacity(256);
    // for i in 0..left.len() {
    //     tuple_vec.push((&left[i], &right[i]));
    // }
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
