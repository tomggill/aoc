use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position(i32, i32);

fn search_trailhead(
    grid: &Vec<Vec<char>>,
    position: &Position,
    reachable_peaks: &mut HashSet<Position>,
) {
    if grid[position.0 as usize][position.1 as usize] == '9' {
        reachable_peaks.insert(*position);
        return;
    }

    for direction in DIRECTIONS {
        let new_position = Position(position.0 + direction.0, position.1 + direction.1);
        if new_position.0 < 0
            || new_position.0 >= grid.len() as i32
            || new_position.1 < 0
            || new_position.1 >= grid[0].len() as i32
        {
            continue;
        }
        if grid[new_position.0 as usize][new_position.1 as usize] as i32
            - grid[position.0 as usize][position.1 as usize] as i32
            != 1
        {
            continue;
        }
        search_trailhead(grid, &new_position, reachable_peaks);
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, HashSet<Position>) {
    let mut grid = Vec::new();
    let mut trailheads = HashSet::new();
    for (row_idx, line) in input.lines().enumerate() {
        let mut grid_row = Vec::new();
        for (col_idx, char) in line.chars().enumerate() {
            if char == '0' {
                trailheads.insert(Position(row_idx as i32, col_idx as i32));
            }
            grid_row.push(char);
        }
        grid.push(grid_row);
    }
    (grid, trailheads)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, trailheads) = parse_input(input);

    let mut reachable_peaks = 0;
    for trailhead in trailheads {
        let mut trailhead_peaks = HashSet::new();
        search_trailhead(&grid, &trailhead, &mut trailhead_peaks);
        reachable_peaks += trailhead_peaks.len();
    }
    Some(reachable_peaks as u32)
}

fn search_trailhead_two(
    grid: &Vec<Vec<char>>,
    position: &Position,
    reachable_peaks: &mut Vec<Position>,
) {
    if grid[position.0 as usize][position.1 as usize] == '9' {
        reachable_peaks.push(*position);
        return;
    }

    for direction in DIRECTIONS {
        let new_position = Position(position.0 + direction.0, position.1 + direction.1);
        if new_position.0 < 0
            || new_position.0 >= grid.len() as i32
            || new_position.1 < 0
            || new_position.1 >= grid[0].len() as i32
        {
            continue;
        }
        if grid[new_position.0 as usize][new_position.1 as usize] as i32
            - grid[position.0 as usize][position.1 as usize] as i32
            != 1
        {
            continue;
        }
        search_trailhead_two(grid, &new_position, reachable_peaks);
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, trailheads) = parse_input(input);

    let mut trailhead_peaks = Vec::new();
    for trailhead in trailheads {
        search_trailhead_two(&grid, &trailhead, &mut trailhead_peaks);
    }
    Some(trailhead_peaks.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
