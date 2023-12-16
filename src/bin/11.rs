use advent_of_code::Grid;
use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Galaxy,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Empty
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("unknown cell type"),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Galaxy => write!(f, "#"),
        }
    }
}

trait GridExt {
    fn empty_rows(&self) -> Vec<usize>;
    fn empty_cols(&self) -> Vec<usize>;
}

impl<T> GridExt for Grid<T>
where
    T: Clone + PartialEq + Default,
{
    fn empty_rows(&self) -> Vec<usize> {
        (0..self.height())
            .filter(|y| (0..self.width()).all(|x| self.get(x, *y).unwrap() == &T::default()))
            .collect::<Vec<_>>()
    }

    fn empty_cols(&self) -> Vec<usize> {
        (0..self.width())
            .filter(|x| (0..self.height()).all(|y| self.get(*x, y).unwrap() == &T::default()))
            .collect::<Vec<_>>()
    }
}

trait GalaxyExt {
    fn galaxies(&self) -> Vec<(usize, usize)>;
}

impl GalaxyExt for Grid<Cell> {
    fn galaxies(&self) -> Vec<(usize, usize)> {
        self.iter()
            .filter_map(|(point, cell)| {
                if cell == Cell::Galaxy {
                    Some(point)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}

fn galaxy_pairwise_distances<CoordMapFn: Fn(&(usize, usize)) -> (usize, usize)>(
    galaxies: &[(usize, usize)],
    mapper: CoordMapFn,
) -> Vec<u64> {
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let (x1, y1) = mapper(a);
            let (x2, y2) = mapper(b);

            #[cfg(debug_assertions)]
            {
                println!("originally: ({}, {}) -> ({}, {})", a.0, a.1, b.0, b.1);
                println!("mapped: ({}, {}) -> ({}, {})", x1, y1, x2, y2);
            }

            ((x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs()) as u64
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    // 2 because we double the size of all the space. This distinction is important in part 2 where
    // it is a replacement by 1_000_000 times, not adding 1m rows/cols.
    Some(compute_with_expansion(input, 2) as u32)
}

// big numbers: this one overflows a u32
fn part_two(input: &str) -> Option<u64> {
    Some(compute_with_expansion(input, 1_000_000))
}

fn compute_with_expansion(input: &str, expansion_size: usize) -> u64 {
    let grid: Grid<Cell> = Grid::parse(input).unwrap();

    let empty_rows = grid.empty_rows();
    let empty_cols = grid.empty_cols();

    let galaxies = grid.galaxies();

    galaxy_pairwise_distances(&galaxies, |&(x, y)| {
        let empty_cols = empty_cols.iter().filter(|&&c| c < x).count();
        let empty_rows = empty_rows.iter().filter(|&&r| r < y).count();

        (
            x + (empty_cols * (expansion_size - 1)) as usize,
            y + (empty_rows * (expansion_size - 1)) as usize,
        )
    })
    .iter()
    .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        [(10, 1030), (100, 8410)]
            .iter()
            .for_each(|(expansion_size, expected)| {
                let result = compute_with_expansion(
                    &advent_of_code::template::read_file("examples", DAY),
                    *expansion_size,
                );
                assert_eq!(result, *expected, "expansion size {}", expansion_size);
            });
    }
}
