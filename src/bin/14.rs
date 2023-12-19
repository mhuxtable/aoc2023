use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use advent_of_code::Grid;

advent_of_code::solution!(14);

#[derive(Clone, PartialEq)]
enum Rock {
    Empty,
    Round,
    Cube,
}

impl Default for Rock {
    fn default() -> Self {
        Rock::Empty
    }
}

impl std::fmt::Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock::Empty => write!(f, "."),
            Rock::Round => write!(f, "O"),
            Rock::Cube => write!(f, "#"),
        }
    }
}

impl From<char> for Rock {
    fn from(c: char) -> Self {
        match c {
            '.' => Rock::Empty,
            'O' => Rock::Round,
            '#' => Rock::Cube,
            _ => panic!("Invalid rock type"),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn roll_grid(grid: &Grid<Rock>, direction: Direction) -> Grid<Rock> {
    let mut rolled_grid = Grid::new(Rock::Empty, grid.width(), grid.height());

    let ys: Box<dyn Iterator<Item = usize>> = match direction {
        Direction::North | Direction::East | Direction::West => Box::new(0..grid.height()),
        Direction::South => Box::new((0..grid.height()).rev()),
    };

    let xs = || -> Box<dyn Iterator<Item = usize>> {
        match direction {
            Direction::North | Direction::South | Direction::West => Box::new(0..grid.width()),
            Direction::East => Box::new((0..grid.width()).rev()),
        }
    };

    for y in ys {
        for x in xs() {
            let rock = grid.get(x, y).unwrap();

            let (new_x, new_y) = match rock {
                Rock::Empty | Rock::Cube => (x, y),
                Rock::Round => {
                    let it: Box<dyn Iterator<Item = usize>> = match direction {
                        Direction::North => Box::new((0..y).rev()),
                        Direction::South => Box::new(y + 1..grid.height()),
                        Direction::East => Box::new(x + 1..grid.width()),
                        Direction::West => Box::new((0..x).rev()),
                    };

                    let get_coord = |(x, y), coord| match direction {
                        Direction::North | Direction::South => (x, coord),
                        Direction::East | Direction::West => (coord, y),
                    };

                    let start = |(x, y)| match direction {
                        Direction::North | Direction::South => y,
                        Direction::East | Direction::West => x,
                    };

                    let (_, new_coord) =
                        it.fold((false, start((x, y))), |(stuck, new_coord), try_coord| {
                            if stuck {
                                (stuck, new_coord)
                            } else {
                                let coord = get_coord((x, y), try_coord);
                                let rock = rolled_grid.get(coord.0, coord.1).unwrap();
                                if *rock != Rock::Empty {
                                    (true, new_coord)
                                } else {
                                    (false, try_coord)
                                }
                            }
                        });

                    match direction {
                        Direction::North | Direction::South => (x, new_coord),
                        Direction::East | Direction::West => (new_coord, y),
                    }
                }
            };

            if rock != &Rock::Empty {
                rolled_grid.set(new_x, new_y, rock.clone());
            }
        }
    }

    rolled_grid
}

fn score_grid(g: &Grid<Rock>) -> usize {
    g.into_iter()
        .filter_map(|((_, y), rock)| if rock == Rock::Round { Some(y) } else { None })
        .map(|y| g.height() - y)
        .sum::<usize>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<Rock> = Grid::parse(input).unwrap();
    let rolled_grid = roll_grid(&grid, Direction::North);

    score_grid(&rolled_grid).try_into().ok()
}

fn hash_grid(g: &Grid<Rock>) -> u64 {
    let mut s = DefaultHasher::new();

    for y in 0..g.height() {
        for x in 0..g.width() {
            let rock = g.get(x, y).unwrap();
            s.write(&rock.to_string().as_bytes());
        }
    }

    s.finish()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut start_grid: Grid<Rock> = Grid::parse(input).unwrap();

    let mut grids_seen = Vec::new();
    let mut cycle: Option<(usize, usize)> = None;

    for i in 0..1_000_000_000 {
        [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .iter()
        .for_each(|d| {
            start_grid = roll_grid(&start_grid, *d);
        });

        let hash = hash_grid(&start_grid);

        if let Some((_, cycle_start, _)) = grids_seen.iter().find(|(h, _, _)| *h == hash) {
            cycle = Some((*cycle_start, i));
            break;
        }

        grids_seen.push((hash, i, score_grid(&start_grid)));
    }

    assert!(cycle.is_some());
    let (cycle_start, cycle_end) = cycle.unwrap();
    let len = cycle_end - cycle_start;

    let steps_remaining = (1_000_000_000 - cycle_start) % len;
    let (_, _, score) = grids_seen[cycle_start + steps_remaining - 1];

    score.try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
