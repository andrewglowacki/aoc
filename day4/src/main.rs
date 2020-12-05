use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

const HAS_ALL: u32 = 127;

type Mappings = HashMap<&'static str, Mapping>;

#[derive(Debug, Clone)]
struct Mapping {
    field: &'static str,
    flag: u32,
    validator: Regex
}

struct FieldValue {
    mapping: Mapping,
    value: String
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).unwrap();

    let mut valid: i32 = 0;
    let mut lines = BufReader::new(file).lines();
    
    let default_mapping: Mapping = Mapping { field: "cid", flag: 0, validator: Regex::new(r".*").unwrap() };

    let mappings: Vec<Mapping> = vec![
        default_mapping,
        Mapping { field: "iyr", flag: 1, validator: Regex::new(r"^20(1[0-9]|20)$").unwrap() },
        Mapping { field: "byr", flag: 2, validator: Regex::new(r"^(19[2-9][0-9]|200[0-2])$").unwrap() },
        Mapping { field: "eyr", flag: 4, validator: Regex::new(r"^20(2[0-9]|30)$").unwrap() },
        Mapping { field: "hgt", flag: 8, validator: Regex::new(r"^(1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in)$").unwrap() },
        Mapping { field: "hcl", flag: 16, validator: Regex::new(r"^#[0-9a-f]{6}$").unwrap() },
        Mapping { field: "ecl", flag: 32, validator: Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap() },
        Mapping { field: "pid", flag: 64, validator: Regex::new(r"^[0-9]{9}$").unwrap() },
    ];

    let token_map: Mappings = mappings.into_iter()
        .map(|mapping| (mapping.field, mapping))
        .collect();

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
            .map(|token| {
                if let Some(mapping) = token_map.get(token[0]) {
                    Some(FieldValue {
                        mapping: mapping.clone(),
                        value: String::from(token[1])
                    })
                } else {
                    None
                }
            })
            .filter(|entry| !entry.is_none())
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.mapping.validator.is_match(entry.value.as_str()))
            .map(|entry| entry.mapping.flag)
            .fold(0, |sum, val| sum | val);
        
    }
    if all == HAS_ALL {
        valid += 1;
    }

    println!("Valid: {}", valid);
}
