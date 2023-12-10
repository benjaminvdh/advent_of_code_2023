#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dir {
    V,
    H,
    L,
    J,
    T,
    F,
    G,
    S,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            '|' => Dir::V,
            '-' => Dir::H,
            'L' => Dir::L,
            'J' => Dir::J,
            '7' => Dir::T,
            'F' => Dir::F,
            '.' => Dir::G,
            'S' => Dir::S,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    x: u64,
    y: u64,
}

impl From<(usize, usize)> for Pos {
    fn from(coords: (usize, usize)) -> Self {
        Self {
            x: coords.0 as u64,
            y: coords.1 as u64,
        }
    }
}

impl From<(u64, u64)> for Pos {
    fn from(coords: (u64, u64)) -> Self {
        Self {
            x: coords.0,
            y: coords.1,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid(Vec<Vec<Dir>>);

impl Grid {
    pub fn get_start(&self) -> Pos {
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                if matches!(self.0[y][x], Dir::S) {
                    return (x, y).into();
                }
            }
        }

        panic!()
    }

    pub fn get(&self, pos: Pos) -> Dir {
        self.0[pos.y as usize][pos.x as usize]
    }

    pub fn get_next(&self, curr: Pos, prev: Pos) -> Pos {
        match self.get(curr) {
            Dir::V => (
                curr.x,
                if prev.y == curr.y - 1 {
                    curr.y + 1
                } else {
                    curr.y - 1
                },
            ),
            Dir::H => (
                if prev.x == curr.x - 1 {
                    curr.x + 1
                } else {
                    curr.x - 1
                },
                curr.y,
            ),
            Dir::L => {
                if prev.x == curr.x {
                    (curr.x + 1, curr.y)
                } else {
                    (curr.x, curr.y - 1)
                }
            }
            Dir::J => {
                if prev.x == curr.x {
                    (curr.x - 1, curr.y)
                } else {
                    (curr.x, curr.y - 1)
                }
            }
            Dir::T => {
                if prev.x == curr.x {
                    (curr.x - 1, curr.y)
                } else {
                    (curr.x, curr.y + 1)
                }
            }
            Dir::F => {
                if prev.x == curr.x {
                    (curr.x + 1, curr.y)
                } else {
                    (curr.x, curr.y + 1)
                }
            }
            _ => panic!(),
        }
        .into()
    }

    pub fn are_connected(&self, start_pos: Pos, cell_pos: Pos) -> bool {
        match self.get(cell_pos) {
            Dir::V => cell_pos.x == start_pos.x,
            Dir::H => cell_pos.y == start_pos.y,
            Dir::L => {
                (cell_pos.x == start_pos.x && cell_pos.y == start_pos.y + 1)
                    || (cell_pos.x == start_pos.x - 1 && cell_pos.y == start_pos.y)
            }
            Dir::J => {
                (cell_pos.x == start_pos.x && cell_pos.y == start_pos.y + 1)
                    || (cell_pos.x == start_pos.x + 1 && cell_pos.y == start_pos.y)
            }
            Dir::T => {
                (cell_pos.x == start_pos.x && cell_pos.y == start_pos.y - 1)
                    || (cell_pos.x == start_pos.x + 1 && cell_pos.y == start_pos.y)
            }
            Dir::F => {
                (cell_pos.x == start_pos.x && cell_pos.y == start_pos.y - 1)
                    || (cell_pos.x == start_pos.x - 1 && cell_pos.y == start_pos.y)
            }
            Dir::G => false,
            Dir::S => false,
        }
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = Grid;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(input: &str) -> Self::Input {
        Grid(input.lines().map(|line| parse_line(line)).collect())
    }

    fn part_1(grid: &Self::Input) -> Self::Output1 {
        let start = grid.get_start();

        let next_steps = [
            Some((start.x + 1, start.y).into()),
            Some((start.x, start.y + 1).into()),
            start.x.checked_sub(1).map(|x| (x, start.y).into()),
            start.y.checked_sub(1).map(|y| (start.x, y).into()),
        ]
        .into_iter()
        .filter_map(|p| p);

        for step in next_steps {
            if !grid.are_connected(start, step) {
                continue;
            }

            let mut prev = start;
            let mut curr = step;
            let mut num_steps = 0;

            loop {
                let next = grid.get_next(curr, prev);
                num_steps += 1;

                if matches!(grid.get(next), Dir::S) {
                    return num_steps / 2;
                }

                prev = curr;
                curr = next;
            }
        }

        panic!()
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn parse_line(line: &str) -> Vec<Dir> {
    line.chars().map(|c| Dir::from(c)).collect()
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        Grid(vec![
            vec![Dir::G, Dir::G, Dir::F, Dir::T, Dir::G],
            vec![Dir::G, Dir::F, Dir::J, Dir::V, Dir::G],
            vec![Dir::S, Dir::J, Dir::G, Dir::L, Dir::T],
            vec![Dir::V, Dir::F, Dir::H, Dir::H, Dir::J],
            vec![Dir::L, Dir::J, Dir::G, Dir::G, Dir::G],
        ])
    }

    #[test]
    fn parsing() {
        let input = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 8);
    }

    #[test]
    fn part_2() {
        //assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
