use std::collections::HashMap;

advent_of_code::solution!(8);

fn parse(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap();
    lines.next();

    let nodes = lines
        .map(|line| {
            let (node, next_nodes) = line.split_once(" = ").unwrap();
            let (left, right) = next_nodes
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .unwrap();

            (node, (left, right))
        })
        .collect::<Vec<_>>();

    let nodes: HashMap<&str, (&str, &str)> = HashMap::from_iter(nodes);

    (instructions, nodes)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, nodes) = parse(input);

    let mut current = "AAA";
    let mut count = 0;

    for instruction in instructions.chars().cycle() {
        let (left, right) = nodes.get(&current).unwrap();

        if current == "ZZZ" {
            break;
        }

        count += 1;

        match instruction {
            'L' => current = left,
            'R' => current = right,
            _ => panic!("Unknown instruction"),
        };
    }

    Some(count)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, nodes) = parse(input);

    let start_nodes = nodes
        .keys()
        .filter(|node| node.ends_with("A"))
        .collect::<Vec<_>>();

    let cycle_length = start_nodes
        .iter()
        .map(|&start_node| {
            let mut current = start_node;
            let mut count = 0u64;
            let mut instruction_idx = 0;

            // (node, instruction_index) -> step count
            let mut seen = HashMap::new();

            loop {
                let instruction = instructions.chars().nth(instruction_idx).unwrap();

                let (left, right) = nodes.get(current).unwrap();

                match instruction {
                    'L' => current = left,
                    'R' => current = right,
                    _ => panic!("Unknown instruction"),
                };

                count += 1;

                if let Some(&last_seen) = seen.get(&(current, instruction_idx)) {
                    if !current.ends_with("Z") {
                        return count - last_seen;
                    }
                } else {
                    seen.insert((current, instruction_idx), count);
                }

                instruction_idx = (instruction_idx + 1) % instructions.len();
            }
        })
        .collect::<Vec<_>>();

    #[cfg(debug_assertions)]
    println!("cycle lengths: {:?}", cycle_length);

    Some(
        cycle_length
            .iter()
            .zip(cycle_length.iter().skip(1))
            .fold(1, |acc, (&a, &b)| lcm(acc, lcm(a, b))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(1, 3), 1);
        assert_eq!(gcd(10, 4), 2);
        assert_eq!(gcd(7, 5), 1);
        assert_eq!(gcd(100, 25), 25);
    }
}
