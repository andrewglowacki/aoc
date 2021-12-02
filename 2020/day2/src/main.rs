use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).unwrap();
    let parser = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    
    let mut valid_one: i32 = 0;
    let mut valid_two: i32 = 0;

    io::BufReader::new(file)
        .lines()
        .for_each(|line| {
            let line_str = line.unwrap();
            let tokens = parser.captures(line_str.as_str()).unwrap();
            let params = parse_tokens(tokens);
            valid_one += valid_count(is_valid_password_pt1(params));
            valid_two += valid_count(is_valid_password_pt2(params));
        });
    
    println!("Valid Count Part One: {}", valid_one);
    println!("Valid Count Part Two: {}", valid_two);
}

fn valid_count(valid: bool) -> i32 {
    match valid {
        true => 1,
        false => 0
    }
}

fn parse_tokens(tokens: regex::Captures) -> (i32, i32, char, &str) {
    let min = tokens.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let max = tokens.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let letter = tokens.get(3).unwrap().as_str().chars().nth(0).unwrap();
    let password = tokens.get(4).unwrap().as_str();
    (min, max, letter, password)
}

fn is_valid_password_pt1(params: (i32, i32, char, &str)) -> bool {
    let (min, max, letter, password) = params;
    let count = password.chars()
        .filter(|c| *c == letter)
        .count() as i32;
    count >= min && count <= max
}

fn is_valid_password_pt2(params: (i32, i32, char, &str)) -> bool {
    let (first_pos, second_pos, letter, password) = params;
    let chars = password.chars().collect::<Vec<char>>();
    let first = chars[(first_pos as usize) - 1];
    let second = chars[(second_pos as usize) - 1];
    first != second && (first == letter || second == letter)
}