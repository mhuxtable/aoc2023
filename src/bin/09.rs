advent_of_code::solution!(9);

fn solve(input: &str) -> Vec<Vec<(Vec<i32>, bool)>> {
    let sequences = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn is_zero(seq: &[i32]) -> bool {
        seq.iter().all(|&n| n == 0)
    }

    sequences
        .iter()
        .map(|seq| {
            let mut diff_seqs: Vec<(Vec<i32>, bool)> = vec![(seq.clone(), is_zero(seq))];

            while !diff_seqs.last().unwrap().1 {
                let last_seq = diff_seqs.last().unwrap();

                let diff_seq = last_seq
                    .0
                    .iter()
                    .zip(last_seq.0.iter().skip(1))
                    .map(|(a, b)| b - a)
                    .collect::<Vec<_>>();

                let zero = is_zero(&diff_seq);
                diff_seqs.push((diff_seq, zero));
            }

            println!(
                "{}",
                diff_seqs
                    .iter()
                    .map(|(seq, _)| seq
                        .iter()
                        .map(|d| format!("{}", d))
                        .collect::<Vec<_>>()
                        .join(" "))
                    .collect::<Vec<_>>()
                    .join("\n")
            );

            diff_seqs
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<i32> {
    let diffs = solve(input);

    diffs
        .iter()
        .map(|diff_seqs| {
            let next = diff_seqs.iter().rev().skip(1).fold(0, |acc, (seq, _)| {
                let last_diff = seq.last().unwrap();
                let this_diff = acc + last_diff;

                this_diff
            });

            println!("next: {}\n", next);

            next
        })
        .sum::<i32>()
        .into()
}

pub fn part_two(input: &str) -> Option<i32> {
    let diffs = solve(input);

    diffs
        .iter()
        .map(|diff_seqs| {
            let zeroth = diff_seqs.iter().rev().skip(1).fold(0, |acc, (seq, _)| {
                let last_diff = seq.first().unwrap();
                let this_diff = last_diff.checked_sub(acc).unwrap();

                this_diff
            });

            println!("0th: {}\n", zeroth);

            zeroth
        })
        .sum::<i32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
