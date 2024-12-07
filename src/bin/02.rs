use std::collections::HashSet;

advent_of_code::solution!(2);

/*
* Cleaner but slower
*/
// pub fn part_one(input: &str) -> Option<u32> {
//     let mut result = 0;
//     for level in input.lines() {
//         let level_nums: Vec<i32> = level
//             .split_whitespace()
//             .filter_map(|s| s.parse().ok())
//             .collect();

//         let is_increasing = level_nums.windows(2).all(|w| w[0] < w[1]);
//         let is_decreasing = level_nums.windows(2).all(|w| w[0] > w[1]);

//         let is_valid_difference = level_nums
//             .windows(2)
//             .all(|w| (1 <= w[1].abs_diff(w[0]) && w[1].abs_diff(w[0]) <= 3));
//         if (is_increasing || is_decreasing) && is_valid_difference {
//             result += 1
//         }
//     }
//     Some(result)
// }
//

/*
* Cleaner but slower
*/
// pub fn part_two(input: &str) -> Option<u32> {
//     let mut result = 0;
//     for level in input.lines() {
//         let numbers: Vec<i32> = level
//             .split_whitespace()
//             .filter_map(|s| s.parse::<i32>().ok())
//             .collect();

//         for i in 0..numbers.len() {
//             let mut modified = numbers.clone();
//             modified.remove(i);
//             if is_valid_level(&modified) {
//                 result += 1;
//                 break;
//             }
//         }
//     }
//     Some(result)
// }

fn is_valid_level(level_numbers: &[i32]) -> bool {
    let mut iter = level_numbers.iter();
    let mut prev = iter.next();
    let mut is_increasing = true;
    let mut is_decreasing = true;

    while let Some(current) = iter.next() {
        if let Some(prev_value) = prev {
            let diff = current - prev_value;

            if diff < 0 {
                is_increasing = false;
            }

            if diff > 0 {
                is_decreasing = false;
            }

            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }
        }
        prev = Some(current);
    }

    is_increasing || is_decreasing
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    for level in input.lines() {
        let numbers: Vec<i32> = level
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if is_valid_level(&numbers) {
            result += 1;
        }
    }
    Some(result)
}

fn is_valid_level_two(level_numbers: &[i32]) -> bool {
    let mut iter = level_numbers.iter();
    let mut prev = iter.next();
    let mut is_increasing = true;
    let mut is_decreasing = true;

    while let Some(current) = iter.next() {
        if let Some(prev_value) = prev {
            let diff = current - prev_value;

            if diff < 0 {
                is_increasing = false;
            }
            if diff > 0 {
                is_decreasing = false;
            }

            if diff.abs() < 1 || diff.abs() > 3 || (!is_increasing && !is_decreasing) {
                let idx = level_numbers.iter().position(|&x| x == *current).unwrap();

                let without_current = [&level_numbers[..idx], &level_numbers[idx + 1..]].concat();
                if is_valid_level(&without_current) {
                    return true;
                }

                if idx > 0 {
                    let without_prev = [&level_numbers[..idx - 1], &level_numbers[idx..]].concat();
                    if is_valid_level(&without_prev) {
                        return true;
                    }
                }

                // Handle edge case
                if idx == 2 {
                    let without_first_element = &level_numbers[1..];
                    if is_valid_level(&without_first_element) {
                        return true;
                    }
                }

                return false;
            }
        }
        prev = Some(current);
    }

    is_increasing || is_decreasing
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    for level in input.lines() {
        let numbers: Vec<i32> = level
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if is_valid_level_two(&numbers) {
            result += 1;
            continue;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
