advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();
    for line in input.lines() {
        let row: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        list_one.push(row[0]);
        list_two.push(row[1]);
    }
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
    None
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
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
