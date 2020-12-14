extern crate regex;

use regex::Regex;
use std::collections::HashMap;

pub fn part1(inp: String) {
    let commands = read_commands(&inp);

    let sum = compute_sum(&commands);
    println!("Sum: {}", sum);
}

pub fn part2(_inp: String) {}
fn compute_sum(commands: &Vec<Command>) -> u64 {
    let mut memory = Memory::new();
    for command in commands {
        match command {
            Command::SetMask(mask) => {
                // println!("set_mask({})", &mask);
                memory.set_mask(&mask);
            }
            Command::SetValue(set_value) => {
                // println!("set_value({}, {})",set_value.address, set_value.value);
                memory.set_value(set_value.address, set_value.value);
            }
        }
    }

    // println!("{:?}", memory.get_all_values());

    let sum: u64 = memory.get_all_values().values().sum();
    sum
}

#[derive(Debug)]
struct SetValue {
    address: usize,
    value: u64,
}

#[derive(Debug)]
enum Command {
    SetMask(String),
    SetValue(SetValue),
}

fn read_commands(inp: &str) -> Vec<Command> {
    inp.split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| parse_command(line))
        .collect::<Vec<_>>()
}

fn parse_command(line: &str) -> Command {
    if line.starts_with("mask = ") {
        let mask_value_str = line.split("mask = ").skip(1).next().unwrap();
        return Command::SetMask(mask_value_str.to_string());
    } else {
        lazy_static! {
            static ref RE: Regex = Regex::new("mem\\[([0-9]+)\\] = ([0-9]+)").unwrap();
        }

        let captures = RE.captures(line).unwrap();

        let address = match captures[1].parse::<usize>() {
            Ok(address) => address,
            Err(err) => panic!(
                "Cannot parse address \"{}\": {}",
                captures[1].to_string(),
                err
            ),
        };

        let value = captures[2].parse::<u64>().unwrap();
        return Command::SetValue(SetValue { address, value });
    }
}

struct Memory {
    mask_force_one: u64,  // if mask bit is 1, force value bit to 1 (bitwise OR)
    mask_force_zero: u64, // if mask bit is 0, force value bit to 0 (bitwise AND)
    memory: HashMap<usize, u64>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mask_force_one: 0,
            mask_force_zero: std::u64::MAX,
            memory: HashMap::new(),
        }
    }

    // TODO: Can we accept the mask as [char; 36] here?
    pub fn set_mask(&mut self, mask: &str) {
        let mask = mask
            .chars()
            .map(|c| match c {
                'X' => None,
                '1' => Some(1),
                '0' => Some(0),
                _ => panic!("argh"),
            })
            .collect::<Vec<Option<u8>>>();

        // Reset masks
        self.mask_force_one = 0;
        self.mask_force_zero = std::u64::MAX;

        // println!("{:#066b}", self.mask_force_one);
        // println!("{:#066b}", self.mask_force_zero);

        for (index, value) in mask.iter().rev().enumerate() {
            match value {
                Some(value) => {
                    if *value == 1 {
                        self.mask_force_one |= 1 << index;
                    } else {
                        self.mask_force_zero &= !(1 << index);
                    }
                }
                None => (),
            }
        }

        // println!("{:#066b}", self.mask_force_one);
        // println!("{:#066b}", self.mask_force_zero);
    }

    pub fn set_value(&mut self, address: usize, value: u64) {
        let mut masked_value = value;
        masked_value |= self.mask_force_one;
        masked_value &= self.mask_force_zero;
        if masked_value == 0 {
            self.memory.remove(&address);
        } else {
            self.memory.insert(address, masked_value);
        }
    }

    // TODO: How to add a lifetime specificier here?
    pub fn get_all_values(&self) -> HashMap<usize, u64> {
        self.memory.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_value_with_mask() {
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let address = 8;
        let value = 11;

        let mut memory = Memory::new();
        memory.set_mask(mask);
        memory.set_value(address, value);

        let all_values = memory.get_all_values();
        assert_eq!(all_values.len(), 1);
        assert_eq!(all_values[&address], 73);
    }

    #[test]
    fn set_multiple_values() {
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";

        let mut memory = Memory::new();
        memory.set_mask(mask);
        memory.set_value(8, 11);
        memory.set_value(7, 101);
        memory.set_value(8, 0);

        let all_values = memory.get_all_values();
        assert_eq!(all_values.len(), 2);
        assert_eq!(all_values[&7], 101);
        assert_eq!(all_values[&8], 64);
    }

    #[test]
    fn compute_sum_for_sample() {
        let program =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";

        let commands = read_commands(&program);
        // println!("{:?}", commands);
        let sum = compute_sum(&commands);

        assert_eq!(sum, 165);
    }
}
