advent_of_code::solution!(15);

fn hasher<S: AsRef<str>>(s: S) -> u32 {
    s.as_ref()
        .chars()
        .fold(0, |acc, c| (17 * (acc + c as u32)) % 256)
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(hasher)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let operations = input.lines().next().unwrap().split(',').collect::<Vec<_>>();

    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];

    for seq in operations {
        let (label, rest) = seq.split_at(seq.len() - if seq.contains("-") { 1 } else { 2 });
        let op = rest.chars().next().unwrap();

        assert!(
            (rest.len() == 1 && op == '-') || (rest.len() == 2 && op == '='),
            "{}: {} {} {}",
            seq,
            label,
            op,
            rest
        );
        let focal_length = if op == '=' {
            Some(rest[1..].parse::<u32>().unwrap())
        } else {
            None
        };

        let lensbox = &mut boxes[hasher(label) as usize];

        let existing_lens_idx = lensbox.iter().position(|(lens, _)| *lens == label);

        match op {
            '-' => {
                if existing_lens_idx.is_some() {
                    lensbox.remove(existing_lens_idx.unwrap());
                }
            }
            '=' => {
                if existing_lens_idx.is_some() {
                    lensbox[existing_lens_idx.unwrap()].1 = focal_length.unwrap();
                } else {
                    lensbox.push((label, focal_length.unwrap()));
                }
            }
            _ => unreachable!(),
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(j, (_, focal_length))| (1 + i as u32) * (1 + j as u32) * focal_length)
                .sum::<u32>()
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hasher() {
        assert_eq!(hasher("HASH"), 52);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
