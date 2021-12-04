use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Line {
    numbers: HashSet<u64>
}

impl Line {
    fn new() -> Line {
        Line {
            numbers: HashSet::new()
        }
    }
    fn add(&mut self, number: u64) {
        self.numbers.insert(number);
    }
    fn called(&mut self, number: u64) -> bool {
        self.numbers.remove(&number);
        self.numbers.len() == 0
    }
}

fn parse_input(file_name: &str) -> (Vec<u64>, Vec<Line>, HashMap::<u64, Vec<usize>>) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .peekable();
    
    let called_numbers = lines.next()
        .unwrap()
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect::<Vec<u64>>();
    
    // skip next blank line
    lines.next();

    let mut all_lines = Vec::<Line>::new();
    
    // parse each row and column in the boards as 'Line' structs
    while lines.peek().is_some() {
        let mut columns = Vec::<Line>::with_capacity(5);
        for _ in 0..5 {
            columns.push(Line::new());
        }
        for _ in 0..5 {
            let mut row = Line::new();

            lines.next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|number| number.parse::<u64>().unwrap())
                .for_each(|number| {
                    let c = row.numbers.len();
                    columns[c].add(number);
                    row.add(number);
                });
            
            all_lines.push(row);
        }
        for column in columns {
            all_lines.push(column);
        }
        lines.next(); // skip blank line
    }

    let mut number_to_lines = HashMap::<u64, Vec<usize>>::new();
    
    // index lines by the numbers that are within them
    for i in 0..all_lines.len() {
        let line = &all_lines[i];
        for number in &line.numbers {
            if let Some(lines_for_number) = number_to_lines.get_mut(&number) {
                lines_for_number.push(i);
            } else {
                number_to_lines.insert(*number, vec![i]);
            }
        }
    }

    (called_numbers, all_lines, number_to_lines)
}

fn part_one(file_name: &str) {

    let (
        called_numbers, 
        mut all_lines, 
        number_to_lines
    ) = parse_input(file_name);

    // do number calling until we have a fully covered line
    let mut result = 0;
    for number in called_numbers {
        if let Some(for_number) = number_to_lines.get(&number) {
            let covered = for_number.iter().find(|index| {
                all_lines.get_mut(**index)
                    .unwrap()
                    .called(number)
            });
            if let Some(covered_index) = covered {
                let board_start = covered_index - (covered_index % 10);
                let board_end = board_start + 5;

                let uncovered_sum = (board_start..board_end)
                    .flat_map(|i| all_lines.get(i))
                    .flat_map(|line| line.numbers.iter())
                    .sum::<u64>();

                result = uncovered_sum * number;
                break;
            }
        }
    }
    
    println!("Part 1: {}", result);
}

fn part_two(file_name: &str) {
    let (
        called_numbers, 
        mut all_lines, 
        number_to_lines
    ) = parse_input(file_name);

    
    // do number calling until we find the last covered board
    let mut boards_covered = HashSet::<usize>::new();
    let mut last_result = 0;

    for number in called_numbers {
        if let Some(for_number) = number_to_lines.get(&number) {
            let newly_covered = for_number.iter()
                .filter(|index| {
                    all_lines.get_mut(**index)
                        .unwrap()
                        .called(number)
                })
                .map(|index| *index - (*index % 10))
                .filter(|board_index| !boards_covered.contains(board_index))
                .collect::<Vec<usize>>();
            
            for board_start in newly_covered {
                let board_end = board_start + 5;

                let uncovered_sum = (board_start..board_end)
                    .flat_map(|i| all_lines.get(i))
                    .flat_map(|line| line.numbers.iter())
                    .sum::<u64>();

                last_result = uncovered_sum * number;
                boards_covered.insert(board_start);
            }
        }
    }
    
    println!("Part 2: {}", last_result);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
