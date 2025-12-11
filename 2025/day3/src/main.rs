use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

struct Bank {
    batteries: Vec<u32>
}

impl Bank {
    fn parse(line: String) -> Self {
        let batteries = line.chars()
            .map(|c| c.to_digit(10).unwrap() as u32)
            .collect::<Vec<_>>();
        Bank { batteries }
    }
    fn get_largest_joltage(self, size: usize) -> u64 {
        let mut result = String::with_capacity(size);
        let mut max = 0;
        let mut start = 0;
        for c in 0..size {
            for i in start..self.batteries.len() - (size - (c + 1)) {
                if self.batteries[i] > max {
                    max = self.batteries[i];
                    start = i + 1;
                }
            }
            result += max.to_string().as_str();
            max = 0;
        }
        result.parse::<u64>().unwrap()
    }
}

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn part_one(file_name: &str) {
    let output = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(Bank::parse)
        .map(|bank| bank.get_largest_joltage(2))
        .sum::<u64>();
    
    println!("Part 1: {}", output);
}

fn part_two(file_name: &str) {
    let output = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(Bank::parse)
        .map(|bank| bank.get_largest_joltage(12))
        .sum::<u64>();
    
    println!("Part 2: {}", output);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
