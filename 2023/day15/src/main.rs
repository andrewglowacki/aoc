use std::collections::{HashMap, VecDeque, HashSet};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn hash_step(step: &String) -> usize {
    step.chars().fold(0, |result, c| {
        ((c as usize + result) * 17) % 256
    })
}

enum Operation {
    Add(u32),
    Remove
}

struct Step {
    label: String,
    operation: Operation,
}

impl Step {
    fn parse(line: String) -> Step {
        if let Some(sep) = line.rfind('=') {
            let lens = line[sep + 1..].parse::<u32>().unwrap();
            let label = line[0..sep].to_owned();
            Step { 
                label,
                operation: Operation::Add(lens)
            }
        } else {
            let label = line[0..line.len() - 1].to_owned();;
            Step {
                label,
                operation: Operation::Remove
            }
        }
    }

    fn apply(&self, boxes: &mut Vec<Box>) {
        let index = hash_step(&self.label);
        let lens_box = &mut boxes[index];
        match self.operation {
            Operation::Add(lens) => {
                lens_box.add(&self.label, lens);
            },
            Operation::Remove => {
                lens_box.remove(&self.label);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Box {
    lens: VecDeque<(String, u32)>,
    by_label: HashSet<String>
}

impl Box {
    fn new() -> Box {
        Box { 
            lens: VecDeque::new(),
            by_label: HashSet::new()
        }
    }
    fn remove(&mut self, label: &String) {
        if self.by_label.contains(label) {
            for i in 0..self.lens.len() {
                let (check, _) = &self.lens[i];
                if check == label {
                    self.lens.remove(i);
                    self.by_label.remove(label);
                    break;
                }
            }
        }
    }
    fn add(&mut self, label: &String, lens: u32) {
        if self.by_label.contains(label) {
            self.lens.iter_mut()
                .filter(|(check, _)| check == label)
                .take(1)
                .for_each(|(_, old_lens)| *old_lens = lens);
        } else {
            self.lens.push_back((label.to_owned(), lens));
            self.by_label.insert(label.to_owned());
        }
    }
    fn get_focusing_power(&self, index: u32) -> u32 {
        let box_base = index + 1;
        let mut lens_index = 1;

        self.lens.iter().map(|(_, lens)| {
            let result = lens * lens_index * box_base;
            lens_index += 1;
            result
        })
        .sum::<u32>()
    }
}

fn part_one(file_name: &str) {
    let total = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .flat_map(|line| line.split(',').map(|part| part.to_owned()).collect::<Vec<_>>())
        .map(|line| hash_step(&line))
        .sum::<usize>();
    
    println!("Part 1: {}", total);
}

fn part_two(file_name: &str) {
    let mut boxes = vec![Box::new(); 256];

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .flat_map(|line| line.split(',').map(|part| part.to_owned()).collect::<Vec<_>>())
        .map(|step_str| Step::parse(step_str))
        .for_each(|step| step.apply(&mut boxes));

    let mut index = 0;
    let total = boxes.iter()
        .map(|lens_box| {
            let power = lens_box.get_focusing_power(index);
            index += 1;
            power
        })
        .sum::<u32>();
    
    println!("Part 2: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
