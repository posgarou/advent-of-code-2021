use std::fs::read_to_string;

use day07::CrabSwarm;

fn main() {
    let input: Vec<i32> = read_to_string("data/input.txt")
        .map(|file| file.split(",").map(str::parse).flatten().collect())
        .unwrap();

    let swarm = CrabSwarm::from(&input);

    println!("Lowest cost position: {:?}", swarm.get_best_alignment());
}
