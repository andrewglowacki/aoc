use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::HashSet;
use std::ops::Range;

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn parse_valid_numbers(fields: &Vec<Field>) -> HashSet<u32> {
    fields.iter() 
        .flat_map(|field| &field.ranges)
        .flat_map(|range| range.clone().collect::<Vec<u32>>())
        .collect::<HashSet<u32>>()
}

#[derive(Debug)]
struct Field {
    name: String,
    ranges: Vec<Range<u32>>
}

impl Field {
    fn is_valid(&self, number: u32) -> bool {
        self.ranges.iter()
            .find(|range| range.contains(&number))
            .is_some()
    }
}

fn parse_valid_fields(lines: &mut Lines<BufReader<File>>) -> Vec<Field> {
    lines.flat_map(|line| line.ok())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let name = parts[0].to_owned();
            let ranges = parts[1].split(" or ")
                .map(|range| {
                    range.split("-")
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .map(|range| range[0]..(range[1] + 1))
                .collect::<Vec<Range<u32>>>();
            Field { name, ranges }
        })
        .collect::<Vec<Field>>()
}

fn parse_ticket(line: String) -> Vec<u32> {
    line.split(",")
        .flat_map(|number_str| number_str.parse::<u32>())
        .collect::<Vec<u32>>()
}

fn test_part_one(file_name: &str) {
    let mut lines = get_file_lines(file_name);

    let valid_fields = parse_valid_fields(&mut lines);
    let valid_numbers = parse_valid_numbers(&valid_fields);

    // skip to nearby tickets
    (&mut lines).flat_map(|line| line.ok())
        .take_while(|line| line.trim() != "nearby tickets:")
        .for_each(|_| ());

    let error_rate = lines.flat_map(|line| line.ok())
        .flat_map(|line| parse_ticket(line))
        .filter(|number| !valid_numbers.contains(number))
        .sum::<u32>();
    
    println!("For {}, error rate is: {}", file_name, error_rate);
}

fn test_part_two(file_name: &str) {
    let mut lines = get_file_lines(file_name);

    let fields = parse_valid_fields(&mut lines);
    let valid_numbers = parse_valid_numbers(&fields);

    lines.next(); // skip 'your ticket:' line

    // parse my ticket
    let my_ticket = parse_ticket(lines.next().unwrap().unwrap());
    
    let mut candidates = (0..fields.len())
        .map(|_| {
            fields.iter()
                .map(|field| field.name.clone())
                .collect::<HashSet<String>>()
        })
        .collect::<Vec<HashSet<String>>>();

    lines.skip(2)
        .flat_map(|line| line.ok())
        .map(|line| parse_ticket(line))
        .filter(|ticket| {
            ticket.iter()
                .find(|number| !valid_numbers.contains(number))
                .is_none()
        })
        .for_each(|ticket| {
            for i in 0..ticket.len() {
                let number = ticket[i];
                for f in 0..fields.len() {
                    let field = &fields[f];
                    if !field.is_valid(number) {
                        let candidate = &mut candidates[i];
                        candidate.remove(&field.name);
                    }
                }
            }
        });

    // keep remove fields until there are none left
    loop {
        let mut removed = false;
        // find all fields we know for sure
        let knowns = (0..fields.len())
            .filter(|i| candidates[*i].len() == 1)
            .map(|i| {
                let field = candidates[i].iter().last().unwrap();
                (field.clone(), i)
            })
            .collect::<Vec<(String, usize)>>();

        // remove all knowns from the other candidates
        for (field, index) in knowns {
            for i in 0..candidates.len() {
                if i != index {
                    let candidate = &mut candidates[i];
                    if candidate.remove(&field) {
                        removed = true;
                    }
                }
            }
        }
        if !removed {
            break;
        }
    }

    let mut result: u64 = 1;
    for i in 0..candidates.len() {
        let candidate = &candidates[i];
        if candidate.len() != 1 {
            panic!("Position {} does not have one candidate: {:?}", i, candidate);
        }
        let field = candidate.iter().last().unwrap();
        if field.starts_with("departure") {
            result *= my_ticket[i] as u64;
        }
    }

    println!("For {}, product is: {}", file_name, result);
}

fn test_input(file_name: &str) {
    test_part_one(file_name);
    test_part_two(file_name);
}

fn main() {
    test_input("sample.txt");
    test_input("next_sample.txt");
    test_input("input.txt");
}
