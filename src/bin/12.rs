use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(12);

fn regex_matcher(groups: &[u8]) -> String {
    let mut regex = String::from("*");

    for (i, group) in groups.iter().enumerate() {
        assert!(*group > 0);
        regex.push_str(format!("#{}", group).as_str());

        if i < groups.len() - 1 {
            regex.push_str("+");
        }
    }

    regex.push_str("*");

    regex
}

#[derive(Debug)]
struct DAG {
    first: Node,
}

impl DAG {
    fn matches<M: AsRef<str>>(&self, input: M) -> u32 {
        let mut matches = 0;
        let mut explore_stack = vec![((Some(&self.first), 0usize), input.as_ref().to_string())];

        'stack: while let Some((start_node, input)) = explore_stack.pop() {
            #[cfg(debug_assertions)]
            println!(
                "\nnew stack item: {}: {} consumed already: {}",
                input,
                start_node.0.as_ref().unwrap(),
                start_node.1
            );

            let (mut node_opt, mut consumed) = start_node;
            let mut chars = input.chars().enumerate().peekable();

            while node_opt.is_some() {
                let node = node_opt.unwrap();
                let ch = chars.peek();

                if let Some((i, ch)) = ch {
                    #[cfg(debug_assertions)]
                    println!(
                        "current char: {:?} ({:?}), node: {}, consumed: {}",
                        ch, i, node, consumed
                    );

                    // nodes only need to be greedy. Create two new options to explore at the
                    // current node and node state (consumption) for ch being # or .
                    if *ch == '?' {
                        let options = ["#", "."].into_iter().map(|ch| {
                            ((Some(node), consumed), format!("{}{}", ch, &input[i + 1..]))
                        });

                        explore_stack.extend(options);
                        continue 'stack;
                    }

                    assert!(['#', '.'].contains(&ch), "invalid character: {:?}", ch);

                    if node.consumes(*ch) && node.can_consume_more(consumed) {
                        consumed += 1;
                        chars.next();
                    } else if node.can_advance_node(consumed) {
                        #[cfg(debug_assertions)]
                        println!("node cannot consume, but can advance");

                        node_opt = node.next();
                        consumed = 0;

                        continue;
                    } else {
                        #[cfg(debug_assertions)]
                        println!("node cannot consume or advance");

                        // characters but no more nodes, cannot match
                        continue 'stack;
                    }
                } else {
                    // no characters, so now need to bookkeep nodes
                    if node_opt.unwrap().can_advance_node(consumed) {
                        #[cfg(debug_assertions)]
                        println!("node can advance");

                        node_opt = node.next();
                        consumed = 0;
                    } else {
                        #[cfg(debug_assertions)]
                        println!("node cannot advance");

                        continue 'stack;
                    }
                }
            }

            if chars.peek().is_some() {
                #[cfg(debug_assertions)]
                println!("input not consumed");

                continue 'stack;
            }

            matches += 1;
        }

        matches
    }
}

impl std::fmt::Display for DAG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut node = Some(&self.first);

        while node.is_some() {
            writeln!(f, "{}", node.unwrap())?;
            node = node.unwrap().next();
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum Repetition {
    ZeroOrMore,
    OneOrMore,
    Exactly(u8),
}

#[derive(Debug)]
struct Node {
    idx: usize,
    consumes: char,
    repetition: Repetition,
    next: Option<Box<Node>>,
}

impl Node {
    fn consumes(&self, ch: char) -> bool {
        self.consumes == ch
    }

    fn can_consume_more(&self, count: usize) -> bool {
        match self.repetition {
            Repetition::ZeroOrMore => true,
            Repetition::OneOrMore => true,
            Repetition::Exactly(n) => count < n as usize,
        }
    }

    fn can_advance_node(&self, count: usize) -> bool {
        match self.repetition {
            Repetition::ZeroOrMore => true,
            Repetition::OneOrMore => count > 0,
            Repetition::Exactly(n) => {
                if count > n as usize {
                    panic!("consumed too many characters for node: {}/{}", count, n);
                }

                count == n as usize
            }
        }
    }

    fn next(&self) -> Option<&Node> {
        self.next.as_ref().map(|node| node.as_ref())
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(idx: {}, consumes: {}, repetition: {:?})",
            self.idx, self.consumes, self.repetition
        )
    }
}

fn dag<Regex: AsRef<str>>(regex: Regex) -> DAG {
    let regex = regex.as_ref();

    let mut nodes = vec![];
    let mut chars = regex.chars().peekable();

    while chars.peek().is_some() {
        let (matches, repetition) = match chars.next().unwrap() {
            '*' => ('.', Repetition::ZeroOrMore),
            '+' => ('.', Repetition::OneOrMore),
            '#' => {
                let digits = chars
                    .peeking_take_while(|ch| ch.is_digit(10))
                    .fold(0, |acc, ch| acc * 10 + ch.to_digit(10).unwrap());

                ('#', Repetition::Exactly(digits as u8))
            }
            ch => panic!("invalid regex token: {}", ch),
        };

        let node = Node {
            idx: nodes.len(),
            consumes: matches,
            repetition,
            next: None,
        };

        nodes.push(node);
    }

    let mut first_node = nodes.remove(0);
    let mut last_node = &mut first_node;

    while !nodes.is_empty() {
        let next_node = nodes.remove(0);
        last_node.next = Some(Box::new(next_node));

        last_node = last_node.next.as_mut().unwrap();
    }

    DAG { first: first_node }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(" ").unwrap();

            let groups = groups
                .split(",")
                .map(|value| value.parse::<u8>().unwrap())
                .collect::<Vec<_>>();

            let dag = dag(regex_matcher(&groups));
            dag.matches(springs)
        })
        .sum::<u32>()
        .into()
}

fn unfold_input(line: &str) -> (String, Vec<u8>) {
    let (springs, groups) = line.split_once(" ").unwrap();

    let groups = groups
        .split(",")
        .map(|value| value.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let springs = std::iter::repeat(springs).take(5).join("?");
    let groups = std::iter::repeat(groups)
        .take(5)
        .flatten()
        .collect::<Vec<_>>();

    (springs.to_string(), groups)
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .enumerate()
        .map(|(_, line)| {
            let (springs, groups) = unfold_input(line);

            let dag = dag(regex_matcher(&groups));
            dag.matches(springs)
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dag_matcher() {
        let inputs = &advent_of_code::template::read_file("examples", DAY);
        let expected = [1, 4, 1, 1, 4, 10];

        for (i, (input, expected)) in inputs.lines().zip(expected.iter()).enumerate() {
            let (springs, groups) = input.split_once(" ").unwrap();

            let groups = groups
                .split(",")
                .map(|value| value.parse::<u8>().unwrap())
                .collect::<Vec<_>>();

            let dag = dag(regex_matcher(&groups));
            let matches = dag.matches(springs);

            assert_eq!(
                matches, *expected,
                "failed on example {}: expected {}, got {}",
                i, expected, matches
            );
        }
    }

    #[test]
    fn test_dag_matcher_unfolded() {
        let inputs = &advent_of_code::template::read_file("examples", DAY);
        let inputs = inputs.lines().map(unfold_input);

        let expected = [1, 16384, 1, 16, 2500, 506250];

        for (i, ((springs, groups), expected)) in inputs.zip(expected.iter()).enumerate() {
            let dag = dag(regex_matcher(&groups));
            let matches = dag.matches(springs);

            assert_eq!(
                matches, *expected,
                "failed on example {}: expected {}, got {}",
                i, expected, matches
            );
        }
    }

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
