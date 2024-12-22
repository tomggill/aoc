use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Debug, Eq, PartialEq, Clone)]
struct TowelSpec {
    pattern_map: HashMap<char, Vec<String>>,
    designs: Vec<String>,
}

impl TowelSpec {
    fn new(input: &str) -> Self {
        let (patterns_string, designs_string) = input.split_once("\n\n").unwrap();
        let mut pattern_map: HashMap<char, Vec<String>> = HashMap::new();
        for pattern in patterns_string.split(", ").map(|p| p.to_string()) {
            pattern_map.entry(pattern.chars().next().unwrap()).or_default().push(pattern);
        }
        let designs = designs_string.lines().map(|line| line.to_string()).collect();
        TowelSpec {
            pattern_map,
            designs,
        }
    }

    fn can_construct(&self, design: &str, cache: &mut HashMap<String, bool>) -> bool {
        if design.is_empty() {
            return true;
        }
        if let Some(&cached) = cache.get(design) {
            return cached;
        }
        if let Some(patterns) = self.pattern_map.get(&design.chars().next().unwrap()) {
            for pattern in patterns {
                if design.starts_with(pattern) {
                    let remaining_design = &design[pattern.len()..];
                    if self.can_construct(remaining_design, cache) {
                        cache.insert(design.to_string(), true);
                        return true;
                    }
                }
            }
        }
        cache.insert(design.to_string(), false);
        false
    }

    fn get_all_combinations(&self, design: &str, cache: &mut HashMap<String, u64>) -> u64 {
        if design.is_empty() {
            return 1;
        }
        if let Some(&cached) = cache.get(design) {
            return cached;
        }
        let mut combinations = 0;
        if let Some(patterns) = self.pattern_map.get(&design.chars().next().unwrap()) {
            for pattern in patterns {
                if design.starts_with(pattern) {
                    let remaining_design = &design[pattern.len()..];
                    combinations += self.get_all_combinations(remaining_design, cache);
                }
            }
        }
        cache.insert(design.to_string(), combinations);
        combinations
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let towel_spec = TowelSpec::new(input);
    let mut cache = HashMap::new();
    let result = towel_spec.designs.iter().filter(|design| towel_spec.can_construct(design, &mut cache)).count();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let towel_spec = TowelSpec::new(input);
    let mut cache = HashMap::new();
    let result = towel_spec.designs.iter().map(|design| towel_spec.get_all_combinations(design, &mut cache)).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
