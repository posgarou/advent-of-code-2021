use std::{cmp, collections::HashMap};

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Default, PartialEq)]
pub struct ThermalVent {
    pub start: Point,
    pub end: Point,
}

impl ThermalVent {
    pub fn get_covered_points(&self) -> Option<Vec<Point>> {
        if self.start.x == self.end.x {
            let y_min = cmp::min(self.start.y, self.end.y);
            let y_max = cmp::max(self.start.y, self.end.y);

            let points = (y_min..=y_max)
                .map(|y| Point { x: self.start.x, y })
                .collect();

            return Some(points);
        }

        if self.start.y == self.end.y {
            let x_min = cmp::min(self.start.x, self.end.x);
            let x_max = cmp::max(self.start.x, self.end.x);

            let points = (x_min..=x_max)
                .map(|x| Point { x, y: self.start.y })
                .collect();

            return Some(points);
        }

        println!(
            "Undefined behavior for diagonal lines, {:?} -> {:?}",
            self.start, self.end
        );

        None
    }
}

pub fn count_overlapping_points(vents: Vec<ThermalVent>) -> HashMap<Point, i32> {
    let mut point_counts = HashMap::new();

    for vent in vents {
        match vent.get_covered_points() {
            Some(points) => {
                for point in points {
                    *point_counts.entry(point).or_insert(0) += 1
                }
            }
            None => {
                println!("A vent had no covered points")
            }
        }
    }

    return point_counts;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_correctly_returns_vertical_points() {
        let vent = ThermalVent {
            start: Point { x: 1, y: 1 },
            end: Point { x: 1, y: 3 },
        };

        assert_eq!(
            vent.get_covered_points().unwrap(),
            vec![
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 1, y: 3 }
            ]
        );
    }

    #[test]
    fn it_correctly_returns_horizontal_points() {
        let vent = ThermalVent {
            start: Point { x: 9, y: 7 },
            end: Point { x: 7, y: 7 },
        };

        assert_eq!(
            vent.get_covered_points().unwrap(),
            vec![
                Point { x: 7, y: 7 },
                Point { x: 8, y: 7 },
                Point { x: 9, y: 7 },
            ]
        )
    }
}
