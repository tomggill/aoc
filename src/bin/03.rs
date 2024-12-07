advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result = 0;
    for numbers in re.captures_iter(input) {
        let first_number: u32 = numbers[1].parse().unwrap();
        let second_number: u32 = numbers[2].parse().unwrap();
        result += first_number * second_number;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

    let mut result = 0;
    let mut do_mult = true;
    for capture in re.captures_iter(input) {
        if let Some(mat) = capture.get(0) {
            if mat.as_str() == "don't()" {
                do_mult = false;
            } else if mat.as_str() == "do()" {
                do_mult = true;
            } else {
                let first_number: u32 = capture[2].parse().unwrap();
                let second_number: u32 = capture[3].parse().unwrap();
                if do_mult {
                    result += first_number * second_number;
                }
            }
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
