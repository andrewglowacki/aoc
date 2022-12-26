use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn parse_snafu(line: String) -> i64 {
    let mut number = 0;
    let mut power = 1;
    line.chars().rev().for_each(|c| {
        match c {
            '0' => (),
            '1' => number += power,
            '2' => number += power * 2,
            '=' => number -= power * 2,
            '-' => number -= power,
            _ => panic!("Invalid character")
        }
        power *= 5;
    });
    number
}

fn to_snafu(mut number: i64) -> String {
    let mut result = Vec::new();
    let mut power = 1;
    while number > 0 {
        let next_power = power * 5;
        if (number + (power * 2)) % next_power == 0 {
            result.push('=');
            number += power * 2;
        } else if (number + power) % next_power == 0 {
            result.push('-');
            number += power;
        } else if (number - power) % next_power == 0 {
            result.push('1');
            number -= power;
        } else if (number - (power * 2)) % next_power == 0 {
            result.push('2');
            number -= power * 2;
        } else {
            result.push('0');
        }
        power = next_power;
    }

    result.iter().rev().collect::<String>()
}

fn part_one(file_name: &str) {
    let sum = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| parse_snafu(line))
        .sum::<i64>();
    
    println!("Part 1: {} / {}", sum, to_snafu(sum));
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
