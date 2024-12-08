use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let (orders, updates) = input.split_once("\n\n").unwrap();
    let mut result = 0;
    for order in orders.lines() {
        let (b, a) = order.split_once('|').unwrap();
        map.entry(b).or_default().insert(a);
    }

    for update in updates.lines() {
        let mut pages_seen: HashSet<&str> = HashSet::new();
        let mut rule_violation = false;
        for page_number in update.split(',') {
            let Some(rule) = map.get(page_number) else {
                pages_seen.insert(page_number);
                continue;
            };
            for seen in &pages_seen {
                if rule.contains(seen) {
                    rule_violation = true;
                    break;
                }
            }
            if rule_violation {
                break;
            }
            pages_seen.insert(page_number);
        }
        if !rule_violation {
            let vector: Vec<&str> = update.split(',').collect();
            let middle_index = vector.len() / 2;
            result += vector[middle_index].parse::<u32>().unwrap();
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let (orders, updates) = input.split_once("\n\n").unwrap();
    let mut result = 0;
    for order in orders.lines() {
        let (b, a) = order.split_once('|').unwrap();
        map.entry(b).or_default().insert(a);
        map.entry(a).or_default();
    }
    for update in updates.lines() {
        let mut pages_seen: HashSet<&str> = HashSet::new();
        let mut rule_violation = false;
        for page_number in update.split(',') {
            let Some(rule) = map.get(page_number) else {
                pages_seen.insert(page_number);
                continue;
            };
            for seen in &pages_seen {
                if rule.contains(seen) {
                    rule_violation = true;
                    break;
                }
            }
            if rule_violation {
                break;
            }
            pages_seen.insert(page_number);
        }
        if rule_violation {
            let sorted = update
                .split(",")
                .sorted_by(|a, b| {
                    if map[a].contains(b) {
                        return Ordering::Less;
                    }
                    Ordering::Greater
                })
                .collect_vec();

            let middle_index = (sorted.len() - 1) / 2;
            result += sorted[middle_index].parse::<u32>().unwrap();
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
