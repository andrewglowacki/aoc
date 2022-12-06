use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};
use std::str::Chars;

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct CodeSet {
    codes: Vec<u32>,
    index: usize,
    size: usize
}

impl CodeSet {
    fn new(size: usize) -> CodeSet {
        CodeSet {
            codes: vec![0; size],
            index: 0,
            size
        }
    }

    fn to_number(code: char) -> u32 {
        1 << (code as u32 - 'a' as u32)
    }

    fn is_start(&mut self, code: char) -> bool {
        let code = Self::to_number(code);
        self.codes[self.index % self.size] = code;
        
        let mut mask = 0;
        for i in 0..self.size {
            let check = self.codes[i];
            let new_mask = mask | check;
            if new_mask == mask {
                self.index += 1;
                return false;
            }
            mask = new_mask
        }
        true
    }

    fn find_start(&mut self, mut message: Chars) -> usize {
        while let Some(code) = message.next() {
            if self.is_start(code) {
                return self.index + 1;
            }
        }
        panic!("No start found!");
    }
}

fn part_one(file_name: &str) {
    let message = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .next()
        .unwrap();
    
    let message = message.chars();
    let mut code_set = CodeSet::new(4);

    println!("Part 1: {}", code_set.find_start(message));
}

fn part_two(file_name: &str) {
    let message = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .next()
        .unwrap();
        
    let message = message.chars();
    let mut code_set = CodeSet::new(14);
    
    println!("Part 1: {}", code_set.find_start(message));
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
