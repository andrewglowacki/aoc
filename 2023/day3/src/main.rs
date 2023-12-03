use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

struct Schematic {
    symbols: HashMap<(i32, i32), String>,
    parts: HashMap<(i32, i32), String>,
    gears_to_parts: HashMap<(i32, i32), Vec<(i32, i32)>>,
    height: i32
}

impl Schematic {
    fn new() -> Schematic {
        Schematic {
            symbols: HashMap::new(),
            parts: HashMap::new(),
            gears_to_parts: HashMap::new(),
            height: 0
        }
    }
    fn add_gear(&self, gears_to_parts: &mut HashMap<(i32, i32), Vec<(i32, i32)>>, x: i32, y: i32, part: &(i32, i32)) {
        if let Some(symbol) = self.symbols.get(&(x, y)) {
            if symbol == "*" {
                if let Some(parts) = gears_to_parts.get_mut(&(x, y)) {
                    parts.push(*part);
                } else {
                    gears_to_parts.insert((x, y), vec![*part]);
                }
            }
        }
    }
    fn populate_gears(&mut self) {
        let mut gears_to_parts = HashMap::new();

        self.parts.iter().for_each(|(coords, part)| {
            let (x, y) = coords;
            let end = x + part.len() as i32;
            for x in x - 1..end + 1 {
                self.add_gear(&mut gears_to_parts, x, y - 1, coords);
                self.add_gear(&mut gears_to_parts, x, y + 1, coords);
            }
            self.add_gear(&mut gears_to_parts, x - 1, *y, coords);
            self.add_gear(&mut gears_to_parts, end, *y, coords);
        });

        gears_to_parts.retain(|_, parts| parts.len() == 2);

        self.gears_to_parts = gears_to_parts;
    }
    fn prune_false_parts(&mut self) {
        self.parts = self.parts.iter().filter(|((x, y), part)| {
            let end = x + part.len() as i32;
            for x in x - 1..end + 1 {
                if self.symbols.contains_key(&(x, y - 1)) ||
                    self.symbols.contains_key(&(x, y + 1)) 
                {
                    return true
                }
            }
            self.symbols.contains_key(&(x - 1, *y)) ||
            self.symbols.contains_key(&(end, *y))
        })
        .map(|(coord, part)| (*coord, part.to_owned()))
        .collect::<HashMap<_,_>>();
    }
    fn parse(file_name: &str) -> Schematic {
        let mut schematic = Schematic::new();

        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .for_each(|line| schematic.parse_line(line));
        
        schematic.prune_false_parts();
        schematic.populate_gears();

        schematic
    }
    fn parse_line(&mut self, line: String) {
        let mut remaining = line.as_str();

        let mut x = 0;
        while let Some(start) = remaining.find(|c| c != '.') {
            let new_remaining = &remaining[start..];
            let is_part = new_remaining.chars()
                .nth(0)
                .unwrap()
                .is_digit(10);
            let end_condition = match is_part {
                true => |c: char| !c.is_digit(10),
                false => |c: char| c == '.' || c.is_digit(10)
            };
            let end = match new_remaining.find(end_condition) {
                Some(end) => end,
                None => new_remaining.len()
            };

            let coords = (x + start as i32, self.height);

            let piece = &new_remaining[..end];
            if is_part {
                self.parts.insert(coords, piece.to_owned());
            } else {
                self.symbols.insert(coords, piece.to_owned());
            }

            x += (start + end) as i32;
            remaining = &new_remaining[end..];
        }

        self.height += 1;
    }
    fn sum_parts(&self) -> u32 {
        self.parts.values()
            .map(|part| part.parse::<u32>().unwrap())
            .sum()
    }
    fn sum_gear_ratios(&self) -> u32 {
        self.gears_to_parts.values()
            .map(|parts| {
                parts.iter().map(|(x, y)| {
                    self.parts.get(&(*x, *y))
                        .unwrap()
                        .parse::<u32>()
                        .unwrap()
                })
                .reduce(|a, b| a * b)
                .unwrap()
            })
            .sum()
    }
    
}

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn part_one(file_name: &str) {
    let schematic = Schematic::parse(file_name);
    println!("Part 1: {}", schematic.sum_parts());
}

fn part_two(file_name: &str) {
    let schematic = Schematic::parse(file_name);
    println!("Part 2: {}", schematic.sum_gear_ratios());
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
