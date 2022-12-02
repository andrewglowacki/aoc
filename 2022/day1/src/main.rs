use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};
use std::collections::BTreeSet;

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn part_one(file_name: &str) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    let mut total = 0;
    let mut max = 0;

    while let Some(line) = lines.next() {
        if line.is_empty() {
            max = total.max(max);
            total = 0;
        } else {
            total += line.parse::<u32>().unwrap();
        }
    }
    
    if total > 0 {
        max = total.max(max);
    }
    
    println!("Part 1: {}", max);
}

fn part_two(file_name: &str) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
        
    let mut all = BTreeSet::<u32>::new();
    let mut total: u32 = 0;

    while let Some(line) = lines.next() {
        if line.is_empty() {
            all.insert(total);
            total = 0;
        } else {
            total += line.parse::<u32>().unwrap();
        }
    }
    
    if total > 0 {
        all.insert(total);
    }

    let mut end = all.iter().rev();
    let result = end.next().unwrap() + end.next().unwrap() + end.next().unwrap();
    
    println!("Part 2: {}", result);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
