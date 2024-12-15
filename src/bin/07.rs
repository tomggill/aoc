advent_of_code::solution!(7);

fn apply_operations(
    current_value: u64,
    numbers: &Vec<u64>,
    index: usize,
    is_part_one: bool,
) -> bool {
    if index == 0 {
        return numbers[0] == current_value;
    }
    let next_value = numbers[index];
    if current_value % next_value == 0
        && apply_operations(current_value / next_value, numbers, index - 1, is_part_one)
    {
        return true;
    }
    if current_value >= next_value
        && apply_operations(current_value - next_value, numbers, index - 1, is_part_one)
    {
        return true;
    }
    if !is_part_one {
        let digit_offset = 10u64.pow(next_value.ilog10() + 1);
        if current_value % digit_offset == next_value
            && apply_operations(
                current_value / digit_offset,
                numbers,
                index - 1,
                is_part_one,
            )
        {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let result: u64 = input
        .lines()
        .filter_map(|line| {
            let (target_string, numbers_string) = line.split_once(": ").unwrap();
            let target = target_string.parse::<u64>().unwrap();
            let numbers: Vec<u64> = numbers_string
                .split_ascii_whitespace()
                .map(|val| val.parse::<u64>().unwrap())
                .collect();

            if apply_operations(target, &numbers, numbers.len() - 1, true) {
                Some(target)
            } else {
                None
            }
        })
        .sum();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result: u64 = input
        .lines()
        .filter_map(|line| {
            let (target_string, numbers_string) = line.split_once(": ").unwrap();
            let target = target_string.parse::<u64>().unwrap();
            let numbers: Vec<u64> = numbers_string
                .split_ascii_whitespace()
                .map(|val| val.parse::<u64>().unwrap())
                .collect();

            if apply_operations(target, &numbers, numbers.len() - 1, false) {
                Some(target)
            } else {
                None
            }
        })
        .sum();
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
