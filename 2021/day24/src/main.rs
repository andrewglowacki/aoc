use std::fmt::Formatter;
use std::fmt::Display;
use std::collections::LinkedList;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

const MODEL_LEN: usize = 14;

enum Operand {
    VAR(usize),
    CONST(i64)
}

impl Display for Operand {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            VAR(index) => write!(fmt, "{}", (('w' as u8) + *index as u8) as char),
            CONST(value) => write!(fmt, "{}", value)
        }
    }
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
    input: Vec<i64>,
    variables: Vec<i64>,
    snapshots: Vec<Vec<i64>>
}

impl Memory {
    fn new() -> Memory {
        let variables = (0..4).map(|_| 0)
            .collect::<Vec<_>>();

        let input = (0..MODEL_LEN).map(|_| 0)
            .collect::<Vec<_>>();

        Memory {
            variables,
            input,
            snapshots: Vec::new()
        }
    }

    fn snapshot(&mut self) {
        self.snapshots.push(self.variables.to_vec());
    }

    fn pop_snapshot(&mut self) {
        self.variables = self.snapshots.pop().unwrap();
    }

    fn clear(&mut self) {
        self.input.clear();
        self.variables.iter_mut()
            .for_each(|v| *v = 0);
    }
}

fn parse_operation(line: String, input_count: &mut usize) -> Box::<dyn Operation> {
    let pieces = line.split_ascii_whitespace()
        .collect::<Vec<_>>();
    let a = pieces[1].chars().next().unwrap();
    let a = a as usize - 'w' as usize;
    if pieces[0] == "inp" {
        let index = *input_count;
        *input_count += 1;
        Box::new(Inp { read_into: a, index })
    } else {
        let b = match pieces[2].parse::<i64>() {
            Ok(result) => CONST(result),
            _ => {
                let v = pieces[2].chars().next().unwrap();
                let v = v as usize - 'w' as usize;
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
    fn print(&self);
}

struct Inp {
    read_into: usize,
    index: usize
}
impl Operation for Inp {
    fn execute(&self, memory: &mut Memory) {
        memory.variables[self.read_into] = memory.input[self.index];
    }
    fn print(&self) {
        println!("{}", self);
    }
}
impl Display for Inp {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{} = input[{}]", (('w' as u8) + (self.read_into as u8)) as char, self.index)
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
    fn print(&self) {
        println!("{}", self);
    }
}
impl Display for Add {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "add {} {}", (('w' as u8) + (self.a as u8)) as char, self.b)
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
    fn print(&self) {
        println!("{}", self);
    }
}
impl Display for Mul {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "mul {} {}", (('w' as u8) + (self.a as u8)) as char, self.b)
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
    fn print(&self) {
        println!("{}", self);
    }
}
impl Display for Div {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "div {} {}", (('w' as u8) + (self.a as u8)) as char, self.b)
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
    fn print(&self) {
        println!("{}", self);
    }
}
impl Display for Mod {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "mod {} {}", (('w' as u8) + (self.a as u8)) as char, self.b)
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
    fn print(&self) {
        println!("{}", self);
    }
}
impl Display for Eql {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "eql {} {}", (('w' as u8) + (self.a as u8)) as char, self.b)
    }
}

struct Monad {
    groups: Vec<Vec<Box<dyn Operation>>>
}

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

impl Monad {
    fn parse(file_name: &str) -> Monad {
        let mut input_count = 0;
        let mut groups = Vec::new();
        let mut current = Vec::<Box<dyn Operation>>::new();
        for line in get_file_lines(file_name).flat_map(|line| line.ok()) {
            let cur_input_count = input_count;
            let operation = parse_operation(line, &mut input_count);
            if cur_input_count != input_count && cur_input_count > 0 {
                println!("Parsed {} operations for group {}", current.len(), groups.len());
                groups.push(current);
                current = Vec::new();
            }
            current.push(operation);
        }
        groups.push(current);
        Monad { groups }
    }
    fn find_largest_model(&self, group: usize, memory: &mut Memory) -> bool {
        // println!("[{}] initial variables are: {:?}", group, memory.variables);
        let operations = &self.groups[group];
        let snapshot = memory.variables.to_vec();
        for i in 1..10 {
            let num = 10 - i;
            if group < 7 {
                println!("[{}] num: {} variables are: {:?}", group, num, memory.variables);
            }
            memory.input[group] = num;
            for operation in operations {
                operation.execute(memory);
            }
            if group + 1 >= MODEL_LEN {
                return memory.variables[3] == 0;
            } else if self.find_largest_model(group + 1, memory) {
                return true;
            } else {
                // println!("[{}] Before pop: {:?}", group, memory.variables);
                for i in 0..4 {
                    memory.variables[i] = snapshot[i];
                }
                // println!("[{}] After pop: {:?}", group, memory.variables);
            }
        }
        false
    }
}

fn part_one(file_name: &str) {
    let monad = Monad::parse(file_name);
    let mut memory = Memory::new();

    assert_eq!(true, monad.find_largest_model(0, &mut memory));
    
    let largest_model = memory.input.iter()
        .fold(0, |acc, num| (acc * 10) + num);

    println!("Part 1: {}", largest_model);
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