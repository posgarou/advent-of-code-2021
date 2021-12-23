#[derive(Debug)]
pub struct HorizontalPosition(i32);

#[derive(Debug)]
pub enum MovementCostStrategy {
    Fixed,
    Linear,
}

#[derive(Debug)]
pub struct PositionCost {
    pub position: HorizontalPosition,
    pub cost: i32,
}

#[derive(Debug)]
pub struct CrabSwarm {
    crab_positions: Vec<HorizontalPosition>,
}

impl From<&Vec<i32>> for CrabSwarm {
    fn from(crab_positions: &Vec<i32>) -> Self {
        CrabSwarm {
            crab_positions: crab_positions
                .iter()
                .map(|x| HorizontalPosition(*x))
                .collect(),
        }
    }
}

impl CrabSwarm {
    pub fn get_alignment_cost(
        &self,
        position: &HorizontalPosition,
        strategy: &MovementCostStrategy,
    ) -> i32 {
        let HorizontalPosition(x1) = position;

        let costs = self.crab_positions.iter().map(|HorizontalPosition(x2)| {
            let distance = (x1 - x2).abs();

            match strategy {
                &MovementCostStrategy::Fixed => distance,
                &MovementCostStrategy::Linear => (0..=distance).sum(),
            }
        });

        costs.sum()
    }

    pub fn get_best_alignment(&self, strategy: &MovementCostStrategy) -> PositionCost {
        let min_position = self
            .crab_positions
            .iter()
            .map(|HorizontalPosition(x)| *x)
            .min()
            .unwrap_or(0);

        let max_position = self
            .crab_positions
            .iter()
            .map(|HorizontalPosition(x)| *x)
            .max()
            .unwrap_or(0);

        let candidates: Vec<PositionCost> = (min_position..=max_position)
            .map(|x| {
                let position = HorizontalPosition(x);
                let cost = self.get_alignment_cost(&position, strategy);

                PositionCost { position, cost }
            })
            .collect();

        candidates
            .into_iter()
            .min_by(|left, right| left.cost.cmp(&right.cost))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_matches_fixed_cost_fixture() {
        let swarm = CrabSwarm::from(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);

        assert_eq!(
            swarm.get_alignment_cost(&HorizontalPosition(2), &MovementCostStrategy::Fixed),
            37
        );

        assert_eq!(
            swarm.get_alignment_cost(&HorizontalPosition(1), &MovementCostStrategy::Fixed),
            41
        );

        assert_eq!(
            swarm.get_alignment_cost(&HorizontalPosition(3), &MovementCostStrategy::Fixed),
            39
        );

        assert_eq!(
            swarm.get_alignment_cost(&HorizontalPosition(10), &MovementCostStrategy::Fixed),
            71
        );

        let PositionCost { position, cost } =
            swarm.get_best_alignment(&MovementCostStrategy::Fixed);

        assert_eq!(position.0, 2);
        assert_eq!(cost, 37);
    }

    #[test]
    fn it_matches_linear_cost_fixture() {
        let swarm = CrabSwarm::from(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);

        assert_eq!(
            swarm.get_alignment_cost(&HorizontalPosition(5), &MovementCostStrategy::Linear),
            168
        );
        assert_eq!(
            swarm.get_alignment_cost(&HorizontalPosition(2), &MovementCostStrategy::Linear),
            206
        );

        let PositionCost { position, cost } =
            swarm.get_best_alignment(&MovementCostStrategy::Linear);

        assert_eq!(position.0, 5);
        assert_eq!(cost, 168);
    }
}
