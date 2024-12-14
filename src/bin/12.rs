use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(12);

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position(i32, i32);

fn flood_fill_area_and_perimeter(
    grid: &Vec<Vec<char>>,
    position: Position,
    visited: &mut HashSet<Position>,
    plant_type: char,
) -> (i32, i32) {
    let mut queue = VecDeque::new();
    queue.push_back(position);
    visited.insert(position);
    let mut area = 0;
    let mut perimeter = 0;

    let rows = grid.len();
    let cols = grid[0].len();
    while let Some(current_position) = queue.pop_front() {
        area += 1;
        for (dr, dc) in DIRECTIONS {
            let next_position = Position(current_position.0 + dr, current_position.1 + dc);
            if next_position.0 < 0
                || next_position.0 >= rows as i32
                || next_position.1 < 0
                || next_position.1 >= cols as i32
            {
                perimeter += 1;
            } else {
                if grid[next_position.0 as usize][next_position.1 as usize] != plant_type {
                    perimeter += 1
                } else if !visited.contains(&next_position) {
                    visited.insert(next_position);
                    queue.push_back(next_position);
                }
            }
        }
    }
    (area, perimeter)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut total_price = 0;
    for row in 0..rows {
        for col in 0..cols {
            let position = Position(row as i32, col as i32);
            if !visited.contains(&position) {
                let plant_type = grid[row][col];
                let (area, perimeter) =
                    flood_fill_area_and_perimeter(&grid, position, &mut visited, plant_type);
                total_price += area * perimeter
            }
        }
    }

    Some(total_price as u32)
}

fn is_boundary_change(
    grid: &Vec<Vec<char>>,
    position: &Position,
    plant_type: char,
    side_one_out_of_bound: bool,
    side_two_out_of_bound: bool,
    side_one_direction: (i32, i32),
    side_two_direction: (i32, i32),
) -> bool {
    if side_one_out_of_bound == side_two_out_of_bound {
        if side_one_out_of_bound {
            return true;
        }

        let diagonal_position = Position(
            position.0 + side_one_direction.0 + side_two_direction.0,
            position.1 + side_one_direction.1 + side_two_direction.1,
        );

        let within_bounds = diagonal_position.0 >= 0
            && diagonal_position.0 < grid.len() as i32
            && diagonal_position.1 >= 0
            && diagonal_position.1 < grid[0].len() as i32;

        if !within_bounds
            || grid[diagonal_position.0 as usize][diagonal_position.1 as usize] != plant_type
        {
            return true;
        }
    }
    false
}

fn is_out_of_region(
    grid: &Vec<Vec<char>>,
    direction: &(i32, i32),
    position: Position,
    plant_type: char,
    visited: &mut HashSet<Position>,
    queue: &mut VecDeque<Position>,
) -> bool {
    let next_position = Position(position.0 + direction.0, position.1 + direction.1);

    let out_of_bounds = next_position.0 < 0
        || next_position.0 >= grid.len() as i32
        || next_position.1 < 0
        || next_position.1 >= grid[0].len() as i32;

    let is_different_plant =
        !out_of_bounds && grid[next_position.0 as usize][next_position.1 as usize] != plant_type;

    let is_out_of_region = out_of_bounds || is_different_plant;

    if !is_out_of_region && !visited.contains(&next_position) {
        visited.insert(next_position);
        queue.push_back(next_position);
    }

    is_out_of_region
}

fn flood_fill_area_and_sides(
    grid: &Vec<Vec<char>>,
    position: Position,
    visited: &mut HashSet<Position>,
    plant_type: char,
) -> (i32, i32) {
    let mut queue = VecDeque::new();
    queue.push_back(position);
    visited.insert(position);
    let mut area = 0;
    let mut sides = 0;

    while let Some(current_position) = queue.pop_front() {
        area += 1;
        let sides_out_of_region: Vec<bool> = DIRECTIONS
            .iter()
            .map(|direction| {
                is_out_of_region(
                    grid,
                    direction,
                    current_position,
                    plant_type,
                    visited,
                    &mut queue,
                )
            })
            .collect();

        sides += sides_out_of_region
            .iter()
            .circular_tuple_windows()
            .enumerate()
            .filter(|(i, (side_one, side_two))| {
                is_boundary_change(
                    grid,
                    &current_position,
                    plant_type,
                    **side_one,
                    **side_two,
                    DIRECTIONS[i % DIRECTIONS.len()],
                    DIRECTIONS[(i + 1) % DIRECTIONS.len()],
                )
            })
            .count();
    }
    (area, sides as i32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut total_price = 0;
    for row in 0..rows {
        for col in 0..cols {
            let position = Position(row as i32, col as i32);
            if !visited.contains(&position) {
                let plant_type = grid[row][col];
                let (area, sides) =
                    flood_fill_area_and_sides(&grid, position, &mut visited, plant_type);
                total_price += area * sides
            }
        }
    }

    Some(total_price as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
