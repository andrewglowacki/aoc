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

enum Direction {
    Left,
    Right,
    Up,
    Down
}

struct Instruction {
    vector: (i32, i32)
}

impl Instruction {
    fn parse(line: String, hex: bool) -> Instruction {
        let pieces = line.split_ascii_whitespace()
            .collect::<Vec<_>>();

        if hex {
            let hex = pieces[2];
            let hex = hex[2..hex.len() - 1];

        } else {
            let vector = match pieces[0] {
                "L" => (-1,  0),
                "R" => ( 1,  0),
                "U" => ( 0, -1),
                "D" => ( 0,  1),
                _ => panic!("Unexpected direction: {}", pieces[0])
            };

            let amount = pieces[1].parse::<i32>().unwrap();
            let vector = (vector.0 * amount, vector.1 * amount);

            Instruction  vector }
        }
    }
    fn dig(&self, from: (i32, i32), trench: &mut Trench) -> (i32, i32) {
        let (from_x, from_y) = from;
        let (vector_x, vector_y) = self.vector;
        let x = from_x + vector_x;
        let y = from_y + vector_y;

        if vector_x == 0 {
            let from = from_y.min(y);
            let to = from_y.max(y) + 1;
            (from..to).for_each(|y| { 
                trench.add(x, y); 
            });
        } else {
            let from = from_x.min(x);
            let to = from_x.max(x) + 1;
            (from..to).for_each(|x| { 
                trench.add(x, y); 
            });
        }

        (x, y)
    }
}

struct Trench {
    border: HashSet<(i32, i32)>,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32
}

impl Trench {
    fn new() -> Trench {
        Trench {
            border: HashSet::new(),
            left: i32::MAX,
            top: i32::MAX,
            right: i32::MIN,
            bottom: i32::MIN
        }
    }

    fn add(&mut self, x: i32, y: i32) {
        self.border.insert((x, y));
        self.left = self.left.min(x);
        self.right = self.right.max(x);
        self.top = self.top.min(y);
        self.bottom = self.bottom.max(y);
    }

    fn _print(&self) {
        for y in self.top..self.bottom + 1 {
            for x in self.left..self.right + 1 {
                if self.border.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
    
    fn get_excavation_size(&self) -> u32 {
        let mut inside = false;
        let mut prev_border = false;
        let mut inside_points = HashSet::<(i32, i32)>::new();
        for y in self.top..self.bottom + 1 {
            for x in self.left..self.right + 1 {
                if self.border.contains(&(x, y)) {
                    if !prev_border {
                        inside = !inside;
                        prev_border = true;
                    }
                    // print!("#");
                } else {
                    if prev_border {
                        if inside {
                            if !inside_points.contains(&(x, y - 1)) && !self.border.contains(&(x, y - 1)) {
                                inside = false;
                            }
                        } else {
                            if inside_points.contains(&(x, y - 1)) {
                                inside = true;
                            }
                        }
                    }
                    prev_border = false;
                    if inside {
                        // print!("X");
                        inside_points.insert((x, y));
                    // } else {
                        // print!(".");
                    }
                }
            }
            inside = false;
            prev_border = false;
            // println!("");
        }
        inside_points.len() as u32 + self.border.len() as u32
    }
}

struct DigPlan {
    instructions: Vec<Instruction>
}

impl DigPlan {
    fn parse(file_name: &str, hex: bool) -> DigPlan {
        let instructions = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| Instruction::parse(line, hex))
            .collect::<Vec<_>>();
        DigPlan { instructions }
    }
    fn dig_trench(&self) -> Trench {
        let mut trench = Trench::new();
        let mut current = (0, 0);

        self.instructions.iter().for_each(|instruction| {
            current = instruction.dig(current, &mut trench);
        });

        trench
    }
}

fn part_one(file_name: &str) {
    let plan = DigPlan::parse(file_name, false);
    let trench = plan.dig_trench();
    // trench._print();
    let hole_size = trench.get_excavation_size();
    println!("Part 1: {}", hole_size);
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
