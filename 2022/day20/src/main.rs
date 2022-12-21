use std::fs::File;
use std::mem::swap;
use std::num;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Number {
    orig_pos: usize,
    value: i32
}

impl Number {
    fn new(orig_pos: usize, value: i32) -> Number {
        Number {
            orig_pos,
            value
        }
    }
}

struct Numbers {
    numbers: Vec<Number>
}

impl Numbers {
    fn from_file(file_name: &str) -> Numbers {
        let mut numbers = Vec::new();

        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| line.parse::<i32>().unwrap())
            .for_each(|number| numbers.push(Number::new(numbers.len(), number)));

        Numbers { numbers }
    }

    fn find_orig_index(&self, index: usize) -> usize {
        for j in 0..self.numbers.len() {
            if (self.numbers[j].orig_pos == index) {
                return j;
            }
        }
        panic!("Index not found: {}", index);
    }

    fn decrypt(&mut self) {
        let count = self.numbers.len();

        for i in 0..count {
            let cur_pos = self.find_orig_index(i);
            let value = self.numbers[cur_pos].value;
            let mut new_pos = cur_pos as i32 + value;
            if new_pos == 0 {
                new_pos = count as i32 - 1;
            } if new_pos < 0 {
                new_pos = -1 * new_pos;
                new_pos = new_pos % count as i32;
                new_pos = count as i32 - new_pos - 1;
                if new_pos == 0 {
                    new_pos = (count as i32) - 1;
                }
            } else {
                if new_pos >= count as i32 {
                    new_pos = (new_pos % count as i32) + 1;
                }
            }
            let new_pos = new_pos as usize;

            if new_pos > cur_pos {
                for j in cur_pos..new_pos {
                    self.numbers.swap(j, j + 1);
                }
            } else if new_pos < cur_pos {
                let len = cur_pos - new_pos;
                for j in 0..len {
                    let j = cur_pos - j;
                    self.numbers.swap(j, j - 1);
                }
            }
        }
    }

    fn sum_grove_coordinates(&self) -> i32 {
        let mut zero_pos = -1;
        for i in 0..self.numbers.len() {
            if self.numbers[i].value == 0 {
                zero_pos = i as i32;
                break;
            }
        }
        let zero_pos = zero_pos as usize;
        self.numbers[(zero_pos + 1000) % self.numbers.len()].value +
        self.numbers[(zero_pos + 2000) % self.numbers.len()].value + 
        self.numbers[(zero_pos + 3000) % self.numbers.len()].value
    }

}

fn part_one(file_name: &str) {
    let mut numbers = Numbers::from_file(file_name);
    
    numbers.decrypt();

    let sum = numbers.sum_grove_coordinates();
    
    println!("Part 1: {}", sum);
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
