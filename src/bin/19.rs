use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Clone, Debug)]
struct Part<T: Clone = u32>(T, T, T, T);

impl<T: Clone> Part<T> {
    fn score(&self, name: char) -> &T {
        match name {
            'x' => &self.0,
            'm' => &self.1,
            'a' => &self.2,
            's' => &self.3,
            _ => panic!("Invalid name: {}", name),
        }
    }

    fn score_mut(&mut self, name: char) -> &mut T {
        match name {
            'x' => &mut self.0,
            'm' => &mut self.1,
            'a' => &mut self.2,
            's' => &mut self.3,
            _ => panic!("Invalid name: {}", name),
        }
    }
}

#[derive(Debug)]
struct Workflow<'a>(Vec<(Option<(u8, std::cmp::Ordering, u32)>, &'a str)>);

impl<'a> Workflow<'a> {
    fn run_part(&self, part: &Part) -> &'a str {
        self.0
            .iter()
            .find_map(|(condition, next)| {
                if let Some((name, op, value)) = condition {
                    if op == &std::cmp::Ordering::Less {
                        if part.score(*name as char) < value {
                            return Some(next);
                        }
                    } else if part.score(*name as char) > value {
                        return Some(next);
                    }
                } else {
                    return Some(next);
                }

                None
            })
            .unwrap()
    }
}

fn parse(input: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let workflows = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (name, flow) = l.split_once("{").unwrap();

            let flows = flow
                .trim_end_matches('}')
                .split(',')
                .map(|flow| {
                    flow.split_once(":")
                        .map_or((None, flow), |(condition, next)| {
                            let (score, op, value) = condition.split_once('<').map_or_else(
                                || {
                                    let (score, value) = condition.split_once('>').unwrap();
                                    (score, std::cmp::Ordering::Greater, value)
                                },
                                |(score, value)| (score, std::cmp::Ordering::Less, value),
                            );

                            assert_eq!(score.len(), 1);

                            (
                                Some((score.bytes().next().unwrap(), op, value.parse().unwrap())),
                                next,
                            )
                        })
                })
                .collect::<Vec<_>>();

            (name, Workflow(flows))
        })
        .collect::<Vec<_>>();

    let parts = input
        .lines()
        .skip(workflows.len() + 1)
        .map(|l| {
            const SCORES: [char; 4] = ['x', 'm', 'a', 's'];

            let scores = l
                .trim_matches(|c: char| c == '{' || c == '}')
                .split(',')
                .zip(SCORES.iter())
                .map(|(score, expect)| {
                    let (score, value) = score.split_once('=').unwrap();
                    assert_eq!(score.len(), 1);
                    assert_eq!(score.bytes().next().unwrap(), *expect as u8);
                    value.parse().unwrap()
                })
                .collect::<Vec<_>>();

            Part(scores[0], scores[1], scores[2], scores[3])
        })
        .collect::<Vec<_>>();

    let mut flows = HashMap::new();
    flows.extend(workflows);

    (flows, parts)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (flows, parts) = parse(input);

    let mut queue = Vec::new();
    queue.extend((0..parts.len()).map(|i| (i, "in")));

    let mut accepted = vec![];

    loop {
        if queue.is_empty() {
            break;
        }

        queue.retain_mut(|(i, next)| {
            let part = &parts[*i];

            *next = flows.get(next).unwrap().run_part(part);

            if *next == "A" {
                accepted.push(part);
                false
            } else if *next == "R" {
                false
            } else {
                true
            }
        });
    }

    accepted
        .into_iter()
        .flat_map(|p| [p.0, p.1, p.2, p.3])
        .sum::<u32>()
        .into()
}

fn get_combos<'a>(
    // (min, max) for the score
    mut current: Part<(u32, u32)>,
    flows: &'a HashMap<&'a str, Workflow>,
    start: &'a str,
) -> u64 {
    let mut combos = 0;
    let flow = flows.get(start).unwrap();

    #[cfg(debug_assertions)]
    println!("starting at {}", start);

    for (op, next) in &flow.0 {
        #[cfg(debug_assertions)]
        println!("{}: checking {} {:?} {}", start, next, op, next == &"A");
        let mut part = current.clone();

        if let &Some((score, op, value)) = op {
            let current_score = part.score_mut(score as char);
            let next_step_score = current.score_mut(score as char);

            *current_score = if op == std::cmp::Ordering::Less {
                (current_score.0, current_score.1.min(value))
            } else {
                (current_score.0.max(value + 1), current_score.1)
            };

            *next_step_score = if op == std::cmp::Ordering::Less {
                (next_step_score.0.max(value), next_step_score.1)
            } else {
                (next_step_score.0, next_step_score.1.min(value + 1))
            };
        }

        if next == &"A" {
            #[cfg(debug_assertions)]
            println!("found a combo accepting: {:?}", part);

            combos += [part.0, part.1, part.2, part.3]
                .into_iter()
                .map(|(min, max)| max.checked_sub(min).unwrap() as u64)
                .inspect(|_c| {
                    #[cfg(debug_assertions)]
                    println!("{}: {}", start, _c);
                })
                .product::<u64>();
        } else if next == &"R" {
            continue;
        } else {
            #[cfg(debug_assertions)]
            println!("recursing into {} {:?}", next, part);
            combos += get_combos(part, flows, next);
        }
    }

    combos
}

// a{x<3:b,A}
// b{s>5:c,R}
// c{a<3:A,m>7:R,A}
//
// [1, 9]
//
// a -> A [1, 10) x > 3 (6) [4, 10) 10-4=6 check
// next: [1, 3)
//
// [1, 10) x < 3 (2) [1, 3)
// next:
//
//
// 2,4,2,9 -> A
// 2,4,9,6 -> A
// 6,9,9,9 -> A

pub fn part_two(input: &str) -> Option<u64> {
    let (flows, _) = parse(input);

    Some(get_combos(
        //Part((1, 10), (1, 10), (1, 10), (1, 10)),
        Part((1, 4001), (1, 4001), (1, 4001), (1, 4001)),
        &flows,
        "in",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
