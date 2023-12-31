use crate::Dir::{East, North, South, West};
use crate::PipeKind::{Ground, Horizontal, Seven, Starting, Vertical, F, J, L};
use ndarray::Axis;
use std::fmt::{Display, Formatter};
advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let (steps, _) = solve(input);
    Some(steps as u32 / 2)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, positions) = solve(input);
    Some(shoelace(positions))
}

fn solve(input: &str) -> (i32, Vec<(usize, usize)>) {
    let lines = input.lines().collect::<Vec<_>>();
    let x_length = lines.first().unwrap().len();
    let y_length = lines.len();
    let mut grid = ndarray::Array2::<Pipe>::default((y_length, x_length));
    let mut start: (usize, usize) = (0, 0);
    for (i, mut row) in grid.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            let c = lines.get(i).unwrap().chars().nth(j).unwrap();
            let pipe = Pipe::from(c);
            if &pipe.pipe_kind == &Starting {
                start = (j, i);
            }
            *col = pipe;
        }
    }
    let mut dir_came_in_on = South;
    let mut steps = 0;
    let mut current_pos = (start.1, start.0);
    let start = current_pos;
    let mut positions = vec![];

    loop {
        let pos = grid.get(current_pos).unwrap();
        dir_came_in_on = pos.get_next(dir_came_in_on).unwrap();
        match dir_came_in_on {
            North => {
                current_pos.0 += 1;
            }
            South => {
                current_pos.0 -= 1;
            }
            East => {
                current_pos.1 -= 1;
            }
            West => {
                current_pos.1 += 1;
            }
        }
        steps += 1;
        positions.push((current_pos.0, current_pos.1));
        if current_pos == start {
            break;
        }
    }
    return (steps, positions);
}

fn shoelace(positions: Vec<(usize, usize)>) -> i32 {
    let mut a = 0i32;
    let mut b = 0i32;
    for x in 0..positions.len() {
        a += (positions[x].0 * positions[(x + 1) % positions.len()].1) as i32;
        b += (positions[x].1 * positions[(x + 1) % positions.len()].0) as i32;
    }
    return (a - b).abs() / 2 - positions.len() as i32 / 2 + 1;
}

impl Pipe {
    fn get_next(&self, input: Dir) -> Option<Dir> {
        return match self.pipe_kind {
            Vertical => {
                return match input {
                    North => Some(North),
                    South => Some(South),
                    East => None,
                    West => None,
                };
            }
            Horizontal => {
                return match input {
                    North => None,
                    South => None,
                    East => Some(East),
                    West => Some(West),
                }
            }
            L => {
                return match input {
                    North => Some(West),
                    South => None,
                    East => Some(South),
                    West => None,
                }
            }
            J => {
                return match input {
                    North => Some(East),
                    South => None,
                    East => None,
                    West => Some(South),
                }
            }
            Seven => {
                return match input {
                    North => None,
                    South => Some(East),
                    East => None,
                    West => Some(North),
                }
            }
            F => {
                return match input {
                    North => None,
                    South => Some(West),
                    East => Some(North),
                    West => None,
                }
            }
            Ground => None,
            Starting => Some(West),
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Pipe {
    pipe_kind: PipeKind,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        Self {
            pipe_kind: PipeKind::from(value),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pipe_kind)
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Self { pipe_kind: Ground }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Display for PipeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Vertical => '|',
            Horizontal => '-',
            L => 'L',
            J => 'J',
            Seven => '7',
            F => 'F',
            Ground => '.',
            Starting => 'S',
        }
        .to_string();
        return write!(f, "{}", str);
    }
}

impl From<char> for PipeKind {
    fn from(value: char) -> Self {
        return match value {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => L,
            'J' => J,
            '7' => Seven,
            'F' => F,
            '.' => Ground,
            'S' => Starting,
            _ => {
                panic!("bad parse")
            }
        };
    }
}

#[derive(Debug, PartialEq, Clone)]
enum PipeKind {
    Vertical,
    Horizontal,
    L,
    J,
    Seven,
    F,
    Ground,
    Starting,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_a() {
        let result = part_one(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        assert_eq!(result, Some(4));
    }
}
