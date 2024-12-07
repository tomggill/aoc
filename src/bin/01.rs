use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_list(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();
    for line in input.lines() {
        let row: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        list_one.push(row[0]);
        list_two.push(row[1]);
    }
    (list_one, list_two)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list_one, mut list_two) = parse_list(input);
    list_one.sort_unstable();
    list_two.sort_unstable();
    let total_distance: u32 = list_one
        .iter()
        .zip(list_two)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list_one, list_two) = parse_list(input);
    let mut count_map: HashMap<i32, u32> = HashMap::new();
    for num in list_two {
        *count_map.entry(num).or_insert(0) += 1;
    }
    let mut result = 0;
    for num in list_one {
        result += (num as u32) * count_map.get(&num).unwrap_or(&0);
    }
    Some(result)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
