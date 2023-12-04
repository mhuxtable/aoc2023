advent_of_code::solution!(1);

fn calibration_value<T: Iterator<Item = u32>>(mut digits: T) -> u32 {
    let first = digits.next().unwrap();
    let last = if let Some(last) = digits.last() {
        last
    } else {
        first
    };

    first as u32 * 10 + last as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)))
        .map(calibration_value)
        .sum::<u32>()
        .into()
}

const DIGITS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let mut digits = vec![];
            let mut history = vec![];

            for c in line.to_lowercase().chars() {
                if let Some(digit) = c.to_digit(10) {
                    digits.push(digit);
                    history.clear();
                } else {
                    history.push(c);

                    let word: String = history.iter().collect();
                    for (i, digit) in DIGITS.iter().enumerate() {
                        if word.ends_with(digit) {
                            digits.push((i + 1) as u32);
                            history.drain(..history.len() - 1);
                            break;
                        }
                    }
                }
            }

            digits.into_iter()
        })
        .map(calibration_value)
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
