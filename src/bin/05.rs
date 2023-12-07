use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Clone, Debug)]
struct Range {
    source_start: u64,
    length: u64,
    destination_start: u64,
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
}

impl LookupRange for Vec<Range> {
    fn find_for_domain(&self, source: u64) -> Option<&Range> {
        self.iter().find(|range| {
            range.source_start <= source && source < range.source_start + range.length
        })
    }

    // mapped_value tries to find a range that will map the provided source to an output, otherwise
    // it maps the value using the identity function, returning itself.
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
        .map(|seed| maps.iter().fold(*seed, |acc, map| map.mapped_value(acc)))
        .min()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = parse(input);
    Some(solve(seeds, maps))
}

// The idea in part 2 is to prune the search space of input seeds to only those seeds that could
// possibly provide a minimum location value. This initially felt intuitive to me, but initially I
// tried to do this via some sort of meta method by splitting the ranges in each mapping itself
// whereever a map had a range boundary that overlapped with a range in the next map. This was more
// complex than desired, until I realised this would be far simpler if I simply considered possible
// candidate values and inverted these in reverse order. Attempting to brute force based on the
// initial set of seeds will simply take too long to solve.
//
// To prune the search space, we consider that each mapping is a piecewise linear function over the
// domain of integers, formed by the re-mapped ranges given in the problem input, and by the
// identity function for other values that are not re-mapped. I assume that the function for each
// map is a bijection; the identity function is trivially bijective, and I assume based on context
// and puzzle description that the re-mapped ranges follow this definition. (It is given that the
// combination of ranges and identity function is an injective function on the integers, by
// definition. I am assuming that the re-mappinged ranges are surjective, i.e. that multiple
// distinct input values will never be remapped to the same output values). With this assumption,
// each mapping function is invertible, allowing us to reverse map output values onto their input
// values.
//
// Intuitively, we can build a list of candidate seeds by looking at the output values of the
// location mapping function and considering the boundaries of discontinuities, i.e. the start of
// each range (including the ranges defined by the identity function).
//
// Using this candidate set, we can then invert the map to obtain the input values that map to
// those output values. This process can be repeated, using this new candidate as the output from
// the previous mapping function, being sure at each step to also include the boundaries of any
// ranges that may not have been included in the inverted output values.
//
// Eventually, this produces a set of values that represent possible seed candidates. We can
// intersect this with the provided seed ranges to prune candidates that are not possible, as no
// input seeds are provided. The remaining seeds are tested as per part_one until a minimum is
// found.
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

    fn range_to_endpoints(range: &Range) -> (u64, u64) {
        (
            range.source_start,
            range.source_start.saturating_add(range.length),
        )
    }

    let seed_candidates = maps.iter().rev().fold(candidates, |codomain, map| {
        let mut domain = HashSet::new();

        // Invert the values from the previous map's domain, treating them as the codomain of
        // this map.
        domain.extend(codomain.iter().map(|value| {
            let range = map.find_for_image(*value).unwrap();
            let mapped_value = range.source_start + (*value - range.destination_start);
            mapped_value
        }));

        for (start, end) in map.iter().map(range_to_endpoints) {
            // Include the end of the range, because this represents the start of an implicit range
            // mapped either by another range (whose start will be included in the possible minimum
            // values set in the next iteration), or an implicit identity function mapped range.
            domain.extend(vec![start, end]);
        }

        domain
    });

    Some(solve(
        seed_candidates
            .iter()
            // Remove seed candidates outside the eligible ranges, as these cannot produce a
            // minimum location value.
            .filter(|&&seed| seed_ranges.find_for_domain(seed).is_some())
            .cloned()
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
