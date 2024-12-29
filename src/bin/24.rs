use std::collections::HashMap;

advent_of_code::solution!(24);


#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Operation {
   And,
   Xor,
   Or,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct GateInput {
    input1: String,
    input2: String,
    operation: Operation,
}

fn parse_input(input: &str) -> ( HashMap<String, bool>, HashMap<String, GateInput>, Vec<String>){
    let mut states = HashMap::new();
    let (states_string, gates_string) = input.split_once("\n\n").unwrap();
    for line in states_string.lines() {
        let (state, value) = line.split_once(": ").unwrap();
        states.insert(state.to_string(), value.parse::<i32>().ok().map(|v| v == 1).unwrap());
    }

    let mut z_wires = Vec::new();
    let mut gates = HashMap::new();
    for line in gates_string.lines() {
        let gate_components: Vec<&str> = line.split_whitespace().collect();
        let (input1, operation_str, input2, output) = (gate_components[0], gate_components[1], gate_components[2], gate_components[4]);
        let operation: Operation = match operation_str {
            "AND" => Operation::And,
            "XOR" => Operation::Xor,
            "OR" => Operation::Or,
            _ => panic!("Unsupported operation: {}", operation_str),
        };
        if output.starts_with('z') {
            z_wires.push(output.to_string());
        }
        gates.insert(output.to_string(), GateInput { input1: input1.to_string(), input2: input2.to_string(), operation });
    }

    (states, gates, z_wires)
}

fn resolve(wire: &String, states: &mut HashMap<String, bool>, gates: &HashMap<String, GateInput>) -> bool {
    if let Some(&value) = states.get(wire) {
        return value;
    }

    let gate = gates.get(wire).unwrap();
    let result = match gate.operation {
        Operation::And => resolve(&gate.input1, states, gates) && resolve(&gate.input2, states, gates),
        Operation::Or => resolve(&gate.input1, states, gates) || resolve(&gate.input2, states, gates),
        Operation::Xor => resolve(&gate.input1, states, gates) != resolve(&gate.input2, states, gates),
    };
    states.insert(wire.to_string(), result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut states, mut dependencies, mut z_wires) = parse_input(input);
    z_wires.sort_by(|a, b| {
        let num_a: i32 = a[1..].parse().unwrap();
        let num_b: i32 = b[1..].parse().unwrap();
        num_b.cmp(&num_a)
    });

    let mut result = 0;

    for wire in z_wires {
        result = (result << 1) | resolve(&wire, &mut states, &mut dependencies) as u64;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
