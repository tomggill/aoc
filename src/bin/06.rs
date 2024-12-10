use std::collections::HashSet;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(6);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_movement(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn next_direction(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position(isize, isize);

pub fn part_one(input: &str) -> Option<u32> {
    let mut guard_position = Position(0, 0);
    let mut guard_direction = Direction::Up;
    let mut obstacles = HashSet::new();
    let mut visited = HashSet::new();
    let mut row_length = 0;
    let mut col_length = 0;
    for (row_idx, line) in input.lines().enumerate() {
        col_length += 1;
        row_length = line.len() as isize;
        for (col_idx, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    guard_position = Position(row_idx as isize, col_idx as isize);
                    guard_direction = Direction::Up;
                    visited.insert(guard_position);
                }
                'v' => {
                    guard_position = Position(row_idx as isize, col_idx as isize);
                    guard_direction = Direction::Down;
                    visited.insert(guard_position);
                }
                '<' => {
                    guard_position = Position(row_idx as isize, col_idx as isize);
                    guard_direction = Direction::Left;
                    visited.insert(guard_position);
                }
                '>' => {
                    guard_position = Position(row_idx as isize, col_idx as isize);
                    guard_direction = Direction::Right;
                    visited.insert(guard_position);
                }
                '#' => {
                    obstacles.insert(Position(row_idx as isize, col_idx as isize));
                },
                _ => {}
            }
        }
    }
    let mut position = guard_position;
    let mut direction = guard_direction;

    while position.0 < row_length && position.0 >= 0 && position.1 < col_length && position.1 >= 0 {
        let next_position = Position(position.0 + direction.get_movement().0, position.1 + direction.get_movement().1);
        if obstacles.contains(&next_position) {
            direction = direction.next_direction();
        } else {
            position = next_position;
            visited.insert(position);
        }
    }
    Some(visited.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut guard_position = Position(0, 0);
    let mut guard_direction = Direction::Up;
    let mut obstacles = HashSet::new();
    let mut visited = HashSet::new();
    let mut row_length = 0;
    let mut col_length = 0;
    for (row_idx, line) in input.lines().enumerate() {
        col_length += 1;
        row_length = line.len() as isize;
        for (col_idx, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    guard_position = Position(row_idx as isize, col_idx as isize);
                    guard_direction = Direction::Up;
                    visited.insert(guard_position);
                }
                'v' => {
                    guard_position = Position(row_idx as isize, col_idx as isize);
                    guard_direction = Direction::Down;
                    visited.insert(guard_position);
                }
                '<' => {
                    guard_position = Position(row_idx as isize, col_idx as isize);
                    guard_direction = Direction::Left;
                    visited.insert(guard_position);
                }
                '>' => {
                    guard_position = Position(row_idx as isize, col_idx as isize);
                    guard_direction = Direction::Right;
                    visited.insert(guard_position);
                }
                '#' => {
                    obstacles.insert(Position(row_idx as isize, col_idx as isize));
                },
                _ => {}
            }
        }
    }
    let mut position = guard_position;
    let mut direction = guard_direction;

    while position.0 < row_length && position.0 >= 0 && position.1 < col_length && position.1 >= 0 {
        let next_position = Position(position.0 + direction.get_movement().0, position.1 + direction.get_movement().1);
        if obstacles.contains(&next_position) {
            direction = direction.next_direction();
        } else {
            position = next_position;
            visited.insert(position);
        }
    }

    let res = visited
        .par_iter()
        .fold_with(0, |acc, new_obstacle| {
            if has_loop(&obstacles, row_length, col_length, new_obstacle, &guard_position, &guard_direction) {
                return acc + 1;
            };
            acc
        })
        .sum();
    Some(res)
}

fn has_loop(obstacles: &HashSet<Position>, row_length: isize, col_length: isize, new_obstacle: &Position, start_position: &Position, start_direction: &Direction) -> bool {
    let mut position = *start_position;
    let mut direction = *start_direction;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&(position, direction)) {
            return true;
        }
        visited.insert((position, direction));

        let next_position = Position(position.0 + direction.get_movement().0, position.1 + direction.get_movement().1);
        if next_position.0 >= row_length || next_position.0 < 0 || next_position.1 >= col_length || next_position.1 < 0 {
            return false;
        }

        if obstacles.contains(&next_position) || next_position.eq(new_obstacle) {
            direction = direction.next_direction();
        } else {
            position = next_position;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
