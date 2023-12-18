use std::collections::HashMap;

advent_of_code::solution!(12);

// I wish I could take credit for how neat this solution is, but it was ultimately inspired by
// someone else's Go implementation of the Thompson NFA:
// https://github.com/ConcurrentCrab/AoC/blob/main/solutions/12-2.go
//
// My first iteration was a generic input -> regex -> NFA implementation, intending to build a
// Thompson NFA style iterative parallel parser. It worked, but I couldn't quite get the caching of
// past results right to avoid memory consumption growing uncontrollably. I needed to work with it.
// I really value the simplicity of the state tracking in this solution, and while simple, it still
// isn't immediately obvious until applying some thought exactly what this is doing. The key to
// recognise is that the initial state, (0, 0, false) has a visit count of 1, which bootstraps the
// whole search process when incrementing all visited states by obs in later iterations.
//
// The runtime of this is phenomenal!
fn solve<S: AsRef<str>>(springs: S, groups: &[u8]) -> u64 {
    let mut states: Option<HashMap<(usize, usize, bool), usize>> = Some(HashMap::new());
    states.as_mut().unwrap().insert((0, 0, false), 1);

    let mut new_states: Option<HashMap<(usize, usize, bool), usize>> = Some(HashMap::new());

    for ch in springs.as_ref().chars() {
        for (&(group, size, expect_working), obs) in states.as_mut().unwrap().iter() {
            let new_states = new_states.as_mut().unwrap();

            match ch {
                '#' | '?' if group < groups.len() && !expect_working => {
                    if ch == '?' && size == 0 {
                        *new_states.entry((group, size, expect_working)).or_insert(0) += obs;
                    }

                    let is_full = size + 1 == groups[group] as usize;
                    let (group, size, expect_working) = if is_full {
                        (group + 1, 0, true)
                    } else {
                        (group, size + 1, expect_working)
                    };

                    *new_states.entry((group, size, expect_working)).or_insert(0) += obs;
                }
                '.' | '?' if size == 0 => {
                    *new_states.entry((group, size, false)).or_insert(0) += obs;
                }
                _ => {}
            }
        }

        let mut old_states = states.take();
        states = new_states.take();
        old_states.as_mut().unwrap().clear();
        new_states = old_states;
    }

    states
        .as_ref()
        .unwrap()
        .iter()
        .filter(|(&(group, _, _), _)| group == groups.len())
        .map(|(_, &obs)| obs as u64)
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(" ").unwrap();

            (
                springs,
                groups
                    .split(",")
                    .map(|value| value.parse::<u8>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(springs, groups)| solve(springs, &groups))
        .sum::<u64>()
        .into()
}

fn unfold_input(line: &str) -> (String, Vec<u8>) {
    let (springs, groups) = line.split_once(" ").unwrap();

    let (springs, groups) = (
        format!(
            "{}?{}?{}?{}?{}",
            springs, springs, springs, springs, springs
        ),
        format!("{},{},{},{},{}", groups, groups, groups, groups, groups)
            .split(",")
            .map(|value| value.parse::<u8>().unwrap())
            .collect::<Vec<_>>(),
    );

    (springs.to_string(), groups)
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .lines()
        .enumerate()
        .map(|(_, line)| unfold_input(line))
        .map(|(springs, groups)| solve(springs, &groups) as u64)
        .sum::<u64>()
        .into()
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

    #[test]
    fn test_individual_part_2() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        assert_eq!(result, Some(525152));
    }
}
