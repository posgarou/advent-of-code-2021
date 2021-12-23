use std::fs::read_to_string;

use day06::School;

fn main() {
    let input: Vec<i32> = read_to_string("data/input.txt")
        .map(|file| {
            file.split(",").map(str::parse).flatten().collect()
        })
        .unwrap();

    let school = School::from(input).tick_n(80);

    println!("School size: {}", school.fish.len());
}
