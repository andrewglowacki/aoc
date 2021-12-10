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

fn find_syntax_error(line: &String) -> Option<char> {
    let mut expected_ends = vec![];

    for c in line.chars() {
        match c {
            '(' => expected_ends.push(')'),
            '[' => expected_ends.push(']'),
            '{' => expected_ends.push('}'),
            '<' => expected_ends.push('>'),
            _ => {
                let expected = expected_ends.pop().unwrap();
                if expected != c {
                    return Some(c);
                }
            }
        }
    }

    None
}

fn get_incomplete_score(line: &String) -> Option<u64> {
    let mut expected_ends = vec![];

    for c in line.chars() {
        match c {
            '(' => expected_ends.push(')'),
            '[' => expected_ends.push(']'),
            '{' => expected_ends.push('}'),
            '<' => expected_ends.push('>'),
            _ => {
                let expected = expected_ends.pop().unwrap();
                if expected != c {
                    return None;
                }
            }
        }
    }

    let mut score = 0;
    expected_ends.reverse();
    for c in &expected_ends {
        let add = match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Unexpected end: {}", c)
        };
        score = (score * 5) + add;
    }
    Some(score)
}

fn part_one(file_name: &str) {
    let mut score_by_expected = HashMap::<char, u32>::new();
    score_by_expected.insert(')', 3);
    score_by_expected.insert(']', 57);
    score_by_expected.insert('}', 1197);
    score_by_expected.insert('>', 25137);
    let score_by_expected = score_by_expected;

    let score = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .flat_map(|line| find_syntax_error(&line))
        .flat_map(|expected| score_by_expected.get(&expected))
        .sum::<u32>();
    
    println!("Part 1: {}", score);
}

fn part_two(file_name: &str) {
    let mut scores = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .flat_map(|line| get_incomplete_score(&line))
        .collect::<Vec<_>>();
    
    scores.sort();
    let mid = scores.len() >> 1;
    let score = scores[mid];
    
    println!("Part 2: {}", score);
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");
    part_two("input.txt");

    println!("Done!");
}
