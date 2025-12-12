use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Machine {
    target: usize,
    button_map: Vec<HashMap<usize, usize>>,
    joltages: Vec<usize>
}

impl Machine {
    fn parse(line: String) -> Self {
        let parts = line.split_ascii_whitespace()
            .collect::<Vec<_>>();
        let len = parts[0].len() - 3;
        let mut max = 0;
        for _ in 0..len + 1 {
            max = (max << 1) | 1
        }
        let target = Machine::parse_target(&parts[0][1..parts[0].len()-1]);

        let button_map = parts[1..parts.len() - 1].iter()
            .map(|part| Machine::parse_button(&part[1..part.len()-1], max, len))
            .collect::<Vec<_>>();

        let joltage_part = parts[parts.len() - 1];
        let joltages = joltage_part[1..joltage_part.len() - 1].split(",")
            .map(|str| str.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        
        Machine { target, button_map, joltages }
    }
    
    fn smallest_presses_to_target(&self) -> usize {
        let mut results = HashMap::<usize, usize>::new();
        let mut from = HashSet::new();
        from.insert(self.target);
        let mut pushes = 0;
        while !results.contains_key(&0) {
            let new_from = from.iter().flat_map(|source| 
                self.button_map.iter()
                    .flat_map(|map| map.get(&source))
                    .filter(|dest| !results.contains_key(dest))
                    .collect::<Vec<_>>()
            )
            .cloned()
            .collect::<HashSet<_>>();

            pushes += 1;
            new_from.iter().for_each(|num| { 
                results.insert(*num, pushes); 
            });
            from = new_from;
        }
        pushes
    }
    
    fn parse_button(part: &str, max: usize, len: usize) -> HashMap<usize, usize> {
        let mask = part.split(",")
            .map(|str| str.parse::<usize>().unwrap())
            .map(|num| 1 << (len - num))
            .fold(0, |acc, item| acc | item);

        let map = (0..max + 1).into_iter()
            .map(|orig| (orig ^ mask, orig))
            .collect::<HashMap<_, _>>();

        map
    }

    fn parse_target(part: &str) -> usize {
        let len = part.len() - 1;
        part.char_indices()
            .map(|(i, c)| match c {
                '#' => 1 << (len - i),
                _ => 0
            })
            .fold(0, |acc, item| acc | item)
    }
}

fn part_one(file_name: &str) {
    let min_presses = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(Machine::parse)
        .map(|machine| machine.smallest_presses_to_target())
        .sum::<usize>();
    
    println!("Part 1: {}", min_presses);
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");

    println!("Done!");
}
