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

    let location = day02_2::run(&instructions);

    println!("{:?}", location);
    println!("Product: {}", location.depth * location.distance)
}
