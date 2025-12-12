use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

struct Map {
    grid: HashSet<(i32,i32)>
}

impl Map {
    fn new(grid: HashSet<(i32,i32)>) -> Self {
        Map { grid }
    }
    fn count_accessible_rolls(&self) -> u32 {
        for y in 0i32..self.grid.len() as i32 {
            for x in 0i32..self.grid[0].len() as i32 {
                if self.grid[y as usize][x as usize] {
                    // Found an '@' character
                }
            }
        }
        0
    }
}

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn part_one(file_name: &str) {
    let mut y: i32 = 0;
    let grid = Map::new(get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .flat_map(|line| {
            let result = line.char_indices()
                .flat_map(|(x, c)| match c == '@' {
                    true => Some((x as i32, y)),
                    false => None
                }).collect::<Vec<_>>();
            y += 1;
            result
        })
        .collect::<HashSet<_>>());
    
    println!("Part 1: {}", grid.count_accessible_rolls());
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("sample.txt");
    part_two("sample.txt");

    println!("Done!");
}
