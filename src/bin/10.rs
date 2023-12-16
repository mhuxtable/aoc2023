use std::collections::HashMap;

use advent_of_code::Grid;

advent_of_code::solution!(10);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum FieldCell {
    Ground,
    Start,
    Vertical,
    Horizontal,
    NorthEastCorner,
    NorthWestCorner,
    SouthWestCorner,
    SouthEastCorner,
}

impl std::fmt::Display for FieldCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            FieldCell::Ground => '.',
            FieldCell::Start => 'S',
            FieldCell::Vertical => '|',
            FieldCell::Horizontal => '-',
            FieldCell::NorthEastCorner => 'L',
            FieldCell::NorthWestCorner => 'J',
            FieldCell::SouthWestCorner => '7',
            FieldCell::SouthEastCorner => 'F',
        };

        write!(f, "{}", c)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

lazy_static::lazy_static! {
    static ref DIRECTIONS: HashMap<FieldCell, [Direction; 2]> = HashMap::from([
        (FieldCell::Vertical, [Direction::North, Direction::South]),
        (FieldCell::Horizontal, [Direction::East, Direction::West]),
        (FieldCell::NorthEastCorner, [Direction::South, Direction::West]),
        (FieldCell::NorthWestCorner, [Direction::South, Direction::East]),
        (FieldCell::SouthWestCorner, [Direction::North, Direction::East]),
        (FieldCell::SouthEastCorner, [Direction::North, Direction::West]),
    ]);
}

fn walk<F: FnMut(usize, usize, &FieldCell)>(
    grid: &Grid<FieldCell>,
    (start_x, start_y): (usize, usize),
    direction: Direction,
    mut callback: F,
) -> Option<()> {
    let (mut x, mut y) = (start_x, start_y);
    let mut direction = direction;

    loop {
        (x, y) = match direction {
            Direction::North => {
                if y == 0 {
                    return None;
                } else {
                    (x, y - 1)
                }
            }
            Direction::East => {
                if x == grid.width() - 1 {
                    return None;
                } else {
                    (x + 1, y)
                }
            }
            Direction::South => {
                if y == grid.height() - 1 {
                    return None;
                } else {
                    (x, y + 1)
                }
            }
            Direction::West => {
                if x == 0 {
                    return None;
                } else {
                    (x - 1, y)
                }
            }
        };

        let cell = grid.get(x, y).unwrap();

        match cell {
            FieldCell::Ground => {
                return None;
            }
            FieldCell::Start => {
                return Some(());
            }
            cell => {
                let valid_directions = DIRECTIONS.get(cell).unwrap();

                if valid_directions.contains(&direction) {
                    callback(x, y, cell);
                } else {
                    return None;
                }

                direction = match cell {
                    FieldCell::Vertical | FieldCell::Horizontal => direction,
                    FieldCell::NorthEastCorner
                    | FieldCell::SouthWestCorner
                    | FieldCell::NorthWestCorner
                    | FieldCell::SouthEastCorner => valid_directions
                        .iter()
                        .find(|d| **d != direction)
                        .unwrap()
                        .opposite(),
                    _ => panic!("invalid direction for next step"),
                };
            }
        }
    }
}

fn parse(input: &str) -> (Grid<FieldCell>, (usize, usize)) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = Grid::new(FieldCell::Ground, width, height);
    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let cell = match c {
                '.' => FieldCell::Ground,
                'S' => {
                    start = Some((x, y));

                    FieldCell::Start
                }
                '|' => FieldCell::Vertical,
                '-' => FieldCell::Horizontal,
                'L' => FieldCell::NorthEastCorner,
                'J' => FieldCell::NorthWestCorner,
                '7' => FieldCell::SouthWestCorner,
                'F' => FieldCell::SouthEastCorner,
                _ => panic!("Unknown character: {}", c),
            };

            grid.set(x, y, cell);
        }
    }

    println!("{}", grid);

    (grid, start.unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start) = parse(input);
    let (start_x, start_y) = start;

    let path_length = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .filter_map(|&start_dir| {
        let mut path_length = 0;

        walk(&grid, (start_x, start_y), start_dir, |_, _, _| {
            path_length += 1;
        })
        .map(|_| path_length)
    })
    .max()
    .unwrap();

    Some((path_length + 1) / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut grid, (start_x, start_y)) = parse(input);

    let (start_direction, path_points): (Vec<_>, Vec<_>) = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .filter_map(|&start_dir| {
        let mut path_points = vec![(start_x, start_y)];

        walk(&grid, (start_x, start_y), start_dir, |x, y, _| {
            path_points.push((x, y));
        })
        .map(|_| (start_dir, path_points))
    })
    .unzip();

    // there should be precisely two paths, which are just the reverse of each other
    assert!(path_points.len() == 2);
    let path = &path_points[0];

    // replace start in the grid with the actual corner piece it represents
    {
        assert!(start_direction.len() == 2);

        let start_corner = {
            let find_corner = |a, b| match (a, b) {
                (Direction::North, Direction::East) => Some(FieldCell::NorthEastCorner),
                (Direction::North, Direction::West) => Some(FieldCell::NorthWestCorner),
                (Direction::South, Direction::West) => Some(FieldCell::SouthWestCorner),
                (Direction::South, Direction::East) => Some(FieldCell::SouthEastCorner),
                (Direction::North, Direction::South) => Some(FieldCell::Vertical),
                (Direction::East, Direction::West) => Some(FieldCell::Horizontal),
                _ => None,
            };

            find_corner(start_direction[0], start_direction[1])
                .or_else(|| find_corner(start_direction[1], start_direction[0]))
                .unwrap()
        };

        grid.set(start_x, start_y, start_corner);
    }

    let mut enclosed_cells = 0;

    lazy_static::lazy_static! {
        // OPENING_CORNER_PAIRS defines pairs which, if observed in succession on the path (in
        // either order), indicate that the path has entered or exited an enclosed area. For corner
        // pieces "a" and "b", this array must be checked for both (a, b) and (b, a).
        static ref OPENING_CORNER_PAIRS: [(FieldCell, FieldCell); 4] = [
            (FieldCell::NorthEastCorner, FieldCell::SouthWestCorner),
            (FieldCell::NorthWestCorner, FieldCell::SouthEastCorner),
            (FieldCell::SouthWestCorner, FieldCell::NorthEastCorner),
            (FieldCell::SouthEastCorner, FieldCell::NorthWestCorner),
        ];
    }

    // The idea here is to do horizontal ray tracing across the grid and book keep the number of
    // times entering or leaving the loop. For any piece not on the path that is encountered when
    // the number of intersections with the way is odd, the cell is enclosed.
    //
    // Corner pieces require additional bookkeeping. The first corner piece encountered in a row
    // cannot be used to determine whether the path is entering or leaving the loop, but tracking
    // until the next corner piece enables a determination to be made. Corner pieces going in
    // opposite directions indicate the loop was entered, e.g. north-west and south-east:
    //
    //                    |
    // cast ray here  ->  L--J XXXX this region is inside the loop XXXX
    //                       |
    //
    // whereas corner pieces that go in the same direction do not cause entry into the loop:
    //
    //                    |  |
    // cast ray here  ->  L--7 XXXX this region is outside the loop XXXX

    for y in 0..grid.height() {
        let mut last_corner = None;
        let mut intersections = 0;

        for x in 0..grid.width() {
            let cell = grid.get(x, y).unwrap();
            let on_path = path.contains(&(x, y));

            if !on_path {
                if intersections % 2 == 1 {
                    enclosed_cells += 1;
                }
            } else {
                match cell {
                    FieldCell::Vertical => {
                        intersections += 1;
                    }
                    FieldCell::Horizontal => {
                        continue;
                    }
                    FieldCell::NorthEastCorner
                    | FieldCell::NorthWestCorner
                    | FieldCell::SouthWestCorner
                    | FieldCell::SouthEastCorner => {
                        if last_corner.is_none() {
                            // Track the corner as possibly entering the loop, but don't know yet.
                            last_corner = Some(cell);
                        } else if [&(last_corner.unwrap(), cell), &(cell, last_corner.unwrap())]
                            .iter()
                            .any(|(&a, &b)| OPENING_CORNER_PAIRS.contains(&(a, b)))
                        {
                            // Entered (or exited) the loop.
                            intersections += 1;
                            last_corner = None;
                        } else {
                            // Two corner pieces encountered but they do not enter the loop. Reset.
                            last_corner = None;
                        }
                    }
                    _ => panic!("invalid cell on path"),
                }
            }
        }
    }

    Some(enclosed_cells)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}
