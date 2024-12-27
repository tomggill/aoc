use std::collections::HashMap;
use itertools::{iterate, Itertools};
use rayon::prelude::*;

advent_of_code::solution!(22);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, PartialOrd, Ord)]
struct ChangeSequence(i64, i64, i64, i64);

#[inline]
fn evolve(secret_number: &i64) -> i64 {
    let mut secret_number = (secret_number ^ (secret_number << 6)) % 16777216;
    secret_number = secret_number ^ (secret_number >> 5) % 16777216;
    (secret_number ^ (secret_number << 11)) % 16777216
}

pub fn part_one(input: &str) -> Option<u64> {
    let result: i64 = input
            .lines()
            .par_bridge()
            .map(|initial_secret| {
                let mut value = initial_secret.parse::<i64>().unwrap();
                for _ in 0..2000 {
                    value = evolve(&value);
                }
                value
            })
            .sum();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut change_map: HashMap<ChangeSequence, (i64, usize)> = HashMap::new();
    for (index, line) in input.lines().enumerate() {
        let initial_secret = line.parse::<i64>().unwrap();

        iterate(initial_secret, evolve)
            .take(2001)
            .map(|price| price % 10)
            .tuple_windows()
            .for_each(|(p1, p2, p3, p4, p5)| {
                let change_sequence = ChangeSequence(p2 - p1, p3 - p2, p4 - p3, p5 - p4);
                change_map
                    .entry(change_sequence)
                    .and_modify(|(sum, last_index)| {
                        if *last_index != index {
                            *sum += p5;
                            *last_index = index;
                        }
                    })
                    .or_insert((p5, index));
            });
    }
    Some(change_map.values().map(|(sum, _)| *sum).max().unwrap() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
