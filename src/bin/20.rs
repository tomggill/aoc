use std::collections::{HashMap, HashSet};

advent_of_code::solution!(20);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position(i32, i32);

impl Position {
    fn get_next_position(&mut self, direction: &Direction) -> Self {
        let direction_movement = direction.get_movement();
        return Position(self.0 + direction_movement.0, self.1 + direction_movement.1);
    }

    fn get_cheat_position(&mut self, direction: &Direction) -> Self {
        let direction_movement = direction.get_movement();
        return Position(
            self.0 + (direction_movement.0 * 2),
            self.1 + (direction_movement.1 * 2),
        );
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_movement(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct RaceTrack {
    track: HashMap<Position, u64>,
    total_time: u64,
    start: Position,
    end: Position,
    map: Vec<Vec<char>>,
    directions: Vec<Direction>,
}

impl RaceTrack {
    fn new(input: &str) -> Self {
        let mut map = Vec::new();
        let mut start = Position(0, 0);
        let mut end = Position(0, 0);
        for (x, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (y, char) in line.chars().enumerate() {
                match char {
                    'S' => {
                        start = Position(x as i32, y as i32);
                    }
                    'E' => {
                        end = Position(x as i32, y as i32);
                    }
                    _ => {}
                }
                row.push(char);
            }
            map.push(row);
        }
        RaceTrack {
            track: HashMap::new(),
            total_time: 0,
            map,
            start,
            end,
            directions: vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
        }
    }

    fn traverse_path(&mut self) {
        let mut current_position = self.start;
        self.track.insert(current_position, self.total_time);

        let mut visited = HashSet::new();
        visited.insert(current_position);

        while current_position != self.end {
            for direction in &self.directions {
                let next_position = current_position.get_next_position(direction);
                if !visited.contains(&next_position)
                    && (self.map[next_position.0 as usize][next_position.1 as usize] == '.'
                        || self.map[next_position.0 as usize][next_position.1 as usize] == 'E')
                {
                    current_position = next_position;
                    break;
                }
            }
            visited.insert(current_position);
            self.total_time += 1;
            self.track.insert(current_position, self.total_time);
        }
    }

    fn calculate_cheats(&mut self) -> u64 {
        let mut result = 0;
        for (mut position, moment) in self.track.clone() {
            for direction in &self.directions {
                let cheat_position = position.get_cheat_position(direction);
                if self.track.contains_key(&cheat_position) {
                    let cheat_moment = self.track.get(&cheat_position).unwrap();
                    if *cheat_moment as i64 - moment as i64 > 100 {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut race_track = RaceTrack::new(input);
    race_track.traverse_path();
    Some(race_track.calculate_cheats())
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
