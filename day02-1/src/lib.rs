pub fn instruction_to_delta((name, delta): &(String, i32)) -> (i32, i32) {
  match name.as_str() {
    "forward" => (*delta, 0),
    "down" => (0, *delta),
    "up" => (0, delta * -1),
    _ => (0, 0)
  }
}

pub fn resolve_deltas(deltas: Vec<(i32, i32)>) -> (i32, i32) {
  deltas.into_iter()
  .fold((0, 0), |current, delta| (
    current.0 + delta.0,
    current.1 + delta.1
  ))
}
