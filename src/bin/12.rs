use regex::Regex;

advent_of_code::solution!(12);

fn possible_placements<S: AsRef<str>>(springs: S) -> Vec<String> {
    let mut placements: Vec<Vec<u8>> = vec![springs.as_ref().as_bytes().to_vec()];

    for i in springs
        .as_ref()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '?')
        .map(|(i, _)| i)
    {
        let mut more_placements = vec![];

        for placement in &mut placements {
            let mut spring = placement.clone();
            spring[i] = '.' as u8;
            more_placements.push(spring);

            placement[i] = '#' as u8;
        }

        placements.extend(more_placements);
    }

    placements
        .iter()
        .map(|p| std::str::from_utf8(p).unwrap().to_owned())
        .collect::<Vec<String>>()
}

fn regex_matcher(groups: &Vec<u8>) -> String {
    let mut regex = String::from("^\\.*");

    for (i, group) in groups.iter().enumerate() {
        assert!(*group > 0);
        regex.push_str(format!("#{{{}}}", group).as_str());

        if i < groups.len() - 1 {
            regex.push_str("\\.+");
        }
    }

    regex.push_str("\\.*$");

    regex
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(" ").unwrap();

            let possible_placements = possible_placements(springs);
            let groups = groups
                .split(",")
                .map(|g| g.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();

            (possible_placements, regex_matcher(&groups))
        })
        .enumerate()
        .inspect(|(i, _)| {
            println!("Solving {}/{}", i + 1, input.lines().count());
        })
        .map(|(_, (placements, regex))| {
            placements
                .iter()
                .filter(|placement| Regex::new(regex.as_str()).unwrap().is_match(placement))
                .count() as u32
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
