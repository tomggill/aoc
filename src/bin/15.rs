use std::{collections::HashSet, mem::swap, ops::{Index, IndexMut}};

advent_of_code::solution!(15);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position(i32, i32);

impl Position {
    fn get_next_position(&mut self, step: &Step) -> Self {
        let step_movement = step.get_movement();
        return Position(self.0 + step_movement.0, self.1 + step_movement.1);
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Grid {
    cells: Vec<Vec<char>>
}

impl Index<Position> for Grid {
    type Output = char;

    fn index(&self, position: Position) -> &Self::Output {
        &self.cells[position.0 as usize][position.1 as usize]
    }
}

impl IndexMut<Position> for Grid {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.cells[position.0 as usize][position.1 as usize]
    }
}


enum Step {
    Up,
    Down,
    Left,
    Right,
}

impl Step {
    fn get_movement(&self) -> (i32, i32) {
        match self {
            Step::Up => (-1, 0),
            Step::Down => (1, 0),
            Step::Left => (0, -1),
            Step::Right => (0, 1),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robot = Position(0, 0);
    let mut grid_elements = Vec::new();
    let (map_string, move_string) = input.split_once("\n\n").unwrap();
    for (x, line) in map_string.lines().enumerate() {
        let mut row = Vec::new();
        for (y, char) in line.chars().enumerate() {
            row.push(char);
            if char == '@' {
                robot = Position(x as i32, y as i32);
            }
        }
        grid_elements.push(row);
    }
    let mut grid = Grid {
        cells: grid_elements,
    };
    for char in move_string.chars() {
        match char {
            '^' => perform_move_narrow(&mut grid, &mut robot, Step::Up),
            'v' => perform_move_narrow(&mut grid, &mut robot, Step::Down),
            '<' => perform_move_narrow(&mut grid, &mut robot, Step::Left),
            '>' => perform_move_narrow(&mut grid, &mut robot, Step::Right),
            _ => {}
        }
    }
    Some(calculate_gps(grid))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robot = Position(0, 0);
    let mut grid_elements = Vec::new();
    let (map_string, move_string) = input.split_once("\n\n").unwrap();
    for (x, line) in map_string.lines().enumerate() {
        let mut row = Vec::new();
        for (y, char) in line.chars().enumerate() {
            let (left, right) = match char {
                '#' => ('#', '#'),
                'O' => ('[', ']'),
                '.' => ('.', '.'),
                '@' => {
                    robot = Position(x as i32, (y * 2) as i32);
                    ('@', '.')
                },
                _ => continue,
            };
            row.push(left);
            row.push(right);
        }
        grid_elements.push(row);
    }
    let mut grid = Grid {
        cells: grid_elements,
    };

    for char in move_string.chars() {
        match char {
            '^' => perform_move_wide(&mut grid, &mut robot, Step::Up),
            'v' => perform_move_wide(&mut grid, &mut robot, Step::Down),
            '<' => perform_move_narrow(&mut grid, &mut robot, Step::Left),
            '>' => perform_move_narrow(&mut grid, &mut robot, Step::Right),
            _ => {}
        }
    }

    Some(calculate_gps(grid))
}

fn perform_move_narrow(grid: &mut Grid, robot: &mut Position, step: Step) {
    let mut next_position = robot.get_next_position(&step);
    let mut size = 2;
    while grid[next_position] != '#' && grid[next_position] != '.' {
        next_position = next_position.get_next_position(&step);
        size += 1;
    }

    if grid[next_position] == '.' {
        let mut previous = '.';
        let mut position = *robot;
        for _ in 0..size {
            swap(&mut previous, &mut grid[position]);
            position = position.get_next_position(&step);
        }
        *robot = robot.get_next_position(&step);
    }
}

fn perform_move_wide(grid: &mut Grid, robot: &mut Position, step: Step) {
    let next_position = robot.get_next_position(&step);
    if grid[next_position] == '.' {
        grid[*robot] = '.';
        grid[next_position] = '@';
        *robot = next_position;
        return;
    }

    let mut seen = HashSet::new();
    let mut queue = vec![*robot];
    let mut index = 0;
    while index < queue.len() {
        let mut next_position = queue[index].get_next_position(&step);
        index += 1;

        let direction = match grid[next_position] {
            '[' => Step::Right,
            ']' => Step::Left,
            '#' => return,
            _ => continue,
        };

        if !seen.contains(&next_position) {
            seen.insert(next_position);
            queue.push(next_position);
        }
        let other = next_position.get_next_position(&direction);
        if !seen.contains(&other) {
            seen.insert(other);
            queue.push(other);
        }
    }
    for position in queue.iter_mut().rev() {
        grid[position.get_next_position(&step)] = grid[*position];
        grid[*position] = '.';
    }

    *robot = robot.get_next_position(&step);
}

fn calculate_gps(grid: Grid) -> u32 {
    let mut result = 0;
    for x in 0..grid.cells.len() {
        for y in 0..grid.cells[0].len() {
            let position = Position(x as i32, y as i32);
            if grid[position] == 'O' || grid[position] == '[' {
                result += 100 * position.0 + position.1;
            }
        }
    }
    result as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
