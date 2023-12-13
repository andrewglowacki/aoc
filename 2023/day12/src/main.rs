use core::num;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};
use regex::Regex;

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

enum Group {
    Damaged(usize, usize, usize),
    Unknown(usize)
}

struct Report {
    counts: Vec<usize>,
    groups: Vec<Group>
}

impl Report {

    fn parse_groups(group_str: &str) -> Vec<Group> {
        let mut groups = Vec::new();
        let mut group_chars = group_str.chars().peekable();
        
        while let Some(c) = group_chars.next() {
            match c {
                '.' => (),
                '?' => {
                    let mut damaged = 0;
                    let mut unknown_left = 0;
                    while let Some(c) = group_chars.next() {
                        match c {
                            '#' => {
                                damaged += 1;
                                break;
                            },
                            '?' => unknown_left += 1,
                            '.' => break,
                            _ => panic!("{}", c)
                        }
                    }
                    if damaged == 0 {
                        groups.push(Group::Unknown(unknown_left));
                        continue;
                    }

                    let mut unknown_right = 0;
                    while let Some(c) = group_chars.next() {
                        match c {
                            '#' => damaged += 1,
                            '?' => {
                                unknown_right += 1;
                                break;
                            },
                            '.' => break,
                            _ => panic!("{}", c)
                        }
                    }

                    if unknown_right > 0 {
                        while let Some(c) = group_chars.peek() {
                            if *c == '?' {
                                unknown_right += 1;
                            } else {
                                break;
                            }
                            group_chars.next();
                        }
                    }

                    groups.push(Group::Damaged(unknown_left, damaged, unknown_right));
                }, 
                '#' => {
                    let mut damaged = 0;
                    let mut unknown = 0;
                    while let Some(c) = group_chars.next() {
                        match c {
                            '#' => damaged += 1,
                            '?' => {
                                unknown += 1;
                                break;
                            },
                            '.' => break,
                            _ => panic!("{}", c)
                        }
                    }
                    if unknown > 0 {
                        while let Some(c) = group_chars.peek() {
                            if *c == '?' {
                                unknown += 1;
                            } else {
                                break;
                            }
                            group_chars.next();
                        }
                    }
                    groups.push(Group::Damaged(0, damaged, unknown));
                },
                _ => panic!("Unexpected character: {}", c)
            }
        }

        groups
    }

    fn parse(line: String) -> Report {
        let mut parts = line.split(' ');

        let groups = Report::parse_groups(parts.next().unwrap());
        
        let counts = parts.next()
            .unwrap()
            .split(',')
            .map(|number| number.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Report { counts, groups }
    }

    fn count_damaged_options(&self) -> usize {
        let damaged_expected = self.counts.iter().sum::<usize>();
        let damaged_known = self.groups.iter()
            .filter_map(|group| match group {
                Group::Damaged(_, damaged, _) => Some(damaged),
                _ => None
            })
            .sum::<usize>();
        let damaged_unknown = damaged_expected - damaged_known;
        
        let mut counts_sorted = (0..self.counts.len())
            .into_iter()
            .map(|i| (self.counts[i], i))
            .collect::<Vec<_>>();
        counts_sorted.sort();

        while let Some((count, index)) = counts_sorted.pop() {
            
        }

        0
    }
}

fn part_one(file_name: &str) {
    let total = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Report::parse(line))
        .map(|report| report.count_damaged_options())
        .sum::<usize>();
    
    println!("Part 1: {}", total);
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
