use advent_of_code::Grid;
use itertools::Itertools;
use priority_queue::PriorityQueue;

advent_of_code::solution!(17);

/// get_losses runs a Dijkstra's algorithm (another word that's hard to type in vim with jk mapped
/// to <ESC>) to find the minimum distance from start to finish. The algorithm is modified to track
/// additional state to accommodate the wagons' movement rules: state is stored for each grid cell,
/// for each possible number of steps possibly spent accessing it, and for each possible direction
/// of access that can be used (modelled as 0 to 3 as principal ordinal directions starting north
/// and moving clockwise).
///
/// To accommodate the possibility that wagons must move a minimum distance before turning, this
/// supports additionally passing a minimum number of steps. The movement rules are given as a
/// half-open interval, i.e. [min, max) where max is the total number of steps that can be taken
/// without turning, plus one.
fn get_losses(grid: &Grid<u32>, (min, max): (usize, usize)) -> u32 {
    let mut fringe = PriorityQueue::new();

    // distance is a 1D array, where for each grid cell we have 16 states - 4 steps, with 4
    // directions each. Lookup is 16 * (y * width + x) + 4 * steps + direction (for part 1,
    // since there are max == 4 and 4 directions each - generalise to 4 * max for part 2 where an
    // arbitrary number of possible states is possible).
    let mut dist = vec![u32::MAX; (4 * max) * grid.width() * grid.height()];
    let mut prev = vec![None; (4 * max) * grid.width() * grid.height()];

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            // on each cell, we can get there by up to max steps
            (0..max)
                // and each of those steps could be taken in different directions
                .cartesian_product(0..4)
                .for_each(|(steps, direction)| {
                    fringe.push(((x, y), steps, direction), i32::MIN);
                });
        }
    }

    // we start at the top left corner, having taken 0 steps ("steps" tracks steps including the
    // current one, so initialised to 1. Note [0][1] – the 1 is direction east, the starting
    // heading)
    dist[1] = 0;
    fringe.change_priority(&((0, 0), 0, 1), 0);

    let offset =
        |(x, y), steps: usize, dir: usize| (4 * max) * (y * grid.width() + x) + steps * 4 + dir;

    while !fringe.is_empty() {
        let (current, _) = fringe.pop().unwrap();
        if current.0 == (grid.width() - 1, grid.height() - 1) {
            break;
        }

        let ((x, y), c_steps, c_dir) = current;
        let c_offset = offset((x, y), c_steps, c_dir);

        for (neigh, val) in grid.neighbours(&(x, y)) {
            let (xn, yn) = neigh;

            let n_dir = match ((xn, yn), (x, y)) {
                ((xn, yn), (x, y)) if xn == x && yn < y => 0, // north
                ((xn, yn), (x, y)) if xn > x && yn == y => 1, // east
                ((xn, yn), (x, y)) if xn == x && yn > y => 2, // south
                ((xn, yn), (x, y)) if xn < x && yn == y => 3, // west
                _ => unreachable!(),
            };

            if c_steps >= max && c_dir == n_dir {
                // neighbour in same direction after 3 steps cannot be reached
                continue;
            } else if (n_dir + 2) % 4 == c_dir {
                // wagons cannot reverse
                continue;
            } else if c_dir != n_dir {
                if c_steps < min {
                    // neighbour in different direction cannot be reached - ultra wagons have
                    // minimum straight line distance
                    continue;
                }

                // Check whether turning is possible before hitting the end of the puzzle before
                // performing the minimum number of steps in this direction.
                let turn_possible = match n_dir {
                    0 => yn >= min.saturating_sub(1),
                    1 => xn <= grid.width() - min,
                    2 => yn <= grid.height() - min,
                    3 => xn >= min.saturating_sub(1),
                    _ => unreachable!(),
                };
                if !turn_possible {
                    continue;
                }
            }

            let n_steps = if n_dir == c_dir { c_steps + 1 } else { 1 };
            let n_offset = offset((xn, yn), n_steps, n_dir);

            if fringe.get(&((xn, yn), n_steps, n_dir)).is_none() {
                continue;
            }

            let alt = dist[c_offset].saturating_add(*val);

            if alt < dist[n_offset] {
                dist[n_offset] = alt;
                fringe.change_priority(&((xn, yn), n_steps, n_dir), -(alt as i32));
                prev[n_offset] = Some(current);
            }
        }
    }

    let path = {
        let mut current = (0..max)
            .cartesian_product(0..4)
            .min_by_key(|(steps, dir)| {
                dist[offset((grid.width() - 1, grid.height() - 1), *steps, *dir)]
            })
            .map(|(steps, dir)| ((grid.width() - 1, grid.height() - 1), steps, dir))
            .unwrap();
        let mut path = vec![current.0];

        while let Some(prev) = prev[offset(current.0, current.1, current.2)] {
            path.push(prev.0);
            current = prev;
        }

        path
    };

    #[cfg(debug_assertions)]
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            print!("{}", if path.contains(&(x, y)) { "O" } else { "." });
        }
        println!();
    }

    (0..(4 * max))
        .filter_map(|off| {
            if off / 4 < min {
                None
            } else {
                Some(dist[offset((grid.width() - 1, grid.height() - 1), 0, off)])
            }
        })
        .min()
        .unwrap() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse_with_parser(input, |c| c.to_digit(10).unwrap() as u32).unwrap();
    Some(get_losses(&grid, (0, 4)))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse_with_parser(input, |c| c.to_digit(10).unwrap() as u32).unwrap();
    Some(get_losses(&grid, (4, 11)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    // This one tests that getting onto the finish position also occurs with minimum steps
    // performed after turning towards it. The naive solution to part_two wil fail this test.
    fn test_part_two_other_example() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(71));
    }
}
