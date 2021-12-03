use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn part_one(file_name: &str) {
    let pattern_len = 12;
    let mut counts = Vec::<i32>::with_capacity(pattern_len);
    for _ in 0..pattern_len {
        counts.push(0);
    }

    let mut total = 0;

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .flat_map(|line| line.char_indices().collect::<Vec<_>>())
        .for_each(|(i, c)| {
            if c == '1' {
                counts[i] += 1;
            }
            if i == 0 {
                total += 1;
            }
        });
    
    let mut gamma: u64 = 0;
    let half = total / 2;
    let mut mask = 0;
    for i in 0..pattern_len {
        let num = counts[i];
        gamma = gamma << 1;
        if num > half {
            gamma = gamma | 1;
        }
        mask = (mask << 1) | 0x1;
    }

    let delta = (gamma ^ mask) & mask;
    
    println!("Part 1: {} * {} = {}", gamma, delta, gamma * delta);
}

struct Node {
    zero: Box<Option<Node>>,
    one: Box<Option<Node>>,
    count: u64
}

impl Node {
    fn new() -> Node {
        Node {
            zero: Box::new(None),
            one: Box::new(None),
            count: 0
        }
    }

    fn add(&mut self, line: Vec<char>, pos: usize) {
        if pos < line.len() {
            match line[pos] {
                '0' => {
                    if self.zero.is_none() {
                        self.zero = Box::new(Some(Node::new()))
                    }
                    let zero = &mut *self.zero;
                    zero.as_mut().unwrap().add(line, pos + 1);
                },
                '1' => {
                    if self.one.is_none() {
                        self.one = Box::new(Some(Node::new()))
                    }
                    let one = &mut *self.one;
                    one.as_mut().unwrap().add(line, pos + 1);
                }
                _ => panic!("Invalid character in {:?} at {}", line, pos)
            };
        }
        self.count += 1;
    }

    fn get_zero(&self) -> &Node {
        (*self.zero).as_ref().unwrap()
    }

    fn get_one(&self) -> &Node {
        (*self.one).as_ref().unwrap()
    }

    fn get_count(side: &Option<Node>) -> u64 {
        side.as_ref().map_or_else(|| 0, |node| node.count)
    }

    fn get_remaining(&self, thus_far: u64) -> u64 {
        match (self.zero.is_some(), self.one.is_some()) { 
            (true, _) => {
                self.get_zero().get_remaining(thus_far << 1)
            },
            (_, true) => {
                self.get_one().get_remaining((thus_far << 1) | 1)
            },
            _ => thus_far
        }
    }

    fn get_oxygen(&self, thus_far: u64) -> u64 {
        let zero_count = Node::get_count(&self.zero);
        let one_count = Node::get_count(&self.one);

        // println!("thus_far: {:b} zeros: {} ones: {}", thus_far, zero_count, one_count);

        match zero_count + one_count {
            0 => return thus_far,
            1 => return self.get_remaining(thus_far),
            _ => ()
        };

        let thus_far = thus_far << 1;

        if one_count >= zero_count {
            return self.get_one().get_oxygen(thus_far | 1);
        } else {
            return self.get_zero().get_oxygen(thus_far);
        }
    }

    fn get_co2(&self, thus_far: u64) -> u64 {
        let zero_count = Node::get_count(&self.zero);
        let one_count = Node::get_count(&self.one);

        // println!("thus_far: {:b} zeros: {} ones: {}", thus_far, zero_count, one_count);

        match zero_count + one_count {
            0 => return thus_far,
            1 => return self.get_remaining(thus_far),
            _ => ()
        };

        let thus_far = thus_far << 1;

        if zero_count <= one_count {
            return self.get_zero().get_co2(thus_far);
        } else {
            return self.get_one().get_co2(thus_far | 1);
        }
    }
}

fn part_two(file_name: &str) {
    let mut root = Node::new();

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| line.chars().collect::<Vec<char>>())
        .for_each(|line| root.add(line, 0));
    
    let oxygen = root.get_oxygen(0);
    let co2 = root.get_co2(0);

    println!("Part 2: {} * {} = {}", oxygen, co2, oxygen * co2);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
