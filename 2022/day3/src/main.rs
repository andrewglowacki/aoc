use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn determine_mis_packed(line: String) -> char {
    let mut left = BTreeSet::new();
    let size = line.len() / 2;

    let mut items = line.chars();
    for _ in 0..size {
        left.insert(items.next().unwrap());
    }

    items.find(|item| left.contains(item)).unwrap()
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
        .map(|item| get_priority(item))
        .sum();
    
    println!("Part 1: {}", total);
}

fn determine_badge(one: String, two: String, three: String) -> char {
    let one = one.chars()
        .collect::<BTreeSet<_>>();
    let two = two.chars()
        .collect::<HashSet<_>>();
    let three = three.chars()
        .collect::<HashSet<_>>();

    one.into_iter().find(|item| two.contains(item) && three.contains(item)).unwrap()
}

fn part_two(file_name: &str) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .peekable();
    
    let mut total = 0;

    while let Some(one) = lines.next() {
        let two = lines.next().unwrap();
        let three = lines.next().unwrap();

        let badge = determine_badge(one, two, three);
        total += get_priority(badge);
    }

    println!("Part 2: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
