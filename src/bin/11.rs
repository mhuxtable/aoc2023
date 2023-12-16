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

pub fn part_one(input: &str) -> Option<u32> {
    // for part 1, we just expand rows and cols in situ, using memory for efficiency

    let grid = {
        // grid is a temporary, before we expand it into new_grid
        let grid: Grid<Cell> = Grid::parse(input).unwrap();

        let empty_rows = (0..grid.height())
            .filter(|y| (0..grid.width()).all(|x| grid.get(x, *y).unwrap() == &Cell::Empty))
            .collect::<Vec<_>>();

        let empty_cols = (0..grid.width())
            .filter(|x| (0..grid.height()).all(|y| grid.get(*x, y).unwrap() == &Cell::Empty))
            .collect::<Vec<_>>();

        let mut new_grid = Grid::new(
            Cell::Empty,
            grid.width() + empty_cols.len(),
            grid.height() + empty_rows.len(),
        );

        let (mut x_offset, mut y_offset) = (0, 0);

        for y in 0..grid.height() {
            if empty_rows.contains(&y) {
                y_offset += 1;
                continue;
            }

            for x in 0..grid.width() {
                if empty_cols.contains(&x) {
                    x_offset += 1;
                    continue;
                }

                new_grid.set(x + x_offset, y + y_offset, *grid.get(x, y).unwrap());
            }

            x_offset = 0;
        }

        new_grid
    };

    let galaxies = grid
        .iter()
        .filter_map(|(point, cell)| {
            if cell == Cell::Galaxy {
                Some(point)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // compute the manhattan distance for each galaxy pair. It shows this as diagonal in the
    // exercise but manhattan is simpler and equivalent
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let (x1, y1) = a;
            let (x2, y2) = b;

            ((*x1 as i32 - *x2 as i32).abs() + (*y1 as i32 - *y2 as i32).abs()) as u32
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    // for part two, for each row/column we need to expand, we just maintain a mapping of the
    // original input coordinates to the expanded galaxy coordinates, to avoid excessive memory
    // use.
    let grid: Grid<Cell> = Grid::parse(input).unwrap();
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
