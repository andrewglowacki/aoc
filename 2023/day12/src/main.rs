use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use concurrent_queue::ConcurrentQueue;

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Clone, Debug)]
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
    
    fn unfold(&mut self) {
        let orig_len = self.springs.len();
        for _ in 0..4 {
            self.springs.push('?');
            for i in 0..orig_len {
                self.springs.push(self.springs[i]);
            }
        }

        let orig_len = self.counts.len();
        for _ in 0..4 {
            for i in 0..orig_len {
                self.counts.push(self.counts[i]);
            }
        }
        
    }

    fn count_options(&self, start: usize, count_index: usize) -> usize {
        let count = self.counts[count_index];

        let right_space_required: usize = 
            self.counts[count_index + 1..].iter().sum::<usize>() + 
            (self.counts.len() - (count_index + 1));

        let mut options = 0;

        for i in start..self.springs.len() - right_space_required {
            if i + count > self.springs.len() {
                break;
            }

            if self.springs[start..i].iter()
                .any(|c| *c == '#') 
            {
                break;
            }
            
            let has_space = self.springs[i..i + count].iter()
                .any(|c| *c == '.') == false;
            if !has_space {
                continue;
            }

            if count_index != self.counts.len() - 1 {
                if i + count >= self.springs.len() {
                    break;
                }
                let sep = self.springs[i + count];

                if sep == '#' {
                    continue;
                }
            }

            let start_next = i + count + 1;
            
            if count_index + 1 < self.counts.len() {
                options += self.count_options(start_next, count_index + 1);
            } else {
                let has_damaged = self.springs[i + count..].iter()
                    .any(|c| *c == '#');
                if !has_damaged {
                    options += 1;
                }
            }
        }

        options
    }

}

fn part_one(file_name: &str) {
    let mut index = 0;
    let total = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Report::parse(line))
        .map(|report| {
            index += 1;
            let count = report.count_options(0, 0);
            println!("#{} = {}", index, count);
            count
        })
        .sum::<usize>();
    
    println!("Part 1: {}", total);
}

fn part_two(file_name: &str) {
    let reports = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Report::parse(line))
        .collect::<Vec<_>>();

    let thread_count = 10;
    let queue = ConcurrentQueue::<Report>::bounded(reports.len());
    
    reports.into_iter().for_each(|mut report| {
        report.unfold();
        queue.push(report).unwrap();
    });

    queue.close();

    let queue_rc = Arc::new(queue);

    let mut handles = Vec::new();

    for _ in 0..thread_count {
        let copy = Arc::clone(&queue_rc);
        handles.push(thread::spawn(move || {
            let mut total = 0;
            while let Ok(report) = copy.pop() {
                total += report.count_options(0, 0) as u64;
            }
            total
        }));
    }

    let copy = Arc::clone(&queue_rc);

    let mut last = 0;
    while copy.len() > 0 {
        let count = copy.len();
        if count != last {
            println!("Left: {}", count);
            last = count;
        }
        thread::sleep(Duration::from_secs(1));
    }

    let total = handles.into_iter()
        .map(|handle| handle.join().unwrap())
        .sum::<u64>();
    
    println!("Part 2: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
