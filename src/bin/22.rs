use std::collections::HashMap;
use rayon::prelude::*;

advent_of_code::solution!(22);

#[derive(Debug, Eq, PartialEq, Clone)]
struct Buyer {
    secret_number: u64,
}

impl Buyer {
    fn new(initial_secret: &str) -> Self {
        Buyer {
            secret_number: initial_secret.parse::<u64>().unwrap(),
        }
    }

    fn generate(&mut self) -> u64 {
        for _ in 0..2000 {
            self.evolve();
        }
        self.secret_number
    }

    fn evolve(&mut self) {
        self.mix(self.secret_number << 6);
        self.prune();

        self.mix(self.secret_number >> 5);
        self.prune();

        self.mix(self.secret_number << 11);
        self.prune();
    }

    fn mix(&mut self, number: u64) {
        self.secret_number = self.secret_number ^ number;
    }

    fn prune(&mut self) {
        self.secret_number = self.secret_number % 16777216;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let result: u64 = input
            .lines()
            .par_bridge()
            .map(|initial_secret| {
                let mut buyer = Buyer::new(initial_secret);
                buyer.generate()
            })
            .sum();
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
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
