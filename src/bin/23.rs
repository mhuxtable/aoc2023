use std::collections::{HashMap, HashSet, VecDeque};

/// I found the logic for computing part one very confusing, as the "icy slopes" are not included
/// in the overall count of steps, but the problem description is a bit unclear about this. It
/// states that if we step onto an icy slope, our next _step_ must be in the direction of the
/// arrow, whereas what it really means is we _slide_ down the slope such that we don't actually
/// "take a step". i.e. we move in the direction of the arrow at cost of zero and this move does
/// not contribute to our maximum path length. The puzzle description should have been clearer that
/// such moves are not steps.
///
/// Anyway, once identified on a hunch, it was easy to compute the longest path including the
/// slides and then simply exclude the slides when doing the cost calculation. I have adjusted the
/// logic to instead exclude the steps onto slides when computing the longest path. My puzzle input
/// gave the same answer whether I computered longest path and then set slide cost to 0, or if I
/// set slide cost to 0 while computing the longest path. In general, the latter technique provides
/// the correct answer because a long sequence of consecutive slides could convince the former
/// algorithm that it is the longest path, while there is actually a longer path with more actual
/// steps. The puzzle input does not seem to test this.
use advent_of_code::Grid;

advent_of_code::solution!(23);

#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell {
    Path,
    Forest,
    Slope(Direction),
}
use Cell::*;

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Path => write!(f, "."),
            Forest => write!(f, "#"),
            Slope(dir) => match dir {
                North => write!(f, "^"),
                East => write!(f, ">"),
                South => write!(f, "v"),
                West => write!(f, "<"),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use priority_queue::PriorityQueue;
use Direction::*;

impl Direction {
    fn between((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Self {
        if x1 == x2 && y1 == y2 {
            panic!("cannot get direction between same point");
        } else if x1 == x2 {
            if y1 > y2 {
                North
            } else {
                South
            }
        } else if y1 == y2 {
            if x1 < x2 {
                East
            } else {
                West
            }
        } else {
            panic!("cannot get direction between non-adjacent points");
        }
    }

    fn opposite(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    fn next(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            North => (x, y.checked_sub(1).unwrap()),
            East => (x + 1, y),
            South => (x, y + 1),
            West => (x.checked_sub(1).unwrap(), y),
        }
    }
}

fn count_visited(grid: &Grid<Cell>, visited: &[(usize, usize)]) -> usize {
    visited
        .iter()
        .filter(|(x, y)| grid.get(*x, *y) == Some(&Path))
        .count()
}

fn neighbours(grid: &Grid<Cell>, (x, y): (usize, usize)) -> (Vec<(usize, usize)>, bool) {
    let neighbours = grid
        .neighbours(&(x, y))
        .into_iter()
        .filter(|(_, &val)| val != Forest)
        .map(|((xn, yn), _)| (xn, yn))
        .collect::<Vec<_>>();

    let is_junction = neighbours.len() > 2;

    (neighbours, is_junction)
}

// the paths through the grid have relatively few junctions, so dfs is fine for part 1, but blows
// up for part 2
fn solve(input: &str, enable_slopes: bool) -> Option<u32> {
    let grid: Grid<Cell> = Grid::parse_with_parser(Path, input, |c| match c {
        '.' => Path,
        '#' => Forest,
        '^' | '>' | 'v' | '<' if !enable_slopes => Path,
        '^' => Slope(North),
        '>' => Slope(East),
        'v' => Slope(South),
        '<' => Slope(West),
        _ => panic!("invalid cell character"),
    })
    .unwrap();

    println!("{}", grid);

    let mut q = VecDeque::new();

    q.push_front(((1, 0), vec![]));
    let mut longest = 0;
    let mut longest_visits = None;

    let mut junction_costs = HashMap::new();

    while let Some(((x, y), mut visited)) = q.pop_front() {
        visited.push((x, y));

        if (x, y) == (grid.width() - 2, grid.height() - 1) {
            let try_visited = count_visited(&grid, &visited);
            if try_visited > longest {
                longest = try_visited;
                longest_visits = Some(visited.clone());
            }

            continue;
        }

        if let Some(Slope(slope_dir)) = grid.get(x, y) {
            let (xn, yn) = slope_dir.next((x, y));
            if visited.contains(&(xn, yn)) {
                continue;
            }

            visited.push((xn, yn));
            q.push_front(((xn, yn), visited));
            continue;
        }

        let (neighbours, is_junction) = neighbours(&grid, (x, y));

        for (xn, yn) in neighbours {
            if visited.contains(&(xn, yn)) {
                continue;
            }

            if is_junction {
                let cost = junction_costs.entry((xn, yn)).or_insert(0);

                if *cost > visited.len() {
                    continue;
                } else {
                    *cost = visited.len();
                }
            }

            let visited = visited.clone();
            let dir = Direction::between((x, y), (xn, yn));

            if let Some(Slope(slope_dir)) = grid.get(xn, yn) {
                if *slope_dir == dir.opposite() {
                    continue;
                }
            }

            q.push_back(((xn, yn), visited));
        }
    }

    println!(
        "{}",
        grid.fmt_with_overrides(|cell| {
            longest_visits
                .as_ref()
                .is_some_and(|longest_visits| longest_visits.contains(cell))
                .then(|| 'O')
        })
    );

    println!("{}", longest_visits.as_ref().unwrap().len());

    Some(longest as u32 - 1)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, true)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }

    #[test]
    fn test_between() {
        assert_eq!(Direction::between((0, 0), (0, 1)), South);
        assert_eq!(Direction::between((0, 0), (1, 0)), East);
        assert_eq!(Direction::between((1, 1), (1, 0)), North);
        assert_eq!(Direction::between((1, 1), (0, 1)), West);
    }
}
