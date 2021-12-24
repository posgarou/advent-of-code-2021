use day10::sum_corrupted_points;
use std::fs::read_to_string;

fn main() {
    match get_corrupted_points() {
        Some(points) => println!("Corrupted point total: {}", points),
        None => println!("Failed to get point total"),
    }
}

fn get_corrupted_points() -> Option<i32> {
    read_to_string("data/input.txt")
        .map(|file| sum_corrupted_points(file.lines().collect()))
        .ok()
}
