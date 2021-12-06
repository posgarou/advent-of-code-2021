pub fn count_increases(measurements: &[i32]) -> usize {
    let measurements_with_previous = measurements.iter().skip(1).zip(measurements.iter());

    measurements_with_previous.filter(|(current, previous)| current > previous).count()
}

#[cfg(test)]
mod tests {
    use crate::count_increases;

  #[test]
  fn it_returns_0_for_empty_vector() {
    assert_eq!(count_increases(&Vec::new()), 0)
  }

  #[test]
  fn it_returns_0_for_vector_len_1() {
    assert_eq!(count_increases(&[0]), 0)
  }

  #[test]
  fn it_counts_increases() {
    let measurements = vec![1, 2, 3, 4, 5];

    assert_eq!(count_increases(&measurements), 4)
  }

  #[test]
  fn it_does_not_count_decreases() {
    let measurements = vec![5, 4, 3, 2, 1];

    assert_eq!(count_increases(&measurements), 0)
  }

  #[test]
  fn it_does_not_count_same_values() {
    let measurements = vec![1, 1, 1, 1];

    assert_eq!(count_increases(&measurements), 0)
  }

  #[test]
  fn it_matches_example_from_spec() {
    let measurements = vec![199,
    200,
    208,
    210,
    200,
    207,
    240,
    269,
    260,
    263];

    assert_eq!(count_increases(&measurements), 7)
  }
}
