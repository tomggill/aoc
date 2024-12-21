use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

advent_of_code::solution!(16);

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

    fn rotate_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position(i32, i32);

impl Position {
    fn get_next_position(&mut self, direction: &Direction) -> Self {
        let direction_movement = direction.get_movement();
        return Position(self.0 + direction_movement.0, self.1 + direction_movement.1);
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Reindeer {
    direction: Direction,
    position: Position,
    cost: i32,
    paths: Vec<Position>,
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Reindeer, Position) {
    let mut reindeer = Reindeer {
        direction: Direction::Right,
        position: Position(0, 0),
        cost: 0,
        paths: Vec::new(),
    };
    let mut grid = Vec::new();
    let mut end_tile = Position(0, 0);
    for (x, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (y, char) in line.chars().enumerate() {
            match char {
                'S' => reindeer.position = Position(x as i32, y as i32),
                'E' => end_tile = Position(x as i32, y as i32),
                _ => {}
            }
            row.push(char);
        }
        grid.push(row);
    }
    (grid, reindeer, end_tile)
}

fn find_min_cost(grid: Vec<Vec<char>>, reindeer: Reindeer, end_tile: Position) -> Option<u32> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(Reverse(reindeer));

    while let Some(Reverse(mut current)) = heap.pop() {
        if current.position == end_tile {
            return Some(current.cost as u32);
        }
        if visited.contains(&(current.position, current.direction)) {
            continue;
        }
        visited.insert((current.position, current.direction));

        let next_position = current.position.get_next_position(&current.direction);
        if grid[next_position.0 as usize][next_position.1 as usize] != '#' {
            heap.push(Reverse(Reindeer {
                position: next_position,
                direction: current.direction,
                cost: current.cost + 1,
                paths: Vec::new(),
            }))
        }

        let left_position = current
            .position
            .get_next_position(&current.direction.rotate_left());
        if grid[left_position.0 as usize][left_position.1 as usize] != '#' {
            heap.push(Reverse(Reindeer {
                position: left_position,
                direction: current.direction.rotate_left(),
                cost: current.cost + 1001,
                paths: Vec::new(),
            }));
        }

        let right_position = current
            .position
            .get_next_position(&current.direction.rotate_right());
        if grid[right_position.0 as usize][right_position.1 as usize] != '#' {
            heap.push(Reverse(Reindeer {
                position: right_position,
                direction: current.direction.rotate_right(),
                cost: current.cost + 1001,
                paths: Vec::new(),
            }));
        }
    }
    None
}

fn find_min_cost_tiles(grid: Vec<Vec<char>>, reindeer: Reindeer, end_tile: Position) -> Option<u32> {
    let mut heap = BinaryHeap::new();
    let mut visited_costs: HashMap<(Position, Direction), i32> = HashMap::new();
    let mut optimal_tiles: HashSet<Position> = HashSet::new();
    let mut min_cost = i32::MAX;
    heap.push(Reverse(reindeer));

    while let Some(Reverse(mut current)) = heap.pop() {
        if current.cost > min_cost {
            continue;
        }

        if current.position == end_tile {
            min_cost = current.cost;
            optimal_tiles.extend(current.paths);
            optimal_tiles.insert(current.position);
            continue;
        }

        if let Some(&existing_cost) = visited_costs.get(&(current.position, current.direction)) {
            if current.cost > existing_cost {
                continue;
            }
        }
        visited_costs.insert((current.position, current.direction), current.cost);

        let next_position = current.position.get_next_position(&current.direction);
        if grid[next_position.0 as usize][next_position.1 as usize] != '#' {
            heap.push(Reverse(Reindeer {
                position: next_position,
                direction: current.direction,
                cost: current.cost + 1,
                paths: {
                    let mut paths = current.paths.clone();
                    paths.push(current.position);
                    paths
                }
            }));
        }

        let left_position = current
            .position
            .get_next_position(&current.direction.rotate_left());
        if grid[left_position.0 as usize][left_position.1 as usize] != '#' {
            heap.push(Reverse(Reindeer {
                position: left_position,
                direction: current.direction.rotate_left(),
                cost: current.cost + 1001,
                paths: {
                    let mut paths = current.paths.clone();
                    paths.push(current.position);
                    paths
                }
            }));
        }

        let right_position = current
            .position
            .get_next_position(&current.direction.rotate_right());
        if grid[right_position.0 as usize][right_position.1 as usize] != '#' {
            heap.push(Reverse(Reindeer {
                position: right_position,
                direction: current.direction.rotate_right(),
                cost: current.cost + 1001,
                paths: {
                    let mut paths = current.paths.clone();
                    paths.push(current.position);
                    paths
                }
            }));
        }
    }

    Some(optimal_tiles.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, reindeer, end_tile) = parse_input(input);
    find_min_cost(grid, reindeer, end_tile)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, reindeer, end_tile) = parse_input(input);
    find_min_cost_tiles(grid, reindeer, end_tile)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
