use std::fs;

pub fn from(filepath: &str) -> Vec<String> {
  fs::read_to_string(filepath)
    .expect("Unable to read input file")
    .lines()
    .map(str::to_string)
    .collect()
}
