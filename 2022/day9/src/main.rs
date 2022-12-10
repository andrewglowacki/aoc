use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new() -> Point {
        Point::coords(0, 0)
    }
    fn coords(x: i32, y: i32) -> Point {
        Point {
            x,
            y
        }
    }

    fn translate(&mut self, x_amount: i32, y_amount: i32){
        self.x += x_amount;
        self.y += y_amount;
    }
    
}

struct RopeBridge {
    points: Vec<Point>,
    visited: HashSet<(i32, i32)>
}

impl RopeBridge {
    fn new(size: usize) -> RopeBridge {
        let mut visited = HashSet::new();

        visited.insert((0, 0));

        let mut points = Vec::new();
        
        for _ in 0..size {
            points.push(Point::new());
        }

        RopeBridge {
            points,
            visited
        }
    }

    fn perform_move(&mut self, instruction: &str) {
        let direction = instruction.chars().next().unwrap();
        let mut amount = instruction[2..].parse::<i32>().unwrap();

        let (x_amount, y_amount) = match direction {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("Invalid move direction in: {}", instruction)
        };
        
        while amount > 0 {
            
            self.points[0].translate(x_amount, y_amount);

            for i in 1..self.points.len() {
                let x_diff = self.points[i - 1].x - self.points[i].x;
                let y_diff = self.points[i - 1].y - self.points[i].y;
                
                let mut this_x_amount = match x_diff {
                    2 => 1,
                    -2 => -1,
                    _ => 0
                };
                let mut this_y_amount = match y_diff {
                    2 => 1,
                    -2 => -1,
                    _ => 0
                };

                if this_x_amount == 0 && x_diff != 0 && this_y_amount != 0 {
                    this_x_amount = x_diff;
                } else if this_y_amount == 0 && y_diff != 0 && this_x_amount != 0 {
                    this_y_amount = y_diff;
                }

                self.points[i].translate(this_x_amount, this_y_amount);
            }

            amount -= 1;
            let last = &self.points[self.points.len() - 1];
            self.visited.insert((last.x, last.y));
        }
        
    }
}

fn part_one(file_name: &str) {
    let mut bridge = RopeBridge::new(2);

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .for_each(|line| bridge.perform_move(line.as_str()));

    println!("Part 1: {}", bridge.visited.len());
}

fn part_two(file_name: &str) {
    let mut bridge = RopeBridge::new(10);

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .for_each(|line| bridge.perform_move(line.as_str()));
    
    println!("Part 2: {}", bridge.visited.len());
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}