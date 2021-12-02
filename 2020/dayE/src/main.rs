use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::HashMap;
use regex::Regex;

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Mask {
    zeros: u64,
    ones: u64,
    permuted_zero_mask: u64,
    permuted: Vec<u64>
}

impl Mask {
    fn new(zeros: u64, ones: u64, permute: bool) -> Mask {
        let unstable_bits = zeros ^ ones;

        let mut permuted = Vec::new();
        if permute {
            let permute_digits = (0..36)
                .filter(|i| unstable_bits & (1 << *i as u64) > 0)
                .collect::<Vec<usize>>();
            
            Mask::create_permutations(&mut permuted, ones, 0, &permute_digits);
        }

        Mask {
            zeros,
            ones,
            permuted: permuted,
            permuted_zero_mask: !unstable_bits
        }
    }
    fn create_permutations(permuted: &mut Vec<u64>, base_mask: u64, index: usize, digits: &Vec<usize>) {
        if index >= digits.len() {
            return;
        }

        let mut permuted = permuted;
        permuted.push(base_mask);
        Mask::create_permutations(&mut permuted, base_mask, index + 1, digits);

        let digit = digits[index];
        let with_one = base_mask | (1 << digit);
        permuted.push(with_one);
        Mask::create_permutations(&mut permuted, with_one, index + 1, digits);
    }
    fn apply(&self, number: u64) -> u64 {
        (number & self.zeros) | self.ones
    }
    fn permute(&self, address: u64) -> Vec<u64> {
        let base_address = self.permuted_zero_mask & address;
        self.permuted.iter()
            .map(|mask| base_address | mask)
            .collect::<Vec<u64>>()
    }
}

fn parse_mask(mask: &str, permute: bool) -> Mask {
    let mut zeros = 0;
    let mut ones = 0;
    let digits = mask.chars().collect::<Vec<char>>();
    for i in 0..digits.len() {
        zeros = zeros << 1;
        ones = ones << 1;
        match digits[i] {
            '0' => (),
            '1' => {
                zeros = zeros | 1;
                ones = ones | 1;
            },
            'X' => {
                zeros = zeros | 1;
            },
            _ => panic!("Unexpected character in mask at pos {}: {}", i, mask)
        };
    }
    Mask::new(zeros, ones, permute)
}

fn set_memory(
    memory: &mut HashMap::<u64, u64>, 
    permuted_memory: &mut HashMap::<u64, u64>, 
    mask: &mut Mask, 
    mem_offset_str: &str, 
    number_str: &str) 
{
    let number_orig = number_str.parse::<u64>().unwrap();
    let number = mask.apply(number_orig);

    let mem_offset = mem_offset_str.parse::<u64>().unwrap();
    memory.insert(mem_offset, number);

    for address in mask.permute(mem_offset) {
        permuted_memory.insert(address, number_orig);
    }
}

fn test_input(file_name: &str, permute: bool) {
    let mut memory = HashMap::<u64, u64>::new();
    let mut permuted_memory = HashMap::<u64, u64>::new();
    let mut mask = Mask::new(0, 0, false);
    let mut lines = get_file_lines(file_name);

    let parser = Regex::new(r"^(?:(mask)|mem\[([0-9]+)\]) = (.+)$").unwrap();

    while let Some(Ok(line)) = lines.next() {
        let tokens = parser.captures(line.as_str()).unwrap()
            .iter()
            .flat_map(|capture| capture)
            .map(|capture| capture.as_str())
            .collect::<Vec<&str>>();

        let instruction = tokens[1];
        let value = tokens[2];
        match instruction {
            "mask" => mask = parse_mask(value, permute),
            _ => set_memory(&mut memory, &mut permuted_memory, &mut mask, instruction, value)
        }
    }

    let value_sum = memory.values().sum::<u64>();
    println!("For {}, sum of values in memory: {}", file_name, value_sum);
    
    if permute {
        let value_sum = permuted_memory.values().sum::<u64>();
        println!("For {}, sum of values in permuted memory: {}", file_name, value_sum);
    }
}

fn main() {
    test_input("normal_sample.txt", false);
    test_input("permute_sample.txt", true);
    test_input("input.txt", true);
}
