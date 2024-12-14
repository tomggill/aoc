advent_of_code::solution!(14);

const BOARD_HEIGHT: i32 = 103;
const BOARD_WIDTH: i32 = 101;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position(i32, i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Velocity(i32, i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn tick(&mut self) {
        fn wrap_around(coordinate: i32, limit: i32) -> i32 {
            if coordinate >= limit {
                coordinate % limit
            } else if coordinate < 0 {
                limit + coordinate
            } else {
                coordinate
            }
        }
        self.position.0 = wrap_around(self.position.0 + self.velocity.0, BOARD_WIDTH);
        self.position.1 = wrap_around(self.position.1 + self.velocity.1, BOARD_HEIGHT);
    }
}

fn parse_robots(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in input.lines() {
        if let Some((position_part, velocity_part)) = line.split_once(" v=") {
            let position_string = position_part.strip_prefix("p=").unwrap();
            let (position_x, position_y) = position_string.split_once(",").unwrap();

            let (velocity_x, velocity_y) = velocity_part.split_once(",").unwrap();

            let robot = Robot {
                position: Position(
                    position_x.parse::<i32>().unwrap(),
                    position_y.parse::<i32>().unwrap(),
                ),
                velocity: Velocity(
                    velocity_x.parse::<i32>().unwrap(),
                    velocity_y.parse::<i32>().unwrap(),
                ),
            };
            robots.push(robot);
        }
    }
    robots
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots = parse_robots(input);
    for _ in 0..100 {
        for robot in &mut robots {
            robot.tick();
        }
    }
    let middle_x = BOARD_WIDTH / 2;
    let middle_y = BOARD_HEIGHT / 2;
    let mut quadrant = (0, 0, 0, 0);
    for robot in &robots {
        match (robot.position.0, robot.position.1) {
            (x, y) if x < middle_x && y < middle_y => quadrant.0 += 1,
            (x, y) if x < middle_x && y > middle_y => quadrant.1 += 1,
            (x, y) if x > middle_x && y < middle_y => quadrant.2 += 1,
            (x, y) if x > middle_x && y > middle_y => quadrant.3 += 1,
            _ => {}
        }
    }
    Some(quadrant.0 * quadrant.1 * quadrant.2 * quadrant.3)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
