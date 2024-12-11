use std::collections::HashMap;

advent_of_code::solution!(11);

fn count_stones(stone: u64, iteration: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if iteration == 0 {
        return 1;
    }
    if let Some(&cached_result) = cache.get(&(stone, iteration)) {
        return cached_result;
    }

    let result = if stone == 0 {
        count_stones(1, iteration - 1, cache)
    } else {
        let stone_string = stone.to_string();
        if stone_string.len() % 2 == 0 {
            let middle_index = stone_string.len() / 2;
            let (left_stone, right_stone) = stone_string.split_at(middle_index);
            count_stones(left_stone.parse::<u64>().unwrap(), iteration - 1, cache)
                + count_stones(right_stone.parse::<u64>().unwrap(), iteration - 1, cache)
        } else {
            count_stones(stone * 2024, iteration - 1, cache)
        }
    };
    cache.insert((stone, iteration), result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    let result: u64 = stones
        .iter()
        .map(|&stone| count_stones(stone, 25, &mut cache))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    let result: u64 = stones
        .iter()
        .map(|&stone| count_stones(stone, 75, &mut cache))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
