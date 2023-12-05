use itertools::iproduct;
use std::fmt::Display;

use advent_of_code::Grid;

advent_of_code::solution!(3);

#[derive(Clone, Debug, PartialEq)]
enum CellCharacter {
    Empty,
    Digit(u8),
    Symbol(char),
}

#[derive(Clone, Debug)]
struct SymbolAdjacent(bool);

#[derive(Clone, Debug)]
enum NumberAdjacent {
    None,
    Some(u32),
}

impl std::fmt::Display for SymbolAdjacent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 {
            write!(f, "X")
        } else {
            write!(f, ".")
        }
    }
}

impl std::fmt::Display for NumberAdjacent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberAdjacent::None => write!(f, "."),
            NumberAdjacent::Some(_) => write!(f, "X"),
        }
    }
}

impl Display for CellCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellCharacter::Empty => write!(f, "."),
            CellCharacter::Digit(d) => write!(f, "{}", d),
            CellCharacter::Symbol(c) => write!(f, "{}", c),
        }
    }
}

fn parse<F: Fn(char) -> bool>(
    input: &str,
    is_adjacency: F,
) -> (
    Grid<CellCharacter>,
    Grid<SymbolAdjacent>,
    Grid<NumberAdjacent>,
) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = Grid::new(CellCharacter::Empty, width, height);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let cell = match c {
                '.' => CellCharacter::Empty,
                d if d.is_digit(10) => CellCharacter::Digit(d.to_digit(10).unwrap() as u8),
                c => CellCharacter::Symbol(c),
            };

            grid.set(x, y, cell);
        }
    }

    // Build another grid of positions bordered by symbols
    let mut symbols = Grid::new(SymbolAdjacent(false), width, height);

    for ((x, y), cell) in &grid {
        if let CellCharacter::Symbol(c) = cell {
            if is_adjacency(c) {
                for (x, y) in iproduct!(x.saturating_sub(1)..=x + 1, y.saturating_sub(1)..=y + 1) {
                    if x < grid.width() && y < grid.height() {
                        symbols.set(x, y, SymbolAdjacent(true));
                    }
                }
            }
        }
    }

    // Build another grid where all Digit cells are mapped to the whole number
    // they represent. e.g. in the following grid:
    //
    // 123
    //
    // The number 123 will be mapped to all three cells:
    //
    // 123 123 123
    //
    // This pre-processing step makes it easier to iterate around a symbol and
    // find all adjacent numbers.
    let numbers = {
        let mut numbers = Grid::new(NumberAdjacent::None, width, height);

        let mut number = 0;
        let (mut start_x, mut end_x) = (None, None);

        for ((x, y), cell) in &grid {
            let is_end = if let CellCharacter::Digit(d) = cell {
                number = number * 10 + d as u32;

                if start_x.is_none() {
                    start_x = Some(x);
                }

                end_x = Some(x);

                false
            } else {
                true
            } || x == grid.width() - 1;

            if is_end && start_x.is_some() {
                for x in start_x.unwrap()..=end_x.unwrap() {
                    numbers.set(x, y, NumberAdjacent::Some(number));
                }

                number = 0;
                start_x = None;
                end_x = None;
            }
        }

        numbers
    };

    (grid, symbols, numbers)
}

// iterate all numbers adjacent to a point, calling some function with each adjacent number.
// Arguments are a grid, a number adjacency grid (where every cell contains the actual number
// represented by that cell and its adjacent cells), the point around which to look (diagonally,
// horizontally and vertically), and a callback to call with eligible numbers.
//
// For example, given the following grid:
//
// 123
// 4*6
// 789
//
// The number adjacency grid looks like
//
// 123 123 123
//  4   0   6
// 789 789 789
//
// Calling this function at the point (1, 1) would call the callback with the numbers 123, 4, 6,
// 789. The 'skip' argument is used to skip the case where more than one digit of the number is
// adjacent to the symbol.
fn iterate_around_with_row_skip(
    grid: &Grid<CellCharacter>,
    numbers: &Grid<NumberAdjacent>,
    (x, y): (usize, usize),
) -> Vec<u32> {
    let mut values = vec![];

    for gy in y.saturating_sub(1)..=y + 1 {
        let mut skip = false;

        for gx in x.saturating_sub(1)..=x + 1 {
            // skip for the case where more than one digit of the number is adjacent to the
            // symbol, e.g.
            //
            // 394
            //  *
            //
            // In this case, the numbers grid will contain 394 three times adjacent to *,
            // but only one instance should be included.
            if skip {
                match grid.get(gx, gy).unwrap() {
                    CellCharacter::Symbol(_) | CellCharacter::Empty => {
                        skip = false;
                    }
                    _ => {}
                }

                continue;
            }

            if let NumberAdjacent::Some(n) = numbers.get(gx, gy).unwrap_or(&NumberAdjacent::None) {
                skip = true;
                values.push(*n);
            }
        }
    }

    values
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, symbols, numbers) = parse(input, |_| true);
    println!("{}", numbers);
    println!("{}", symbols);

    grid.iter()
        .filter(|(_, cell)| {
            if let &CellCharacter::Symbol(_) = cell {
                true
            } else {
                false
            }
        })
        .flat_map(|((x, y), _)| iterate_around_with_row_skip(&grid, &numbers, (x, y)))
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, _, numbers) = parse(input, |_| true);

    grid.iter()
        .filter(|(_, s)| *s == CellCharacter::Symbol('*'))
        .map(|((x, y), _)| iterate_around_with_row_skip(&grid, &numbers, (x, y)))
        .filter_map(|gears| {
            // Exactly two gears must be adjacent for it to be a gear
            if gears.len() == 2 {
                Some(gears.iter().product::<u32>())
            } else {
                None
            }
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
