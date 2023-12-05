use std::collections::BinaryHeap;

advent_of_code::solution!(4);

fn parse(input: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (winning, candidates) = numbers.split_once(" | ").unwrap();

            (
                winning
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
                candidates
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn count_winning_numbers(cards: &(Vec<u8>, Vec<u8>)) -> usize {
    let (winning, candidates) = cards;
    candidates.iter().filter(|c| winning.contains(c)).count()
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse(input);

    cards
        .iter()
        .map(count_winning_numbers)
        .map(|d| match d {
            0 => 0,
            d => 2u32.pow(d as u32 - 1),
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse(input);
    // BinaryHeap is a max-heap, so to get the desired order we have to insert card indices as
    // negative
    let mut instances = BinaryHeap::from_iter((0..cards.len()).map(|n| -(n as isize)));

    cards
        .iter()
        .map(count_winning_numbers)
        .enumerate()
        .map(|(i, d)| {
            let count = {
                let mut count = 0;

                loop {
                    if let Some(card) = instances.pop() {
                        if card.abs() as usize != i {
                            instances.push(card);
                            break;
                        } else {
                            count += 1;
                        }
                    } else {
                        break;
                    }
                }

                count
            };

            // We have count winning cards, which means we win (i+1..i+1+d) additional cards.
            instances.extend(
                // doesn't need a min(cards.len()) check, instructions say we'll never duplicate
                // off the end of the table
                ((i + 1)..(i + 1 + d))
                    .map(|n| -(n as isize))
                    .flat_map(|n| std::iter::repeat(n).take(count)),
            );

            count as u32
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
