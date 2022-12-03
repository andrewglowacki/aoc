use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn to_number(line: String) -> u64 {
    line.chars()
        .map(|item| 1 << (get_priority(item) - 1))
        .fold(0, |result, item| result | item)
}

fn determine_mis_packed(line: String) -> u32 {
    let size = line.len() / 2;

    let left = line[0..size].to_owned();
    let right = line[size..].to_owned();

    let left = to_number(left);
    let right = to_number(right);

    let result = left & right;
    result.trailing_zeros() + 1
}

fn get_priority(item: char) -> u32 {
    match item.is_lowercase() {
        true => (item as u32 - 'a' as u32) + 1,
        false => (item as u32 - 'A' as u32) + 27
    }
}

fn part_one(file_name: &str) {
    let total: u32 = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| determine_mis_packed(line))
        .sum();
    
    println!("Part 1: {}", total);
}

fn determine_badge(one: String, two: String, three: String) -> u32 {
    let one = to_number(one);
    let two = to_number(two);
    let three = to_number(three);

    let result = one & two & three;
    result.trailing_zeros() + 1
}

fn part_two(file_name: &str) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    let mut total = 0;

    while let Some(one) = lines.next() {
        let two = lines.next().unwrap();
        let three = lines.next().unwrap();

        total += determine_badge(one, two, three);
    }

    println!("Part 2: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
