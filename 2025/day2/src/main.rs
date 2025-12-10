use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Range {
    start: u64,
    start_str: String,
    end: u64,
    end_str: String,
}

impl Range {
    fn parse(text: &str) -> Self {
        let parts = text.split("-").collect::<Vec<_>>();
        let start_str = parts[0].to_string();
        let end_str = parts[1].to_string();
        let start = start_str.parse::<u64>().unwrap();
        let end = end_str.parse::<u64>().unwrap();
        Range { start, start_str, end, end_str }
    }
    fn count_bad_ids(&self) -> u64 {
        let diff = (self.end - self.start) + 1;
        if self.start_str.len() % 2 == 1 && self.end_str.len() % 2 == 1 {
            return 0;
        }
        let mut total = 0;
        for i in 0..diff {
            let number = self.start + i;
            let str = number.to_string();
            if str.len() % 2 == 1 {
                continue;
            }
            let half_len = str.len() / 2;
            if str[0..half_len] == str[half_len..] {
                println!("Found bad ID: {}", str);
                total += number;
            }
        }
        total
    }
    fn accumulate_bad_ids(&self, bad_ids: &mut HashSet<u64>) {
        let diff = (self.end - self.start) + 1;
        for i in 0..diff {
            let number = self.start + i;
            let str = number.to_string();
            for pieces in 2..str.len() + 1 {
                if str.len() % pieces != 0 {
                    continue;
                }
                let piece_len = str.len() / pieces;
                let piece = &str[0..piece_len];
                let mut matches = true;
                for j in 1..pieces {
                    if str[j * piece_len..(j + 1) * piece_len] != *piece {
                        matches = false;
                        break;
                    }
                }
                if matches {
                    println!("Found bad ID: {}", str);
                    bad_ids.insert(number);
                    break;
                }
            }
        }
    }
}

fn part_one(file_name: &str) {
    let invalid_parts = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .next()
        .unwrap()
        .split(",")
        .map(Range::parse)
        .map(|range| range.count_bad_ids())
        .sum::<u64>();
    
    println!("Part 1: {}", invalid_parts);
}

fn part_two(file_name: &str) {
    let mut bad_ids: HashSet<u64> = HashSet::new();
    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .next()
        .unwrap()
        .split(",")
        .map(Range::parse)
        .for_each(|range| range.accumulate_bad_ids(&mut bad_ids));
    
    println!("Part 2: {}", bad_ids.iter().sum::<u64>());
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
