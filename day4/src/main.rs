use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::HashMap;
use std::iter::Peekable;
use regex::Regex;

const HAS_ALL: u32 = 127;

type Mappings = HashMap<&'static str, Mapping>;

#[derive(Debug, Clone)]
struct Mapping {
    field: &'static str,
    flag: u32,
    validator: Regex
}

impl Mapping {
    fn new(field: &'static str, flag: u32, regex: &str) -> Mapping {
        Mapping {
            field,
            flag, 
            validator: Regex::new(regex).unwrap()
        }
    }
}

struct FieldValue {
    mapping: Mapping,
    value: String
}

fn split_copy(line: String, split_char: char) -> Vec<String> {
    line.split(split_char)
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}

fn is_next_passport_valid(token_map: &Mappings, lines: &mut Peekable<Lines<BufReader<File>>>) -> bool {
    lines.filter_map(|line| line.ok())
        .take_while(|line| !line.is_empty())
        .flat_map(|line| split_copy(line, ' '))
        .map(|token| split_copy(token, ' '))
        .filter_map(|token| {
            if let Some(mapping) = token_map.get(token[0].as_str()) {
                Some(FieldValue {
                    mapping: mapping.clone(),
                    value: String::from(token[1].as_str())
                })
            } else {
                None
            }
        })
        .filter(|entry| entry.mapping.validator.is_match(entry.value.as_str()))
        .map(|entry| entry.mapping.flag)
        .fold(0, |sum, val| sum | val) == HAS_ALL
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).unwrap();

    let mut valid: i32 = 0;
    let mut lines = BufReader::new(file).lines().peekable();
    
    let mappings: Vec<Mapping> = vec![
        Mapping::new("cid", 0, r".*"),
        Mapping::new("iyr", 1, r"^20(1[0-9]|20)$"),
        Mapping::new("byr", 2, r"^(19[2-9][0-9]|200[0-2])$"),
        Mapping::new("eyr", 4, r"^20(2[0-9]|30)$"),
        Mapping::new("hgt", 8, r"^(1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in)$"),
        Mapping::new("hcl", 16, r"^#[0-9a-f]{6}$"),
        Mapping::new("ecl", 32, r"^(amb|blu|brn|gry|grn|hzl|oth)$"),
        Mapping::new("pid", 64, r"^[0-9]{9}$"),
    ];

    let token_map: Mappings = mappings.into_iter()
        .map(|mapping| (mapping.field, mapping))
        .collect();

    while lines.peek().is_some() {
        if is_next_passport_valid(&token_map, &mut lines) {
            valid += 1;
        }
    }

    println!("Valid: {}", valid);
}
