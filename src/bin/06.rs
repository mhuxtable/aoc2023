advent_of_code::solution!(6);

fn solve(input: &str, strip_spaces: bool) -> u32 {
    let races = {
        let mut lines = input.lines();

        let mut parse = move || -> Vec<u64> {
            lines
                .next()
                .unwrap()
                .trim()
                .split_once(":")
                .unwrap()
                .1
                .chars()
                .filter(|c| !strip_spaces || *c != ' ')
                .chain(std::iter::once(' '))
                .fold((vec![], 0u64), |(mut ns, n), c| {
                    if let Some(digit) = c.to_digit(10) {
                        (ns, n * 10 + digit as u64)
                    } else if c == ' ' {
                        if n != 0 {
                            ns.push(n);
                        }
                        (ns, 0)
                    } else {
                        panic!("unexpected character: {}", c);
                    }
                })
                .0
        };

        let times = parse();
        let distances = parse();

        times
            .into_iter()
            .zip(distances.into_iter())
            .collect::<Vec<_>>()
    };

    let race_margins = races
        .iter()
        .map(|&(t, s)| {
            let disc = ((t.pow(2) - 4u64.checked_mul(s).unwrap()) as f64).sqrt();
            let (lower, upper) = ((t as f64 - disc) / 2.0, (t as f64 + disc) / 2.0);

            (upper.ceil() as u32)
                .checked_sub((lower + 1.0).floor() as u32)
                .unwrap()
        })
        .collect::<Vec<_>>();

    race_margins.iter().product()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
