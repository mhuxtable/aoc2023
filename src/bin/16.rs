use std::collections::HashSet;

use advent_of_code::Grid;
use itertools::Itertools;

advent_of_code::solution!(16);

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    Mirror(MirrorType),
    Splitter(SplitterType),
}

#[derive(Clone, Debug, PartialEq)]
enum MirrorType {
    Forward,
    Backward,
}

#[derive(Clone, Debug, PartialEq)]
enum SplitterType {
    Vertical,
    Horizontal,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '/' => Cell::Mirror(MirrorType::Forward),
            '\\' => Cell::Mirror(MirrorType::Backward),
            '|' => Cell::Splitter(SplitterType::Vertical),
            '-' => Cell::Splitter(SplitterType::Horizontal),
            _ => panic!("Invalid cell type"),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => ".",
                Cell::Mirror(MirrorType::Forward) => "/",
                Cell::Mirror(MirrorType::Backward) => "\\",
                Cell::Splitter(SplitterType::Vertical) => "|",
                Cell::Splitter(SplitterType::Horizontal) => "-",
            }
        )
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum BeamDirection {
    Left,
    Right,
    Up,
    Down,
}

fn intersect_beam_with_cell(
    grid: &Grid<Cell>,
    beam: ((usize, usize), BeamDirection),
) -> Option<Vec<((usize, usize), BeamDirection)>> {
    let ((x, y), dir) = beam;
    let cell = grid.get(x, y).unwrap();

    #[cfg(debug_assertions)]
    println!("intersect: {:?} {:?}", beam, cell);

    match (cell, dir) {
        (Cell::Empty, _) => None,
        (Cell::Mirror(MirrorType::Forward), BeamDirection::Left) => {
            if y < grid.height() - 1 {
                Some(vec![((x, y + 1), BeamDirection::Down)])
            } else {
                Some(vec![])
            }
        }
        (Cell::Mirror(MirrorType::Forward), BeamDirection::Right) => {
            if y > 0 {
                Some(vec![((x, y - 1), BeamDirection::Up)])
            } else {
                Some(vec![])
            }
        }
        (Cell::Mirror(MirrorType::Forward), BeamDirection::Up) => {
            if x < grid.width() - 1 {
                Some(vec![((x + 1, y), BeamDirection::Right)])
            } else {
                Some(vec![])
            }
        }
        (Cell::Mirror(MirrorType::Forward), BeamDirection::Down) => {
            if x > 0 {
                Some(vec![((x - 1, y), BeamDirection::Left)])
            } else {
                Some(vec![])
            }
        }
        (Cell::Mirror(MirrorType::Backward), BeamDirection::Left) => {
            if y > 0 {
                Some(vec![((x, y - 1), BeamDirection::Up)])
            } else {
                Some(vec![])
            }
        }
        (Cell::Mirror(MirrorType::Backward), BeamDirection::Right) => {
            if y < grid.height() - 1 {
                Some(vec![((x, y + 1), BeamDirection::Down)])
            } else {
                Some(vec![])
            }
        }
        (Cell::Mirror(MirrorType::Backward), BeamDirection::Up) => {
            if x > 0 {
                Some(vec![((x - 1, y), BeamDirection::Left)])
            } else {
                Some(vec![])
            }
        }
        (Cell::Mirror(MirrorType::Backward), BeamDirection::Down) => {
            if x < grid.width() - 1 {
                Some(vec![((x + 1, y), BeamDirection::Right)])
            } else {
                Some(vec![])
            }
        }
        (Cell::Splitter(SplitterType::Horizontal), BeamDirection::Up)
        | (Cell::Splitter(SplitterType::Horizontal), BeamDirection::Down) => {
            let mut next = vec![];
            if x < grid.width() - 1 {
                next.push(((x + 1, y), BeamDirection::Right));
            }
            if x > 0 {
                next.push(((x - 1, y), BeamDirection::Left));
            }

            Some(next)
        }
        (Cell::Splitter(SplitterType::Vertical), BeamDirection::Left)
        | (Cell::Splitter(SplitterType::Vertical), BeamDirection::Right) => {
            let mut next = vec![];
            if y < grid.height() - 1 {
                next.push(((x, y + 1), BeamDirection::Down));
            }
            if y > 0 {
                next.push(((x, y - 1), BeamDirection::Up));
            }

            Some(next)
        }
        (Cell::Splitter(_), _) => None,
    }
}

fn energised(grid: &Grid<Cell>, initial: ((usize, usize), BeamDirection)) -> u32 {
    let mut energised: HashSet<((usize, usize), BeamDirection)> = HashSet::new();

    #[cfg(debug_assertions)]
    println!("{}", grid);

    let mut beams = vec![initial];
    let mut new_beams = vec![];

    loop {
        if beams.is_empty() {
            break;
        }

        for (beam, dir) in beams.drain(..) {
            let (x, y) = beam;

            if energised.contains(&(beam, dir)) {
                continue;
            }

            energised.insert((beam, dir));

            if let Some(add_beams) = intersect_beam_with_cell(&grid, (beam, dir)) {
                new_beams.extend(add_beams);
                continue;
            }

            #[cfg(debug_assertions)]
            println!("{:?} {:?}", beam, dir);

            let coords_it: Box<dyn Iterator<Item = (usize, usize)>> = match dir {
                BeamDirection::Left => {
                    if x == 0 {
                        continue;
                    }

                    Box::new((0..x).rev().map(|x| (x, y)))
                }
                BeamDirection::Right => Box::new((x + 1..grid.width()).map(|x| (x, y))),
                BeamDirection::Up => {
                    if y == 0 {
                        continue;
                    }

                    Box::new((0..y).rev().map(|y| (x, y)))
                }
                BeamDirection::Down => Box::new((y + 1..grid.height()).map(|y| (x, y))),
            };

            for (x, y) in coords_it {
                #[cfg(debug_assertions)]
                println!("it: {:?} {:?}", (x, y), dir);
                energised.insert(((x, y), dir));

                if let Some(add_beams) = intersect_beam_with_cell(&grid, ((x, y), dir)) {
                    #[cfg(debug_assertions)]
                    println!("it x: {:?}", add_beams);
                    new_beams.extend(add_beams);
                    break;
                }
            }
        }

        #[cfg(debug_assertions)]
        println!("new beams: {:?}", new_beams);
        beams.extend(new_beams.drain(..));
    }

    #[cfg(debug_assertions)]
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if energised.contains(&((x, y), BeamDirection::Left))
                || energised.contains(&((x, y), BeamDirection::Right))
                || energised.contains(&((x, y), BeamDirection::Up))
                || energised.contains(&((x, y), BeamDirection::Down))
            {
                print!("X");
            } else {
                print!(".");
            }
        }

        print!("\n");
    }

    energised
        .into_iter()
        .map(|((x, y), _)| (x, y))
        .unique()
        .count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<Cell> = Grid::parse(input).unwrap();
    Some(energised(&grid, ((0, 0), BeamDirection::Right)))
}

pub fn part_two<'a>(input: &'a str) -> Option<u32> {
    let grid: Grid<Cell> = Grid::parse(input).unwrap();

    let (width, height) = (grid.width(), grid.height());

    let a = (0..width)
        .zip(std::iter::repeat(0))
        .map(|(x, y)| energised(&grid, ((x, y), BeamDirection::Down)))
        .max()
        .unwrap_or(0);
    let b = (0..width)
        .zip(std::iter::repeat(height - 1))
        .map(|(x, y)| energised(&grid, ((x, y), BeamDirection::Up)))
        .max()
        .unwrap_or(0);
    let c = std::iter::repeat(0)
        .zip(0..height)
        .map(|(x, y)| energised(&grid, ((x, y), BeamDirection::Right)))
        .max()
        .unwrap_or(0);
    let d = std::iter::repeat(width - 1)
        .zip(0..height)
        .map(|(x, y)| energised(&grid, ((x, y), BeamDirection::Left)))
        .max()
        .unwrap_or(0);

    [a, b, c, d].iter().cloned().max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }

    #[test]
    fn toy_tests() {
        assert_eq!(
            part_one(
                r"\./.-.\
-\.....
/./.|./
\-|...."
            ),
            Some(22)
        );

        assert_eq!(
            part_one(
                r"...\...
.../..."
            ),
            Some(8)
        );

        assert_eq!(
            part_one(
                r"..-.|-\
....\/.
\.....|
..|.../
\.-...."
            ),
            Some(27)
        );
    }
}
