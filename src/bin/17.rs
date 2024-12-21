advent_of_code::solution!(17);

pub enum State {
    Halt,
    Continue,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Computer {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
    ip: usize,
    output: Vec<String>,
}

impl Computer {
    fn new(input: &str) -> Self {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut program: Vec<u64> = Vec::new();
        input.lines().for_each(|line| {
            if line.starts_with("Register A:") {
                a = line.split(": ").nth(1).unwrap().trim().parse().unwrap();
            } else if line.starts_with("Register B:") {
                b = line.split(": ").nth(1).unwrap().trim().parse().unwrap();
            } else if line.starts_with("Register C:") {
                c = line.split(": ").nth(1).unwrap().trim().parse().unwrap();
            } else if line.starts_with("Program:") {
                program = line.split(": ").nth(1).unwrap()
                    .trim()
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
            }
        });
        Computer {
            a,
            b,
            c,
            program,
            ip: 0,
            output: Vec::new(),
        }
    }

    fn run_program(&mut self) {
        while let State::Continue = self.process() {
            continue;
        }
    }

    fn process(&mut self) -> State {
        if self.ip + 1 >= self.program.len() {
            return State::Halt;
        }
        let opcode = self.program[self.ip];
        let literal_operand = self.program[self.ip + 1];
        let combo_operand = self.get_combo_operand_value(literal_operand);
        match opcode {
            0 => self.adv(combo_operand),
            1 => self.bxl(literal_operand),
            2 => self.bst(combo_operand),
            3 => {
                if self.a != 0 {
                    self.ip = literal_operand as usize;
                    return State::Continue;
                }
            },
            4 => self.bxc(),
            5 => self.out(combo_operand),
            6 => self.bdv(combo_operand),
            7 => self.cdv(combo_operand),
            _ => {},
        }
        self.ip += 2;
        State::Continue
    }

    fn get_combo_operand_value(&self, literal_operand: u64) -> u64 {
        match literal_operand {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => literal_operand
        }
    }

    fn adv(&mut self, combo_operand: u64) {
        let numerator = self.a;
        let denominator = 2u64.pow(combo_operand as u32);
        self.a = numerator / denominator;
    }

    fn bxl(&mut self, literal_operand: u64) {
        self.b = self.b ^ literal_operand;
    }

    fn bst(&mut self, combo_operand: u64) {
        self.b = combo_operand % 8;
    }

    fn bxc(&mut self) {
        self.b = self.b ^ self.c;
    }

    fn out(&mut self, combo_operand: u64) {
        let output = combo_operand % 8;
        self.output.push(output.to_string());
    }

    fn bdv(&mut self, combo_operand: u64) {
        let numerator = self.a;
        let denominator = 2u64.pow(combo_operand as u32);
        self.b = numerator / denominator;
    }

    fn cdv(&mut self, combo_operand: u64) {
        let numerator = self.a;
        let denominator = 2u64.pow(combo_operand as u32);
        if denominator == 0 {
            println!("here");
        }
        self.c = numerator / denominator;
    }

    fn get_output(&self) -> String {
        self.output.join(",")
    }

    fn reverse_engineer(&mut self) -> u64 {
        let mut next = vec![0];

        for output_value in self.program.clone().iter().rev() {
            next = self.get_potential_previous_values(next, output_value);
        }

        next[0]
    }

    fn get_potential_previous_values(&mut self, next: Vec<u64>, output_value: &u64) -> Vec<u64> {
        next.iter().flat_map(|&a_next| {
            (0..8).filter_map(|k| {
                if a_next == 0 && k == 0 {
                    return None;
                }
                let a_curr = (a_next * 8) + k;
                let mut temp_computer = self.clone();
                temp_computer.a = a_curr;

                while temp_computer.output.is_empty() {
                    temp_computer.process();
                }

                if temp_computer.output[0] == output_value.to_string() {
                    Some(a_curr)
                } else {
                    None
                }
            })
            .collect::<Vec<u64>>()
        }).collect()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::new(input);
    computer.run_program();
    Some(computer.get_output())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut computer = Computer::new(input);
    Some(computer.reverse_engineer())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
