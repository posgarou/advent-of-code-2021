use std::{collections::HashMap, hash::Hash};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct LanternFish(i32);

#[derive(Debug, PartialEq, Clone)]
pub struct School {
    pub fish_counts: HashMap<LanternFish, i64>,
    pub days_passed: i32,
}

impl From<&Vec<i32>> for School {
    fn from(fish_days: &Vec<i32>) -> Self {
        let mut fish_counts = HashMap::new();

        for fish in fish_days {
            *fish_counts.entry(LanternFish(*fish)).or_insert(0) += 1;
        }

        School {
            fish_counts,
            days_passed: 0,
        }
    }
}

impl School {
    pub fn get_total_fish(&self) -> i64 {
        self.fish_counts.values().sum()
    }

    pub fn tick(&self) -> School {
        let mut fish_counts = HashMap::new();

        for (LanternFish(days_remaining), count) in &self.fish_counts {
            if days_remaining > &0 {
                *fish_counts
                    .entry(LanternFish(days_remaining - 1))
                    .or_insert(0) += count;
            } else {
                *fish_counts.entry(LanternFish(6)).or_insert(0) += count;
                *fish_counts.entry(LanternFish(8)).or_insert(0) += count;
            }
        }

        School {
            fish_counts,
            days_passed: self.days_passed + 1,
        }
    }

    pub fn tick_n(&self, days: i32) -> School {
        let mut school = School {
            fish_counts: self.fish_counts.clone(),
            days_passed: self.days_passed,
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
        let mut school = School::from(&vec![3, 4, 3, 1, 2]);

        school = school.tick();

        assert_eq!(school.get_total_fish(), 5);
    }

    #[test]
    fn it_matches_fixture_after_80_days() {
        let mut school = School::from(&vec![3, 4, 3, 1, 2]);

        school = school.tick_n(80);

        assert_eq!(school.get_total_fish(), 5934);
    }
}
