#[derive(Clone, Copy, Debug, PartialEq)]
enum DataPoint {
    EmptySpace,
    Galaxy,
}

impl From<char> for DataPoint {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::EmptySpace,
            '#' => Self::Galaxy,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct StarMap(Vec<Vec<DataPoint>>);

impl StarMap {
    pub fn enlarged(&self) -> StarMap {
        let enlarged_rows = self.enlarged_rows();
        let transposed = enlarged_rows.transposed();
        let enlarged = transposed.enlarged_rows();
        enlarged.transposed()
    }

    fn enlarged_rows(&self) -> StarMap {
        let mut new_rows = vec![];

        for row in self.0.iter() {
            if is_empty(&row) {
                new_rows.push([DataPoint::EmptySpace].repeat(row.len()));
                new_rows.push([DataPoint::EmptySpace].repeat(row.len()));
            } else {
                new_rows.push(row.clone());
            }
        }

        Self(new_rows)
    }

    fn transposed(&self) -> StarMap {
        let mut new_rows = vec![];

        for x in 0..self.0[0].len() {
            let mut new_row = vec![];

            for y in 0..self.0.len() {
                new_row.push(self.0[y][x]);
            }

            new_rows.push(new_row);
        }

        Self(new_rows)
    }

    pub fn get_sum_of_distances(&self) -> usize {
        let galaxy_locations = self.get_galaxy_locations();

        galaxy_locations
            .iter()
            .map(|a| {
                galaxy_locations
                    .iter()
                    .map(|b| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
                    .sum::<usize>()
            })
            .sum::<usize>()
            / 2
    }

    fn get_galaxy_locations(&self) -> Vec<(usize, usize)> {
        (0..self.0.len())
            .flat_map(|y| {
                (0..self.0[y].len()).filter_map(move |x| {
                    if matches!(self.0[y][x], DataPoint::Galaxy) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

fn is_empty(row: &[DataPoint]) -> bool {
    row.iter()
        .all(|data_point| matches!(data_point, DataPoint::EmptySpace))
}

impl From<&str> for StarMap {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| line.chars().map(|c| c.into()).collect())
                .collect(),
        )
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = StarMap;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        input.into()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.enlarged().get_sum_of_distances()
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        StarMap(vec![
            vec![
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::Galaxy,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
            ],
            vec![
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::Galaxy,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
            ],
            vec![
                DataPoint::Galaxy,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
            ],
            vec![
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
            ],
            vec![
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::Galaxy,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
            ],
            vec![
                DataPoint::EmptySpace,
                DataPoint::Galaxy,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
            ],
            vec![
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::Galaxy,
            ],
            vec![
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
            ],
            vec![
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::Galaxy,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
            ],
            vec![
                DataPoint::Galaxy,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::Galaxy,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
                DataPoint::EmptySpace,
            ],
        ])
    }

    #[test]
    fn parsing() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 374);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
