use std::collections::LinkedList;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

enum Operand {
    VAR(usize),
    CONST(i64)
}

use Operand::*;

impl Operand {
    fn get(&self, memory: &Memory) -> i64 {
        match self {
            VAR(index) => memory.variables[*index],
            CONST(value) => *value
        }
    }
}

struct Memory {
    input: LinkedList<i64>,
    variables: Vec<i64>
}

impl Memory {
    fn new() -> Memory {
        let variables = (0..4).map(|_| 0)
            .collect::<Vec<_>>();

        Memory {
            variables,
            input: LinkedList::new()
        }
    }

    fn clear(&mut self) {
        self.input.clear();
        self.variables.iter_mut()
            .for_each(|v| *v = 0);
    }
}

fn parse_operation(line: String) -> Box::<dyn Operation> {
    let pieces = line.split_ascii_whitespace()
        .collect::<Vec<_>>();
    let a = pieces[1].chars().next().unwrap();
    let a = 'w' as usize - a as usize;
    if pieces[0] == "inp" {
        Box::new(Inp { read_into: a })
    } else {
        let b = match pieces[2].parse::<i64>() {
            Ok(result) => CONST(result),
            _ => {
                let v = pieces[2].chars().next().unwrap();
                let v = 'w' as usize - v as usize;
                VAR(v)
            }
        };
        match pieces[0] {
            "add" => Box::new(Add { a, b }),
            "mul" => Box::new(Mul { a, b }),
            "div" => Box::new(Div { a, b }),
            "mod" => Box::new(Mod { a, b }),
            "eql" => Box::new(Eql { a, b }),
            _ => panic!("Invalid operator: {}", pieces[0])
        }
    }
}

trait Operation {
    fn execute(&self, memory: &mut Memory);
}

struct Inp {
    read_into: usize
}
impl Operation for Inp {
    fn execute(&self, memory: &mut Memory) {
        let next = memory.input.pop_front().unwrap();
        memory.variables[self.read_into] = next;
    }
}
struct Add {
    a: usize,
    b: Operand
}
impl Operation for Add {
    fn execute(&self, memory: &mut Memory) {
        let a = memory.variables[self.a];
        let b = self.b.get(memory);
        memory.variables[self.a] = a + b;
    }
}
struct Mul {
    a: usize,
    b: Operand
}
impl Operation for Mul {
    fn execute(&self, memory: &mut Memory) {
        let a = memory.variables[self.a];
        let b = self.b.get(memory);
        memory.variables[self.a] = a * b;
    }
}
struct Div {
    a: usize,
    b: Operand
}
impl Operation for Div {
    fn execute(&self, memory: &mut Memory) {
        let a = memory.variables[self.a];
        let b = self.b.get(memory);
        memory.variables[self.a] = a / b;
    }
}
struct Mod {
    a: usize,
    b: Operand
}
impl Operation for Mod {
    fn execute(&self, memory: &mut Memory) {
        let a = memory.variables[self.a];
        let b = self.b.get(memory);
        memory.variables[self.a] = a % b;
    }
}
struct Eql {
    a: usize,
    b: Operand
}
impl Operation for Eql {
    fn execute(&self, memory: &mut Memory) {
        let a = memory.variables[self.a];
        let b = self.b.get(memory);
        memory.variables[self.a] = match a == b {
            true => 1,
            false => 0
        };
    }
}

struct Monad {
    operations: Vec<Box<dyn Operation>>
}

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

impl Monad {
    fn parse(file_name: &str) -> Monad {
        let operations = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| parse_operation(line))
            .collect::<Vec<_>>();
        Monad { operations }
    }
}

fn part_one(file_name: &str) {
    let monad = Monad::parse(file_name);
    let mut memory = Memory::new();

    
    println!("Part 1: {}", "incomplete");
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
