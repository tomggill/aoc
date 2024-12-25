use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position(i32, i32);

fn calculate_anti_nodes(position_one: &Position, position_two: &Position, x_max: i32, y_max: i32) -> Vec<Position> {
    let mut anti_nodes = Vec::new();
    let offset = Position(position_one.0 - position_two.0, position_one.1 - position_two.1);
    let anti_node_one = Position(position_one.0 + offset.0, position_one.1 + offset.1);
    let anti_node_two = Position(position_two.0 - offset.0, position_two.1 - offset.1);
    if anti_node_one.0 >= 0 && anti_node_one.0 < x_max && anti_node_one.1 >= 0 && anti_node_one.1 < y_max {
        anti_nodes.push(anti_node_one)
    }
    if anti_node_two.0 >= 0 && anti_node_two.0 < x_max && anti_node_two.1 >= 0 && anti_node_two.1 < y_max {
        anti_nodes.push(anti_node_two)
    }
    anti_nodes
}

fn find_anti_nodes(positions: &Vec<Position>, anti_nodes: &mut HashSet<Position>, x_max: i32, y_max: i32) {
    for i in 0..positions.len() {
        for j in i+1..positions.len() {
            anti_nodes.extend(calculate_anti_nodes(&positions[i], &positions[j], x_max, y_max))
        }
    }
}

fn parse_input(input: &str) -> (HashMap<char, Vec<Position>>, i32, i32) {
    let mut antennas = HashMap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            if char != '.' {
                antennas
                .entry(char)
                .or_insert_with(Vec::new)
                .push(Position(x as i32, y as i32))
            }
        }
        x_max += 1;
        y_max = line.len() as i32;
    }
    (antennas, x_max, y_max)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (antennas, x_max, y_max) = parse_input(input);

    let mut anti_nodes = HashSet::new();
    for (_, positions) in antennas.iter() {
        find_anti_nodes(positions, &mut anti_nodes, x_max, y_max);
    }
    Some(anti_nodes.len() as u32)
}

fn calculate_anti_nodes_with_resonance(position_one: &Position, position_two: &Position, x_max: i32, y_max: i32) -> Vec<Position> {
    let mut anti_nodes = Vec::new();
    let offset = Position(position_one.0 - position_two.0, position_one.1 - position_two.1);
    let mut next_anti_node = *position_one;
    while next_anti_node.0 >= 0 && next_anti_node.0 < x_max && next_anti_node.1 >= 0 && next_anti_node.1 < y_max {
        anti_nodes.push(next_anti_node);
        next_anti_node = Position(next_anti_node.0 + offset.0, next_anti_node.1 + offset.1);
    }
    let mut next_anti_node = *position_two;
    while next_anti_node.0 >= 0 && next_anti_node.0 < x_max && next_anti_node.1 >= 0 && next_anti_node.1 < y_max {
        anti_nodes.push(next_anti_node);
        next_anti_node = Position(next_anti_node.0 - offset.0, next_anti_node.1 - offset.1);
    }
    anti_nodes
}

fn find_anti_nodes_with_resonance(positions: &Vec<Position>, anti_nodes: &mut HashSet<Position>, x_max: i32, y_max: i32) {
    for i in 0..positions.len() {
        for j in i+1..positions.len() {
            anti_nodes.extend(calculate_anti_nodes_with_resonance(&positions[i], &positions[j], x_max, y_max))
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (antennas, x_max, y_max) = parse_input(input);
    let mut anti_nodes = HashSet::new();
    for (_, positions) in antennas.iter() {
        find_anti_nodes_with_resonance(positions, &mut anti_nodes, x_max, y_max);
    }
    Some(anti_nodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
