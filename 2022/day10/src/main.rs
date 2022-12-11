use std::collections::LinkedList;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Display {
    cycle: i64,
    register: i64,
    add_pipeline: LinkedList<i64>
}

impl Display {
    fn new() -> Display {
        Display {
            cycle: 1,
            register: 1,
            add_pipeline: LinkedList::new()
        }
    }

    fn cycle(&mut self) -> bool {
        if let Some(amount) = self.add_pipeline.pop_front() {
            self.register += amount;
            self.cycle += 1;
            true
        } else {
            false
        }
    }

    fn process(&mut self, command: String) {
        self.add_pipeline.push_back(0);
        match &command[0..4] {
            "addx" => {
                let amount = command[5..].parse::<i64>().unwrap();
                self.add_pipeline.push_back(amount);
            },
            "noop" => (),
            _ => panic!("Invalid operation: {}", command)
        }
    }
}

fn part_one(file_name: &str) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());

    let mut display = Display::new();
    let mut strength_sum = 0;
    let mut next_report = 20;

    while let Some(command) = lines.next() {
        display.process(command);
        
        while display.cycle() {
            if display.cycle == next_report {
                strength_sum += display.cycle * display.register;
                next_report += 40;
                if next_report > 220 {
                    break;
                }
            }
        }
    }
    
    println!("Part 1: {}", strength_sum);
}

fn part_two(file_name: &str) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());

    let mut display = Display::new();

    println!("Part 2:");
    print!("#");

    while let Some(command) = lines.next() {
        display.process(command);

        while display.cycle() {
            let sprite = display.register;
            let position = (display.cycle - 1) % 40;
            if position >= sprite - 1 && position <= sprite + 1 {
                print!("#");
            } else {
                print!(".");
            }
            if position == 39 {
                println!("");
            }
        }
    }
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
