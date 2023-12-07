use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Clone, Debug)]
struct Range {
    source_start: u64,
    length: u64,
    destination_start: u64,
}

impl Range {
    fn try_map_value(&self, source: u64) -> Option<u64> {
        if self.source_start <= source && source < self.source_start + self.length {
            let offset = source - self.source_start;
            Some(self.destination_start + offset)
        } else {
            None
        }
    }

    fn try_map_from_result(&self, destination: u64) -> Option<u64> {
        if self.destination_start <= destination
            && destination < self.destination_start + self.length
        {
            let offset = destination - self.destination_start;
            Some(self.source_start + offset)
        } else {
            None
        }
    }
}

fn map_range(input: &str) -> Range {
    let parts = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    assert!(parts.len() == 3);

    Range {
        source_start: parts[1],
        length: parts[2],
        destination_start: parts[0],
    }
}

// LookupRange provides a way to identify the range, and optionally complete a mapping, for a value
// from the domain of the range functions to the mapped value in the codomain. It assumes that
// ranges are non-overlapping and that the function defined by the ranges is injective (which is
// the case).
trait LookupRange {
    fn find_for_domain(&self, source: u64) -> Option<&Range>;
    fn mapped_value(&self, source: u64) -> u64;
}

// ReverseLookupRange provides a way to map from a value in the codomain to the applicable range in
// the domain of the function defined by the ranges. It assumes that ranges are non-overlapping and
// form a surjective function (which, allowing for the possibility of ranges not explicitly defined
// to map using the identity function, is indeed the case).
trait ReverseLookupRange {
    fn find_for_image(&self, destination: u64) -> Option<Range>;
    fn reverse_mapped_value(&self, destination: u64) -> u64;
}

impl LookupRange for Vec<Range> {
    fn find_for_domain(&self, source: u64) -> Option<&Range> {
        self.iter().find(|range| {
            range.source_start <= source && source < range.source_start + range.length
        })
    }

    fn mapped_value(&self, source: u64) -> u64 {
        if let Some(range) = self.find_for_domain(source) {
            let offset = source - range.source_start;
            range.destination_start + offset
        } else {
            source
        }
    }
}

impl ReverseLookupRange for Vec<Range> {
    fn find_for_image(&self, destination: u64) -> Option<Range> {
        self.iter()
            .find(|range| {
                range.destination_start <= destination
                    && destination < range.destination_start + range.length
            })
            .map(|item| item.clone())
            .or_else(|| {
                // There is no range, so find the boundaries of the range that is mapped by the
                // identity function and synthesise a range instead.
                let (last_end, next_begin) =
                    self.iter()
                        .fold((0u64, u64::MAX), |(last_end, next_begin), range| {
                            let range_end = range.destination_start + range.length;

                            if range_end <= destination
                                && range_end.abs_diff(destination) < last_end.abs_diff(destination)
                            {
                                (range_end, next_begin)
                            } else if range.destination_start > destination
                                && range.destination_start.abs_diff(destination)
                                    < next_begin.abs_diff(destination)
                            {
                                (last_end, range.destination_start)
                            } else {
                                (last_end, next_begin)
                            }
                        });

                Some(Range {
                    source_start: last_end,
                    length: next_begin - last_end,
                    destination_start: last_end,
                })
            })
    }

    fn reverse_mapped_value(&self, destination: u64) -> u64 {
        if let Some(range) = self.find_for_image(destination) {
            let offset = destination - range.destination_start;
            range.source_start + offset
        } else {
            destination
        }
    }
}

fn parse(input: &str) -> (Vec<u64>, Vec<Vec<Range>>) {
    let mut lines = input.lines();

    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut maps = vec![];

    while let Some(line) = lines.next() {
        if line.is_empty() {
            maps.push(vec![]);
        } else if line.ends_with(" map:") {
            continue;
        } else {
            maps.last_mut().unwrap().push(map_range(line));
        };
    }

    maps.iter_mut()
        .for_each(|map| map.sort_by_key(|range| range.destination_start));

    (seeds, maps)
}

fn solve(seeds: Vec<u64>, maps: Vec<Vec<Range>>) -> u64 {
    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |acc, map| {
                let mapped = map.mapped_value(acc);
                mapped
            })
        })
        .min()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = parse(input);
    Some(solve(seeds, maps))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = parse(input);

    // seed ranges are just special cases where source maps to destination, but we model them here
    // as ranges to allow a generic reverse search algorithm from location backwards to be used.
    let mut seed_ranges = seeds
        .iter()
        .tuples()
        .map(|(a, b)| Range {
            source_start: *a,
            length: *b,
            destination_start: *a,
        })
        .collect::<Vec<Range>>();
    seed_ranges.sort_by_key(|range| range.source_start);

    let mut candidates = HashSet::new();
    candidates.insert(0);
    candidates.insert(u64::MAX);

    fn range_to_endpoints(range: &Range) -> (u64, u64) {
        (range.source_start, range.source_start + range.length - 1)
    }

    let seed_candidates = maps.iter().rev().fold(candidates, |codomain, map| {
        {
            let mut p = codomain.iter().collect::<Vec<_>>();
            p.sort();
        }

        let mut domain = HashSet::new();
        domain.insert(0);
        domain.insert(u64::MAX);

        // Invert the values from the previous map's domain, treating them as the codomain of
        // this map.
        domain.extend(
            codomain
                .iter()
                .filter(|&&v| v > 0 && v < u64::MAX)
                .flat_map(|value| {
                    let range = map.find_for_image(*value).unwrap();
                    let (start, end) = range_to_endpoints(&range);
                    let mapped_value = range.try_map_from_result(*value).unwrap();

                    vec![start, end, mapped_value, start.saturating_sub(1), end + 1]
                }),
        );

        for (start, end) in map.iter().map(range_to_endpoints) {
            domain.insert(start);
            domain.insert(end);
            domain.insert(start.saturating_sub(1));
            domain.insert(end.saturating_add(1));
        }

        domain.extend(codomain);
        domain.remove(&(u64::MAX - 1));

        domain
    });

    let mut s = seed_candidates.iter().collect::<Vec<_>>();
    s.sort();

    let mut possible_seeds = HashSet::new();
    possible_seeds.extend(s.iter().filter_map(|&seed| {
        seed_ranges
            .find_for_domain(*seed)
            .map(|range| range.try_map_value(*seed).unwrap())
    }));

    Some(solve(
        possible_seeds
            .into_iter()
            .filter(|&seed| seed > 0)
            .collect::<Vec<_>>(),
        maps,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
