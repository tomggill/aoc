use std::io::Write;
use std::{collections::HashMap, fs::File};

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

fn parse_input(
    input: &str,
) -> (
    HashMap<String, bool>,
    HashMap<String, GateInput>,
    Vec<String>,
) {
    let mut states = HashMap::new();
    let (states_string, gates_string) = input.split_once("\n\n").unwrap();
    for line in states_string.lines() {
        let (state, value) = line.split_once(": ").unwrap();
        states.insert(
            state.to_string(),
            value.parse::<i32>().ok().map(|v| v == 1).unwrap(),
        );
    }

    let mut z_wires = Vec::new();
    let mut gates = HashMap::new();
    for line in gates_string.lines() {
        let gate_components: Vec<&str> = line.split_whitespace().collect();
        let (input1, operation_str, input2, output) = (
            gate_components[0],
            gate_components[1],
            gate_components[2],
            gate_components[4],
        );
        let operation: Operation = match operation_str {
            "AND" => Operation::And,
            "XOR" => Operation::Xor,
            "OR" => Operation::Or,
            _ => panic!("Unsupported operation: {}", operation_str),
        };
        if output.starts_with('z') {
            z_wires.push(output.to_string());
        }
        gates.insert(
            output.to_string(),
            GateInput {
                input1: input1.to_string(),
                input2: input2.to_string(),
                operation,
            },
        );
    }

    (states, gates, z_wires)
}

fn resolve(
    wire: &String,
    states: &mut HashMap<String, bool>,
    gates: &HashMap<String, GateInput>,
) -> bool {
    if let Some(&value) = states.get(wire) {
        return value;
    }

    let gate = gates.get(wire).unwrap();
    let result = match gate.operation {
        Operation::And => {
            resolve(&gate.input1, states, gates) && resolve(&gate.input2, states, gates)
        }
        Operation::Or => {
            resolve(&gate.input1, states, gates) || resolve(&gate.input2, states, gates)
        }
        Operation::Xor => {
            resolve(&gate.input1, states, gates) != resolve(&gate.input2, states, gates)
        }
    };
    states.insert(wire.to_string(), result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut states, mut gates, mut z_wires) = parse_input(input);
    z_wires.sort_by(|a, b| {
        let num_a: i32 = a[1..].parse().unwrap();
        let num_b: i32 = b[1..].parse().unwrap();
        num_b.cmp(&num_a)
    });

    let mut result = 0;

    for wire in z_wires {
        result = (result << 1) | resolve(&wire, &mut states, &mut gates) as u64;
    }
    Some(result)
}

fn binary_add(x: Vec<i32>, y: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut carry = 0;

    let len = x.len().max(y.len());

    for i in 0..len {
        let a = *x.get(i).unwrap_or(&0); // Get the i-th bit of X (default to 0 if out of bounds)
        let b = *y.get(i).unwrap_or(&0); // Get the i-th bit of Y (default to 0 if out of bounds)

        let sum = a + b + carry; // Add bits plus carry
        carry = sum / 2; // Update carry (1 if sum >= 2, else 0)
        result.push(sum % 2); // Add the current bit (sum mod 2)
    }

    if carry > 0 {
        result.push(carry); // If carry is 1, add it as the most significant bit
    }

    result.reverse(); // Reverse to match the correct order (most significant bit on the left)
    result
}

// GraphViz command - "dot -Tpng boolean_graph.dot -o boolean_graph.png"
pub fn part_two(input: &str) -> Option<String> {
    let mut graph_lines = vec![
        "digraph BooleanGraph {".to_string(),
        "    rankdir=LR;".to_string(),
    ];

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let line = line.trim();

        if let Some((inputs, output)) = line.split_once("->") {
            let inputs = inputs.trim();
            let output = output.trim();

            let gate_type = match inputs {
                _ if inputs.contains("AND") => "and",
                _ if inputs.contains("XOR") => "xor",
                _ if inputs.contains("OR") => "or",
                _ => continue,
            };

            let parts: Vec<&str> = inputs.split_whitespace().collect();
            if parts.len() >= 3 {
                let input1 = parts[0];
                let input2 = parts[2];
                let gate_node = format!("gate_{}", output);
                graph_lines.push(format!(
                    "    {} [shape=polygon, sides=6, label=\"{}\"];",
                    gate_node,
                    gate_type.to_uppercase()
                ));
                graph_lines.push(format!("    {} -> {};", input1, gate_node));
                graph_lines.push(format!("    {} -> {};", input2, gate_node));

                graph_lines.push(format!(
                    "    {} [shape=ellipse, label=\"{}\"];",
                    output, output
                ));
                graph_lines.push(format!("    {} -> {};", gate_node, output));
            }
        }
    }

    graph_lines.push("}".to_string());

    let mut file = File::create("boolean_graph.dot").expect("Unable to create file");
    for line in graph_lines {
        writeln!(file, "{}", line).expect("Unable to write to file");
    }
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
