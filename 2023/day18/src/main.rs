use std::collections::{HashSet, BTreeSet, HashMap};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Instruction {
    vector: (i128, i128)
}

impl Instruction {
    fn parse(line: String, hex: bool) -> Instruction {
        let pieces = line.split_ascii_whitespace()
            .collect::<Vec<_>>();

        if hex {
            let hex = pieces[2];
            let hex = &hex[2..hex.len() - 1];

            let vector = match &hex[hex.len() - 1..] {
                "2" => (-1,  0), // left
                "0" => ( 1,  0), // right
                "3" => ( 0, -1), // up
                "1" => ( 0,  1), // down
                _ => panic!("Unexpected direction: {}", pieces[0])
            };

            let amount = i128::from_str_radix(&hex[0..hex.len() - 1], 16).unwrap();
            let vector = (vector.0 * amount, vector.1 * amount);

            Instruction { vector }
        } else {
            let vector = match pieces[0] {
                "L" => (-1,  0),
                "R" => ( 1,  0),
                "U" => ( 0, -1),
                "D" => ( 0,  1),
                _ => panic!("Unexpected direction: {}", pieces[0])
            };

            let amount = pieces[1].parse::<i128>().unwrap();
            let vector = (vector.0 * amount, vector.1 * amount);

            Instruction { vector }
        }
    }
    fn dig(&self, from: (i128, i128), trench: &mut Trench) -> (i128, i128) {
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
    border: HashSet<(i128, i128)>,
    left: i128,
    right: i128,
    top: i128,
    bottom: i128
}

impl Trench {
    fn new() -> Trench {
        Trench {
            border: HashSet::new(),
            left: i128::MAX,
            top: i128::MAX,
            right: i128::MIN,
            bottom: i128::MIN
        }
    }

    fn add(&mut self, x: i128, y: i128) {
        self.border.insert((x, y));
        self.left = self.left.min(x);
        self.right = self.right.max(x);
        self.top = self.top.min(y);
        self.bottom = self.bottom.max(y);
    }

    fn get_excavation_size(&self) -> i128 {
        let mut inside = false;

        let mut edges = HashMap::<i128, BTreeSet<i128>>::new();

        for (x, y) in self.border.iter() {
            if let Some(edges) = edges.get_mut(y) {
                edges.insert(*x);
            } else {
                let mut x_edges = BTreeSet::new();
                x_edges.insert(*x);
                edges.insert(*y, x_edges);
            }
        }

        let mut inside_count = 0;
        // let mut inside_points = HashSet::<(i128, i128)>::new();
        
        for y in self.top..self.bottom + 1 {

            let current = &edges.get(&y).unwrap();

            let mut next = 0;
            let mut prev_inside_start = 0;
            while let Some(x) = current.range(next..).next() {
                if current.contains(&(x + 1)) {
                    let mut last_x = *x;
                    for x in current.range(x + 1..) {
                        if *x == last_x + 1 {
                            last_x = *x;
                        } else {
                            break;
                        }
                    }

                    let above_start = self.border.contains(&(*x, y - 1));
                    let above_end = self.border.contains(&(last_x, y - 1));

                    if inside {
                        inside_count += (x - prev_inside_start) - 1;
                        // for x in prev_inside_start + 1..*x {
                        //     inside_points.insert((x, y));
                        // }
                    }
                    prev_inside_start = last_x;

                    if above_end != above_start {
                        inside = !inside;
                    }

                    next = last_x + 1;
                    
                } else {
                    if inside {
                        inside_count += (x - prev_inside_start) - 1;
                        // for x in prev_inside_start + 1..*x {
                        //     inside_points.insert((x, y));
                        // }
                    }
                    prev_inside_start = *x;
                    inside = !inside;
                    next = x + 1;
                }
            }

            inside = false;
        }

        // for y in self.top..self.bottom + 1 {
        //     for x in self.left..self.right + 1 {
        //         let point = (x, y);
        //         if inside_points.contains(&point) {
        //             print!("O");
        //         } else if self.border.contains(&point) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!("");
        // }

        inside_count + self.border.len() as i128
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
    let hole_size = trench.get_excavation_size();
    println!("Part 1: {}", hole_size);
}

fn part_two(file_name: &str) {
    let plan = DigPlan::parse(file_name, true);
    let trench = plan.dig_trench();
    let hole_size = trench.get_excavation_size();
    println!("Part 2: {}", hole_size);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
