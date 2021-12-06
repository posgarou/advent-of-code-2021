use day02_1::{instruction_to_delta, resolve_deltas};

mod read_lines;

fn main() {
    let instructions = read_lines::from("data/input.txt")
        .iter()
        .map(|line| line.split_whitespace())
        .map(|mut parts| (
            parts.next().unwrap().to_string(),
            parts.next().unwrap().parse::<i32>().unwrap()
        ))
        .collect::<Vec<(String, i32)>>();

    let deltas = instructions.iter()
        .map(instruction_to_delta)
        .collect();

    let location = resolve_deltas(deltas);

    println!("({}, {})", location.0, location.1);
    println!("Product: {}", location.0 * location.1);
}
