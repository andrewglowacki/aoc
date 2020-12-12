use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

trait MovableObject {
    fn rotate(&mut self, amount: i32, neg_const: i32);
    fn change_pos(&mut self, change_x: i32, change_y: i32);
}

struct Ferry {
    x: i32,
    y: i32,
    x_dir: i32,
    y_dir: i32
}

impl Ferry {
    fn new() -> Ferry {
        Ferry {
            x: 0,
            y: 0,
            x_dir: 1,
            y_dir: 0
        }
    }
    fn set_direction(&mut self, x: i32, y: i32) {
        self.x_dir = x;
        self.y_dir = y;
    }
    fn get_manhattan_distance(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

impl MovableObject for Ferry {
    fn rotate(&mut self, amount: i32, neg_const: i32) {
        match amount {
            90 => self.set_direction(neg_const * -self.y_dir, neg_const * self.x_dir),
            180 => self.set_direction(-self.x_dir, -self.y_dir),
            270 => self.set_direction(neg_const * self.y_dir, neg_const * -self.x_dir), 
            _ => panic!("Invalid rotation amount: {}", amount)
        }
    }
    fn change_pos(&mut self, change_x: i32, change_y: i32) {
        self.x += change_x;
        self.y += change_y;
    }
}

struct Waypoint {
    x: i32,
    y: i32
}

impl Waypoint {
    fn new(x: i32, y: i32) -> Waypoint {
        Waypoint { x, y }
    }
    fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

impl MovableObject for Waypoint {
    fn rotate(&mut self, amount: i32, neg_const: i32) {
        match amount {
            90 => self.set_pos(neg_const * -self.y, neg_const * self.x),
            180 => self.set_pos(-self.x, -self.y),
            270 => self.set_pos(neg_const * self.y, neg_const * -self.x),
            _ => panic!("Invalid rotation amount: {}", amount)
        }
    }
    fn change_pos(&mut self, change_x: i32, change_y: i32) {
        self.x += change_x;
        self.y += change_y;
    }
}

fn execute_instruction(object: &mut dyn MovableObject, instruction: char, amount: i32) {
    match instruction {
        'L' => object.rotate(amount, 1),
        'R' => object.rotate(amount, -1),
        'N' => object.change_pos(0, amount),
        'S' => object.change_pos(0, -amount),
        'E' => object.change_pos(amount, 0),
        'W' => object.change_pos(-amount, 0),
        _ => panic!("Invalid instruction code {} with amount {}", instruction, amount)
    }
}

fn get_instructions(file_name: &str) -> Vec<(char, i32)> {
    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| {
            let instruction = line.chars().next().unwrap();
            let amount = line[1..].parse::<i32>().unwrap();
            (instruction, amount)
        })
        .collect::<Vec<(char, i32)>>()
}

fn part_one(file_name: &str) {
    let mut ferry = Ferry::new();
    
    for (instruction, amount) in get_instructions(file_name) {
        if instruction == 'F' {
            ferry.change_pos(ferry.x_dir * amount, ferry.y_dir * amount);
        } else {
            execute_instruction(&mut ferry, instruction, amount);
        }
    }

    println!("For {}, Part 1: Manhattan distance is {}", file_name, ferry.get_manhattan_distance());
}

fn part_two(file_name: &str) {
    let mut ferry = Ferry::new();
    let mut waypoint = Waypoint::new(10, 1);

    for (instruction, amount) in get_instructions(file_name) {
        if instruction == 'F' {
            ferry.change_pos(waypoint.x * amount, waypoint.y * amount);
        } else {
            execute_instruction(&mut waypoint, instruction, amount);
        }
    }
    
    println!("For {}, Part 2: Manhattan distance is {}", file_name, ferry.get_manhattan_distance());
}

fn test_input(file_name: &str) {
    part_one(file_name);
    part_two(file_name);
}

fn main() {
    test_input("sample.txt");
    test_input("input.txt");
}
