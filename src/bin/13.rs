use regex::Regex;

advent_of_code::solution!(13);

struct Machine {
    a_x: i32,
    a_y: i32,
    b_x: i32,
    b_y: i32,
    prize_x: i32,
    prize_y: i32,
}

impl Machine {
    fn solve(self, part_one: bool) -> Option<(f64, f64)> {
        let a = self.a_x as f64;
        let b = self.b_x as f64;
        let c = self.a_y as f64;
        let d = self.b_y as f64;
        let determinant = a * d - b * c;
        if determinant == 0.0 {
            return None;
        }
        let mut prize_x = self.prize_x as f64;
        let mut prize_y = self.prize_y as f64;
        if !part_one {
            prize_x += 10000000000000.0;
            prize_y += 10000000000000.0;
        }

        let a_clicks = (d * prize_x as f64 - (b * prize_y as f64)) / determinant;
        let b_clicks = (-1.0 * c * prize_x as f64 + (a * prize_y as f64)) / determinant;
        Some((a_clicks, b_clicks))
    }
}

pub fn parse_input(input: &str, part_one: bool) -> u64 {
    let re = Regex::new(r"Button A: X(?P<a_x>[-+]?\d+), Y(?P<a_y>[-+]?\d+)\nButton B: X(?P<b_x>[-+]?\d+), Y(?P<b_y>[-+]?\d+)\nPrize: X=(?P<prize_x>\d+), Y=(?P<prize_y>\d+)").unwrap();
    let mut result = 0;
    for captures in re.captures_iter(input) {
        let machine = Machine {
            a_x: captures["a_x"].parse().unwrap(),
            a_y: captures["a_y"].parse().unwrap(),
            b_x: captures["b_x"].parse().unwrap(),
            b_y: captures["b_y"].parse().unwrap(),
            prize_x: captures["prize_x"].parse().unwrap(),
            prize_y: captures["prize_y"].parse().unwrap(),
        };
        if let Some((a_clicks, b_clicks)) = machine.solve(part_one) {
            if a_clicks.fract() == 0.0 && b_clicks.fract() == 0.0 {
                result += 3 * a_clicks as u64 + b_clicks as u64;
            }
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse_input(input, true))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse_input(input, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
