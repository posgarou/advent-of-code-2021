use std::fs::read_to_string;

use day06::School;

fn main() {
    let input: Vec<i32> = read_to_string("data/input.txt")
        .map(|file| file.split(",").map(str::parse).flatten().collect())
        .unwrap();

    let school_1 = School::from(&input).tick_n(80);
    println!("School size (80 days): {}", school_1.get_total_fish());

    let school_2 = School::from(&input).tick_n(256);
    println!("School size (256 days): {}", school_2.get_total_fish());
}
