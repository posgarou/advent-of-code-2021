use std::fs::read_to_string;

use day07::{CrabSwarm, MovementCostStrategy};

fn main() {
    let input: Vec<i32> = read_to_string("data/input.txt")
        .map(|file| file.split(",").map(str::parse).flatten().collect())
        .unwrap();

    let swarm = CrabSwarm::from(&input);

    println!(
        "Lowest cost position (fixed cost): {:?}",
        swarm.get_best_alignment(&MovementCostStrategy::Fixed)
    );
    println!(
        "Lowest cost position (linear cost): {:?}",
        swarm.get_best_alignment(&MovementCostStrategy::Linear)
    );
}
