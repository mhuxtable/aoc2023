advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let orders = vec![
            self.red.partial_cmp(&other.red).unwrap(),
            self.green.partial_cmp(&other.green).unwrap(),
            self.blue.partial_cmp(&other.blue).unwrap(),
        ];

        if orders.iter().any(|order| order.is_gt()) {
            return Some(std::cmp::Ordering::Greater);
        } else if orders.iter().all(|order| order.is_eq()) {
            return Some(std::cmp::Ordering::Equal);
        } else {
            return Some(std::cmp::Ordering::Less);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    const CUBES: Round = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .lines()
        .map(|line| {
            let (game, rounds) = line.split_once(": ").unwrap();

            (
                game.strip_prefix("Game ").unwrap().parse::<u32>().unwrap(), // game ID
                rounds
                    .split("; ")
                    .map(|round| {
                        round.split(", ").fold(
                            Round {
                                red: 0,
                                green: 0,
                                blue: 0,
                            },
                            |mut counts, colour| {
                                let (count, colour) = colour.split_once(" ").unwrap();
                                let count = count.parse::<u32>().unwrap();

                                match colour {
                                    "red" => counts.red += count,
                                    "green" => counts.green += count,
                                    "blue" => counts.blue += count,
                                    _ => panic!("Unknown colour: {}", colour),
                                }

                                counts
                            },
                        )
                    })
                    .collect::<Vec<Round>>(),
            )
        })
        .filter(|(_, rounds)| rounds.iter().all(|round| round <= &CUBES))
        .map(|(game, _)| game)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let (_, rounds) = line.split_once(": ").unwrap();

            rounds
                .split("; ")
                .map(|round| {
                    round.split(", ").fold(
                        Round {
                            red: 0,
                            green: 0,
                            blue: 0,
                        },
                        |mut counts, colour| {
                            let (count, colour) = colour.split_once(" ").unwrap();
                            let count = count.parse::<u32>().unwrap();

                            match colour {
                                "red" => counts.red += count,
                                "green" => counts.green += count,
                                "blue" => counts.blue += count,
                                _ => panic!("Unknown colour: {}", colour),
                            }

                            counts
                        },
                    )
                })
                .reduce(|max, round| Round {
                    red: max.red.max(round.red),
                    green: max.green.max(round.green),
                    blue: max.blue.max(round.blue),
                })
                .unwrap()
        })
        .map(|round| round.red * round.green * round.blue)
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
