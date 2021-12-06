use std::fs;
use day01_1::count_increases;

fn main() {
    let measurements = fs::read_to_string("./data/input.txt")
        .expect("Unable to read input file")
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, std::num::ParseIntError>>()
        .unwrap();

    println!("{}", count_increases(&measurements));
}
