#[derive(Debug, PartialEq, Clone)]
pub struct LanternFish(i32);

#[derive(Debug, PartialEq, Clone)]
pub struct School {
    pub fish: Vec<LanternFish>,
    pub days_passed: i32,
}

impl From<Vec<i32>> for School {
    fn from(fish_days: Vec<i32>) -> Self {
        let fish = fish_days
            .iter()
            .map(|days| LanternFish(*days))
            .collect();

        School { fish, days_passed: 0 }
    }
}

impl School {
    pub fn tick(&self) -> School {
        let fish = self.fish
            .iter()
            .flat_map(|fish| {
                let LanternFish(days_remaining) = fish;

                if days_remaining > &0 {
                    vec![LanternFish(days_remaining - 1)]
                } else {
                    vec![LanternFish(6), LanternFish(8)]
                }
            })
            .collect();

        School { fish, days_passed: self.days_passed + 1 }
    }

    pub fn tick_n(&self, days: i32) -> School {
        let mut school = School {
            fish: self.fish.clone(),
            days_passed: self.days_passed
        };

        for _ in 0..days {
            school = school.tick();
        }

        school
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_matches_fixture() {
        let mut school = School {
            days_passed: 0,
            fish: vec![
                LanternFish(3),
                LanternFish(4),
                LanternFish(3),
                LanternFish(1),
                LanternFish(2)
            ]
        };

        school = school.tick();

        assert_eq!(
            school.fish,
            vec![
                LanternFish(2),
                LanternFish(3),
                LanternFish(2),
                LanternFish(0),
                LanternFish(1)
            ]
        );
    }

    #[test]
    fn it_matches_fixture_after_18_days() {
        let mut school = School {
            days_passed: 0,
            fish: vec![
                LanternFish(3),
                LanternFish(4),
                LanternFish(3),
                LanternFish(1),
                LanternFish(2)
            ]
        };

        school = school.tick_n(80);

        assert_eq!(school.fish.len(), 5934);
    }
}
