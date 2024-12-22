use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(18);

const WIDTH: i32 = 71;
const HEIGHT: i32 = 71;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position(i32, i32);

impl Position {
    fn get_next_position(&mut self, direction: &Direction) -> Self {
        let direction_movement = direction.get_movement();
        return Position(self.0 + direction_movement.0, self.1 + direction_movement.1);
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

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Grid {
    bytes: Vec<Position>,
    map: Vec<Vec<char>>,
    directions: Vec<Direction>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let map = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];
        let mut bytes = Vec::new();
        input.lines().for_each(|line| {
            let (x, y) = line.split_once(',').unwrap();
            bytes.push(Position(y.parse::<i32>().unwrap(),x.parse::<i32>().unwrap()));
        });
        Grid {
            bytes,
            map,
            directions: vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right],
        }
    }

    fn get_shortest_path(&mut self) -> Option<u32> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((Position(0, 0), 0));
        visited.insert(Position(0, 0));

        let exit = Position(WIDTH - 1, HEIGHT - 1);

        while let Some((mut position, path_size)) = queue.pop_front() {
            if position == exit {
                return Some(path_size);
            }

            for direction in &self.directions {
                let next_position = position.get_next_position(&direction);
                if next_position.0 >= 0 && next_position.0 < HEIGHT && next_position.1 >= 0 && next_position.1 < WIDTH {
                    if self.map[next_position.0 as usize][next_position.1 as usize] == '.' && !visited.contains(&next_position) {
                        visited.insert(next_position);
                        queue.push_back((next_position, path_size + 1));
                    }
                }
            }

        }
        None
    }

    fn binary_search_block(&mut self) -> Option<String> {
        let mut start = 0;
        let mut end = self.bytes.len() as i32;
        while start <= end {
            let middle = (start + end) / 2;
            for position in &self.bytes[0..middle as usize] {
                self.map[position.0 as usize][position.1 as usize] = '#';
            }
            if self.get_shortest_path().is_some() {
                start = middle + 1;
            } else {
                end = middle - 1;
            }
            self.map = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];
        }
        let blocking_byte = self.bytes[(start - 1) as usize];
        let result = blocking_byte.1.to_string() + "," + blocking_byte.0.to_string().as_str();
        Some(result)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::new(input);
    for position in &grid.bytes[0..1024] {
        grid.map[position.0 as usize][position.1 as usize] = '#';
    }
    grid.get_shortest_path()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut grid = Grid::new(input);
    grid.binary_search_block()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
