use crate::day01::Input;

const INPUT: &str = include_str!("../../input/01/input.txt");

pub fn read() -> Input {
    // println!("{}", INPUT);

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

    // let input: Vec<u32> = INPUT.split("\n")
    //     .map(|x| x.trim())
    //     .filter(|x| !x.is_empty())
    //     .map(|x| x.parse().unwrap())
    //     .collect();
    o
}
