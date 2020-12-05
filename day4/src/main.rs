use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

const HAS_ALL: i32 = 127;

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).unwrap();

    let mut valid: i32 = 0;
    let mut lines = BufReader::new(file).lines();
    
    let mapping = vec![
        ("cid", 0),
        ("iyr", 1),
        ("byr", 2),
        ("eyr", 4),
        ("hgt", 8),
        ("hcl", 16),
        ("ecl", 32),
        ("pid", 64)
    ];

    let token_map: HashMap<String, i32> = mapping.into_iter()
        .map(|entry| (String::from(entry.0), entry.1))
        .collect();

    let valid_mapping: Vec<(i32, Regex)> = vec![
        (0, Regex::new(r".*").unwrap()), // cid/none
        (1, Regex::new(r"^20(1[0-9]|20)$").unwrap()), // iyr
        (2, Regex::new(r"^(19[2-9][0-9]|200[0-2])$").unwrap()), // byr
        (4, Regex::new(r"^20(2[0-9]|30)$").unwrap()), // eyr
        (8, Regex::new(r"^(1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in)$").unwrap()), // hgt
        (16, Regex::new(r"^#[0-9a-f]{6}$").unwrap()), // hcl
        (32, Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap()), // ecl
        (64, Regex::new(r"^[0-9]{9}$").unwrap()), // pid
    ];
    
    let valid_checks: HashMap<i32, Regex> = valid_mapping.into_iter().collect();

    let mut all = 0;
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            if all == HAS_ALL {
                valid += 1;
            }
            all = 0;
            continue;
        }

        all |= line.split(char::is_whitespace)
            .map(|token| token.split(':').collect::<Vec<&str>>() )
            .map(|token| (token_map.get(token[0]).unwrap_or(&0), token[1]))
            .filter(|entry| valid_checks.get(entry.0).unwrap().is_match(entry.1))
            .map(|entry| entry.0)
            .fold(0, |sum, val| sum | val);
        
    }
    if all == HAS_ALL {
        valid += 1;
    }

    println!("Valid: {}", valid);
}
