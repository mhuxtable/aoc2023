use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use advent_of_code::Grid;

advent_of_code::solution!(21);

#[derive(Clone, PartialEq)]
enum Cell {
    Gardens,
    Rocks,
    Start,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Gardens
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Gardens,
            '#' => Cell::Rocks,
            'S' => Cell::Start,
            _ => panic!("Invalid cell: {}", c),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Gardens => write!(f, "."),
            Cell::Rocks => write!(f, "#"),
            Cell::Start => write!(f, "S"),
        }
    }
}

fn solve_part_one(input: &str, needed_steps: usize) -> u32 {
    let grid: Grid<Cell> = Grid::parse(input).unwrap();
    let start_cell = grid.iter().find(|(_, c)| c == &Cell::Start).unwrap().0;

    println!("{}", grid);
    println!("{:?}", start_cell);

    let mut q = HashSet::new();
    q.insert(start_cell);

    let mut next_q = vec![];

    for _ in 0..needed_steps {
        for coord in q.drain() {
            for (next_coord, next_cell) in grid.neighbours(coord) {
                if next_cell == &Cell::Rocks {
                    continue;
                }

                next_q.push(next_coord);
            }
        }

        q.clear();
        q.extend(next_q.drain(..));
        println!(
            "{}",
            grid.fmt_with_overrides(|&(x, y)| {
                if q.contains(&(x, y)) {
                    Some('O')
                } else {
                    None
                }
            })
        );
    }

    println!("{:?}", q);

    q.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve_part_one(input, 64))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
