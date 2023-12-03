use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};
use regex::Regex;
use map_macro::hash_map;

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn part_one(file_name: &str) {
    let sum = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| {
            let numbers = line.chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<_>>();
            
            let first = numbers.iter()
                .take(1)
                .last()
                .unwrap();
            
            let last = numbers.last()
                .unwrap();

            let number = first.to_string() + &last.to_string();
            number.parse::<i32>().unwrap()
        })
        .sum::<i32>();
    
    println!("Part 1: {}", sum);
}

fn part_two(file_name: &str) {
    let word_numbers = hash_map! {
        "one"   => "1",
        "two"   => "2",
        "three" => "3",
        "four"  => "4",
        "five"  => "5",
        "six"   => "6",
        "seven" => "7",
        "eight" => "8",
        "nine"  => "9"
    };
    let word_number_regex = "one|two|three|four|five|six|seven|eight|nine";
    let word_number_regex_rev = word_number_regex.chars().rev().collect::<String>();
    let forward = "([1-9]|".to_owned() + word_number_regex + ")";
    let backward = "([1-9]|".to_owned() + word_number_regex_rev.as_str() + ")";
    let number_regex = Regex::new(forward.as_str()).unwrap();
    let number_regex_rev = Regex::new(backward.as_str()).unwrap();
    let sum = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| {
            let line_rev = line.chars().rev().collect::<String>();
            let first = number_regex.find(&line).unwrap().as_str();
            let last = number_regex_rev.find(&line_rev).unwrap().as_str();
            let last = last.chars().rev().collect::<String>();

            let first = match word_numbers.get(&first) {
                Some(x) => *x,
                None => first
            };
            let last = match word_numbers.get(last.as_str()) {
                Some(x) => *x,
                None => last.as_str()
            };
            
            let number = first.to_string() + &last.to_string();
            number.parse::<i32>().unwrap()
        })
        .sum::<i32>();
    
    println!("Part 2: {}", sum);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
