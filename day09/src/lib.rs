#[derive(Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
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
        self.get_low_points().iter().map(|point| 1 + *point).sum()
    }

    fn get_low_points(&self) -> Vec<&u32> {
        let mut low_points = vec![];

        for (y, row) in self.points.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let lower_or_equal_neighbors_count = self
                    .get_neighbors(Point { x, y })
                    .iter()
                    .filter(|neighbor| **neighbor <= cell)
                    .count();

                if lower_or_equal_neighbors_count == 0 {
                    low_points.push(cell);
                }
            }
        }

        low_points
    }

    fn get_neighbors(&self, point: Point) -> Vec<&u32> {
        point
            .neighbors()
            .iter()
            .filter_map(|point| self.get(point))
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
    fn it_matches_fixtures() {
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
}
