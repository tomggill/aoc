use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u64> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    input.lines().for_each(|line| {
        let (n1, n2) = line.split_once('-').unwrap();
        graph.entry(n1.to_string()).or_insert_with(Vec::new).push(n2.to_string());
        graph.entry(n2.to_string()).or_insert_with(Vec::new).push(n1.to_string());
    });

    let result: u64 = graph.iter().map(|(node, neighbours)| {
        let mut triplet_count = 0;
        let num_neighbours = neighbours.len();
        for i in 0..num_neighbours {
            for j in i+1..num_neighbours {
                let n1 = &neighbours[i];
                let n2 = &neighbours[j];
                if !n1.starts_with('t') && !n2.starts_with('t') && !node.starts_with('t') {
                    continue;
                }
                if graph.get(n1).unwrap().contains(n2) {
                    triplet_count += 1;
                }
            }
        }
        triplet_count
    }).sum();

    Some(result / 3)
}

// See here: https://www.geeksforgeeks.org/maximal-clique-problem-recursive-solution/
fn bron_kerbosch(
    current_clique: HashSet<String>,
    potential_nodes: HashSet<String>,
    graph: &HashMap<String, Vec<String>>,
    max_clique: &mut HashSet<String>,
) {
    if potential_nodes.is_empty() {
        if current_clique.len() > max_clique.len() {
            *max_clique = current_clique;
        }
        return;
    }

    let mut next_potential_nodes = potential_nodes.clone();
    for node in potential_nodes.iter() {
        let mut next_clique = current_clique.clone();
        next_clique.insert(node.clone());

        let neighbors = graph
            .get(node)
            .unwrap()
            .iter()
            .cloned()
            .collect();

        bron_kerbosch(
            next_clique,
            next_potential_nodes.intersection(&neighbors).cloned().collect(),
            graph,
            max_clique,
        );
        next_potential_nodes.remove(node);
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let mut graph = HashMap::new();
    let mut node_set = HashSet::new();
    input.lines().for_each(|line| {
        let (n1, n2) = line.split_once('-').unwrap();
        graph.entry(n1.to_string()).or_insert_with(Vec::new).push(n2.to_string());
        graph.entry(n2.to_string()).or_insert_with(Vec::new).push(n1.to_string());
        node_set.insert(n1.to_string());
        node_set.insert(n2.to_string());
    });

    let mut max_clique = HashSet::new();
    bron_kerbosch(
        HashSet::new(),
        node_set,
        &graph,
        &mut max_clique,
    );
    let mut max_clique = max_clique.into_iter().collect::<Vec<String>>();
    max_clique.sort();
    Some(max_clique.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
