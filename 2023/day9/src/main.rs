use std::collections::LinkedList;
use std::f32::consts::E;
use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn parse_numbers(file_name: &str) -> Vec<Vec<i64>> {
    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_next_point(numbers: Vec<i64>, end: bool) -> i64 {
    let mut number_stack = Vec::new();
    number_stack.push(LinkedList::from_iter(numbers));
    let mut all_same = false;
    while !all_same {
        let mut differences = LinkedList::new();
        let mut current = number_stack.last().unwrap().iter();
        let mut prev = current.next().unwrap();
        let mut prev_diff = None;
        all_same = true;
        for next in current {
            let diff = next - prev;
            differences.push_back(diff);
            if let Some(prev_diff) = prev_diff {
                if prev_diff != diff {
                    all_same = false;
                }
            }
            prev_diff = Some(diff);
            prev = next;
        }
        number_stack.push(differences);
    }

    if end {
        number_stack.iter()
            .map(|numbers| numbers.back().unwrap())
            .sum::<i64>()
    } else {
        let last_index = number_stack.len() - 1;
        for i in 0..number_stack.len() - 1 {
            let produce = number_stack[last_index - i].front().unwrap();
            let from = number_stack[last_index - (i + 1)].front().unwrap();

            // from - x = produce
            // from - produce = x
            let number = from - produce;
            number_stack[last_index - (i + 1)].push_front(number);
        }

        *number_stack[0].front().unwrap()
    }
}

fn part_one(file_name: &str) {
    let sum_of_next = parse_numbers(file_name).into_iter()
        .map(|numbers| get_next_point(numbers, true))
        .sum::<i64>();
    
    println!("Part 1: {}", sum_of_next);
}

fn part_two(file_name: &str) {
    let sum_of_next = parse_numbers(file_name).into_iter()
        .map(|numbers| get_next_point(numbers, false))
        .sum::<i64>();
    
    println!("Part 2: {}", sum_of_next);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
