use advent_of_code::Grid;
use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Clone, PartialEq)]
enum Ground {
    Ash,
    Rock,
}

impl Default for Ground {
    fn default() -> Self {
        Self::Ash
    }
}

impl From<char> for Ground {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Rock,
            '.' => Self::Ash,
            _ => panic!("invalid ground"),
        }
    }
}

impl std::fmt::Display for Ground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ash => write!(f, "."),
            Self::Rock => write!(f, "#"),
        }
    }
}

fn parse_puzzles(input: &str) -> Result<Vec<Grid<Ground>>, String> {
    let mut puzzles = vec![];
    let mut lines = input.lines().peekable();

    while lines.peek().is_some() {
        let puzzle_lines = lines.take_while_ref(|line| !line.is_empty()).join("\n");
        lines.next(); // skip the empty

        puzzles.push(Grid::parse(puzzle_lines)?);
    }

    Ok(puzzles)
}

#[derive(Debug)]
enum Reflection {
    X,
    Y,
}

fn find_symmetry(puzzle: &Grid<Ground>, find_smudges: bool) -> (Reflection, usize) {
    // vertical lines of symmetry in x
    'x: for x in 1..puzzle.width() {
        let mut last_partial_match = false;

        for y in 0..puzzle.height() {
            for (x1, x2) in (0..x).rev().zip(x..puzzle.width()) {
                if puzzle.get(x1, y) != puzzle.get(x2, y) {
                    if last_partial_match || !find_smudges {
                        continue 'x;
                    } else {
                        last_partial_match = true;
                    }
                }
            }
        }

        if !find_smudges || last_partial_match {
            return (Reflection::X, x);
        }
    }

    // horizontal lines of symmetry in y
    'y: for y in 1..puzzle.height() {
        let mut last_partial_match = false;

        for x in 0..puzzle.width() {
            for (y1, y2) in (0..y).rev().zip(y..puzzle.height()) {
                if puzzle.get(x, y1) != puzzle.get(x, y2) {
                    if last_partial_match || !find_smudges {
                        continue 'y;
                    } else {
                        last_partial_match = true;
                    }
                }
            }
        }

        if !find_smudges || last_partial_match {
            return (Reflection::Y, y);
        }
    }

    panic!("no symmetry found");
}

fn score(reflection: Reflection, coord: usize) -> u32 {
    match reflection {
        Reflection::X => coord as u32,
        Reflection::Y => 100 * coord as u32,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzles = parse_puzzles(input).unwrap();

    puzzles
        .iter()
        .map(|puzzle| {
            let (r, c) = find_symmetry(puzzle, false);
            score(r, c)
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzles = parse_puzzles(input).unwrap();

    puzzles
        .iter()
        .map(|puzzle| {
            let (r, c) = find_symmetry(puzzle, true);
            score(r, c)
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
