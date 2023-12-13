use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Report {
    counts: Vec<usize>,
    springs: Vec<char>
}

impl Report {

    fn parse(line: String) -> Report {
        let mut parts = line.split(' ');

        let springs = parts.next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>();

        let counts = parts.next()
            .unwrap()
            .split(',')
            .map(|number| number.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Report { counts, springs }
    }

    fn count_options(&self, start: usize, count_index: usize) -> usize {
        let count = self.counts[count_index];

        let right_space_required: usize = 
            self.counts[count_index + 1..].iter().sum::<usize>() + 
            (self.counts.len() - (count_index + 1));

        let mut options = 0;

        // println!("counting options with start {} count_index: {}, count: {}", start, count_index, count);
        // println!("right space required: {}", right_space_required);

        for i in start..self.springs.len() - right_space_required {
            if i + count > self.springs.len() {
                break;
            }

            if count_index == 0 {
                if self.springs[0..i].iter()
                    .any(|c| *c == '#') 
                {
                    break;
                }
            }
            
            // println!("checking start {} for count_index {}", i, count_index);

            let has_space = self.springs[i..i + count].iter()
                .any(|c| *c == '.') == false;
            if !has_space {
                // not possible
                // println!("not possible starting at {} - no space", i);
                continue;
            }

            if count_index != self.counts.len() - 1 {
                if i + count >= self.springs.len() {
                    break;
                }
                let sep = self.springs[i + count];

                if sep == '#' {
                    // not possible
                    // println!("not possible starting at {} - sep is #", i);
                    continue;
                }
            }

            let start_next = i + count + 1;
            
            if count_index + 1 < self.counts.len() {
                // println!("starting next count index at {}", start_next);
                options += self.count_options(start_next, count_index + 1);
                // println!("done with next count index at {}", start_next);
            } else {
                let has_damaged = self.springs[i + count..].iter()
                    .any(|c| *c == '#');
                if !has_damaged {
                    options += 1;
                //     println!("option found");
                // } else {
                    // println!("not possible at end - damages are left");
                }
            }
        }

        // println!("returning options {} for count_index {}", options, count_index);
        options
    }

}

fn part_one(file_name: &str) {
    let total = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Report::parse(line))
        .map(|report| {
            // println!("testing {:?} - {:?}", report.springs, report.counts);
            let count = report.count_options(0, 0);
            // println!("count for {:?} - {:?} is {}", report.springs, report.counts, count);
            count
        })
        .sum::<usize>();
    
    println!("Part 1: {}", total);
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
