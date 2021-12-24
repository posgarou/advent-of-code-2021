use std::{cmp::Reverse, collections::HashSet};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PositionHeight {
    position: Point,
    height: u32,
}

impl Point {
    pub fn neighbors(&self) -> Vec<Point> {
        let Point { x, y } = self;

        let mut points = vec![Point { x: x + 1, y: *y }, Point { x: *x, y: y + 1 }];

        // prevent subtraction overflow
        if x > &0 {
            points.push(Point { x: x - 1, y: *y })
        }

        // prevent subtraction overflow
        if y > &0 {
            points.push(Point { x: *x, y: y - 1 })
        }

        points
    }
}

#[derive(Debug)]
pub struct HeightMap {
    pub points: Vec<Vec<u32>>,
}

impl HeightMap {
    pub fn get_low_point_risk_total(&self) -> u32 {
        self.get_low_points()
            .iter()
            .map(|point| 1 + point.height)
            .sum()
    }

    pub fn get_three_largest_basin_product(&self) -> usize {
        let mut basin_sizes: Vec<usize> = self
            .get_low_points()
            .iter()
            .map(|low_point| self.get_basin(low_point.position))
            .map(|basin| basin.len())
            .collect();

        basin_sizes.sort_by_key(|w| Reverse(*w));

        basin_sizes.iter().take(3).product()
    }

    fn get_basin(&self, point: Point) -> Vec<PositionHeight> {
        let mut basin = HashSet::new();
        let mut queue = Vec::from([PositionHeight {
            position: point,
            height: *self.get(&point).unwrap(),
        }]);

        let mut index = 0;

        while index < queue.len() {
            let PositionHeight { height, position } = queue[index];
            basin.insert(PositionHeight { height, position });
            index += 1;

            let candidates = self
                .get_neighbors(position)
                .into_iter()
                .filter(|position_height| position_height.height < 9)
                .filter(|position_height| !basin.contains(position_height));

            for candidate in candidates {
                queue.push(candidate);
            }
        }

        basin.into_iter().collect()
    }

    fn get_low_points(&self) -> Vec<PositionHeight> {
        let mut low_points = vec![];

        for (y, row) in self.points.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let lower_or_equal_neighbors_count = self
                    .get_neighbors(Point { x, y })
                    .iter()
                    .filter(|neighbor| neighbor.height <= *cell)
                    .count();

                if lower_or_equal_neighbors_count == 0 {
                    low_points.push(PositionHeight {
                        position: Point { x, y },
                        height: *cell,
                    });
                }
            }
        }

        low_points
    }

    fn get_neighbors(&self, point: Point) -> Vec<PositionHeight> {
        point
            .neighbors()
            .iter()
            .filter_map(|point| {
                self.get(point).map(|height| PositionHeight {
                    position: *point,
                    height: *height,
                })
            })
            .collect()
    }

    fn get(&self, point: &Point) -> Option<&u32> {
        self.points.get(point.y).and_then(|row| row.get(point.x))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_calculates_risk_total_to_match_fixtures() {
        let points = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        let map = HeightMap { points };

        assert_eq!(map.get_low_point_risk_total(), 15);
    }

    #[test]
    fn it_calculates_basins_to_match_fixtures() {
        let map = HeightMap {
            points: vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
            ],
        };

        assert_eq!(map.get_basin(Point { x: 1, y: 0 }).len(), 3);
        assert_eq!(map.get_basin(Point { x: 9, y: 0 }).len(), 9);
        assert_eq!(map.get_basin(Point { x: 2, y: 2 }).len(), 14);
    }
}
