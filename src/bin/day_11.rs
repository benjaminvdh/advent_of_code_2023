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

#[derive(Clone, Copy, Debug, PartialEq)]
struct Galaxy {
    x: u64,
    y: u64,
}

#[derive(Debug, PartialEq)]
struct StarMap(Vec<Vec<DataPoint>>, Vec<bool>, Vec<bool>); // (datapoints, row_empty, col_empty)

impl StarMap {
    pub fn get_sum_of_distances(&self, scale_factor: u64) -> u64 {
        let galaxies = self.get_galaxies();

        let galaxy_pairs: Vec<_> = galaxies
            .iter()
            .flat_map(|a| galaxies.iter().map(move |b| (*a, *b)))
            .collect();

        galaxy_pairs
            .iter()
            .map(|(a, b)| self.get_distance(a, b, scale_factor))
            .sum::<u64>()
            / 2
    }

    fn get_distance(&self, a: &Galaxy, b: &Galaxy, scale_factor: u64) -> u64 {
        self.get_horizontal_distance(a, b, scale_factor)
            + self.get_vertical_distance(a, b, scale_factor)
    }

    fn get_horizontal_distance(&self, a: &Galaxy, b: &Galaxy, scale_factor: u64) -> u64 {
        let x_min = a.x.min(b.x);
        let x_max = a.x.max(b.x);

        (x_min..x_max).fold(0, |acc, x| {
            acc + if self.is_column_empty(x as usize) {
                scale_factor
            } else {
                1
            }
        })
    }

    fn get_vertical_distance(&self, a: &Galaxy, b: &Galaxy, scale_factor: u64) -> u64 {
        let y_min = a.y.min(b.y);
        let y_max = a.y.max(b.y);

        (y_min..y_max).fold(0, |acc, y| {
            acc + if self.is_row_empty(y as usize) {
                scale_factor
            } else {
                1
            }
        })
    }

    fn get_galaxies(&self) -> Vec<Galaxy> {
        (0..self.0.len())
            .flat_map(|y| {
                (0..self.0[y].len()).filter_map(move |x| {
                    if matches!(self.0[y][x], DataPoint::Galaxy) {
                        Some(Galaxy {
                            x: x as u64,
                            y: y as u64,
                        })
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn is_row_empty(&self, row: usize) -> bool {
        self.1[row]
    }

    fn is_column_empty(&self, col: usize) -> bool {
        self.2[col]
    }
}

impl From<&str> for StarMap {
    fn from(input: &str) -> Self {
        let star_map: Vec<Vec<DataPoint>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();

        let rows_empty = star_map
            .iter()
            .map(|row| {
                row.iter()
                    .all(|data_point| matches!(data_point, DataPoint::EmptySpace))
            })
            .collect();

        let cols_empty = (0..star_map[0].len())
            .map(|col| {
                star_map
                    .iter()
                    .all(|row| matches!(row[col], DataPoint::EmptySpace))
            })
            .collect();

        Self(star_map, rows_empty, cols_empty)
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = StarMap;
    type Output1 = u64;
    type Output2 = u64;

    fn parse(input: &str) -> Self::Input {
        input.into()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.get_sum_of_distances(2)
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input.get_sum_of_distances(1_000_000)
    }
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        StarMap(
            vec![
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
            ],
            vec![
                false, false, false, true, false, false, false, true, false, false,
            ],
            vec![
                false, false, true, false, false, true, false, false, true, false,
            ],
        )
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
        assert_eq!(get_input().get_sum_of_distances(10), 1030);
        assert_eq!(get_input().get_sum_of_distances(100), 8410);
    }
}
