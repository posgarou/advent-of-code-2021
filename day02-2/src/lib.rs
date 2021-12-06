#[derive(Debug)]
pub enum PositionDelta {
  Aim(i32),
  Distance(i32)
}

#[derive(Debug, Default)]
pub struct SubmarinePosition {
  pub depth: i32,
  pub distance: i32,
  aim: i32,
}

impl SubmarinePosition {
  fn move_by(&self, delta: PositionDelta) -> Self {
    match delta {
      PositionDelta::Aim(amount) => SubmarinePosition {
        aim: self.aim + amount,
        ..*self
      },
      PositionDelta::Distance(amount) => SubmarinePosition {
        depth: self.depth + amount * self.aim,
        distance: self.distance + amount,
        ..*self
      }
    }
  }
}

pub fn run(instructions: &Vec<(String, i32)>) -> SubmarinePosition {
  let initial = SubmarinePosition {
    aim: 0,
    depth: 0,
    distance: 0
  };

  resolve_deltas(
    initial,
    instructions.iter()
      .map(instruction_to_delta)
      .collect()
  )
}

fn instruction_to_delta((name, amount): &(String, i32)) -> PositionDelta {
  match name.as_str() {
    "forward" => PositionDelta::Distance(*amount),
    "down" => PositionDelta::Aim(*amount),
    "up" => PositionDelta::Aim(-1 * amount),
    _ => PositionDelta::Aim(0)
  }
}

fn resolve_deltas(
  initial_position: SubmarinePosition,
  deltas: Vec<PositionDelta>
) -> SubmarinePosition {
  deltas.into_iter()
  .fold(initial_position, |current, delta| {
    current.move_by(delta)
  })
}

#[cfg(test)]
mod test {
  use crate::run;

  #[test]
  fn it_matches_example_in_specs() {
    let instructions = vec![
      ("forward", 5),
      ("down", 5),
      ("forward", 8),
      ("up", 3),
      ("down", 8),
      ("forward", 2)
    ].into_iter().map(|(name, amount)| {
      ( name.to_string(), amount )
    }).collect();

    let position = run(&instructions);

    assert_eq!(position.distance, 15);
    assert_eq!(position.depth, 60)
  }
}
