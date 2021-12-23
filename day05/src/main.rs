use day05::{count_overlapping_points, Point, ThermalVent};
use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let thermal_vent_regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let input = read_to_string("data/input.txt").unwrap();

    let vents: Vec<ThermalVent> = thermal_vent_regex
        .captures_iter(&input)
        .filter_map(|capture| {
            let groups = (
                capture.get(1).and_then(|x| x.as_str().parse().ok()),
                capture.get(2).and_then(|y| y.as_str().parse().ok()),
                capture.get(3).and_then(|x| x.as_str().parse().ok()),
                capture.get(4).and_then(|y| y.as_str().parse().ok()),
            );

            match groups {
                (Some(x1), Some(y1), Some(x2), Some(y2)) => Some(ThermalVent {
                    start: Point { x: x1, y: y1 },
                    end: Point { x: x2, y: y2 },
                }),
                _ => None,
            }
        })
        .collect();

    let point_counts = count_overlapping_points(vents);

    let overlapping_points_count = point_counts.values().filter(|count| **count >= 2).count();

    println!("Overlapping points: {:?}", overlapping_points_count);
}
