use day09::HeightMap;
use std::fs::read_to_string;

fn main() {
    let digits: Vec<Vec<u32>> = read_to_string("data/input.txt")
        .map(|contents| {
            contents
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap())
                        .collect()
                })
                .collect()
        })
        .unwrap();

    let map = HeightMap { points: digits };

    println!(
        "Risk total for low points: {:?}",
        map.get_low_point_risk_total()
    );

    println!(
        "Largest three basin product: {:?}",
        map.get_three_largest_basin_product()
    );
}
