use itertools::Itertools;

advent_of_code::solution!(4);

#[derive(Copy, Clone, PartialEq, Eq)]
enum Orientation {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Orientation {
    fn direction(&self) -> (i32, i32) {
        match self {
            Orientation::North => (0, -1),
            Orientation::NorthEast => (1, -1),
            Orientation::East => (1, 0),
            Orientation::SouthEast => (1, 1),
            Orientation::South => (0, 1),
            Orientation::SouthWest => (-1, 1),
            Orientation::West => (-1, 0),
            Orientation::NorthWest => (-1, -1),
        }
    }
}

fn dfs(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    orientation: Option<Orientation>,
    index: usize,
) -> usize {
    if index == 4 {
        return 1;
    }
    let current_char = "XMAS".as_bytes()[index] as char;
    if grid[row][col] != current_char {
        return 0;
    }

    if let Some(orientation) = orientation {
        let (col_move, row_move) = orientation.direction();
        let next_row = row as i32 + row_move;
        let next_col = col as i32 + col_move;

        if index == 3 {
            return 1;
        }

        if next_row < 0
            || next_row >= grid.len() as i32
            || next_col < 0
            || next_col >= grid[0].len() as i32
        {
            return 0;
        }

        dfs(
            grid,
            next_row as usize,
            next_col as usize,
            Some(orientation),
            index + 1,
        )
    } else {
        let directions = [
            Orientation::North,
            Orientation::NorthEast,
            Orientation::East,
            Orientation::SouthEast,
            Orientation::South,
            Orientation::SouthWest,
            Orientation::West,
            Orientation::NorthWest,
        ];

        let mut total_matches = 0;
        for dir in directions.iter() {
            let (col_move, row_move) = dir.direction();
            let next_row = row as i32 + row_move;
            let next_col = col as i32 + col_move;

            if next_row >= 0
                && next_row < grid.len() as i32
                && next_col >= 0
                && next_col < grid[0].len() as i32
            {
                total_matches += dfs(
                    grid,
                    next_row as usize,
                    next_col as usize,
                    Some(*dir),
                    index + 1,
                );
            }
        }

        total_matches
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let row_size = grid.len();
    let col_size = grid[0].len();

    let mut result = 0;
    for x in 0..row_size {
        for y in 0..col_size {
            if grid[x][y] == 'X' {
                let d = dfs(&grid, x, y, None, 0);
                result += d;
            }
        }
    }
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let row_size = grid.len();
    let col_size = grid[0].len();

    let mut result = 0;
    for x in 0..row_size {
        for y in 0..col_size {
            if grid[x][y] == 'A' && x > 0 && y > 0 && x < row_size - 1 && y < col_size - 1 {
                let north_west = grid[x - 1][y - 1];
                let south_east = grid[x + 1][y + 1];
                let north_east = grid[x - 1][y + 1];
                let south_west = grid[x + 1][y - 1];
                if ((north_west == 'S' && south_east == 'M')
                    || (north_west == 'M' && south_east == 'S'))
                    && ((north_east == 'S' && south_west == 'M')
                        || (north_east == 'M' && south_west == 'S'))
                {
                    result += 1;
                }
            }
        }
    }
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
