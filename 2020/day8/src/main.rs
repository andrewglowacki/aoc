use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

struct State {
    next: usize,
    acc: i32,
    executed: HashSet<usize>
}

impl State {
    fn new(start_at: usize) -> State {
        State {
            next: start_at,
            acc: 0,
            executed: HashSet::new()
        }
    }

    fn is_loop(&self) -> bool {
        self.executed.contains(&self.next)
    }
}

#[derive(PartialEq, Clone, Copy)]
enum InstructionType {
    Noop,
    Jump,
    Accumulate
}

trait Instruction {
    fn execute(&self, state: &mut State);
    fn get_type(&self) -> InstructionType;
    fn get_amount(&self) -> i32;
}

#[derive(Clone, Copy)]
struct Noop { 
    amount: i32
}

impl Instruction for Noop {
    fn execute(&self, state: &mut State) {
        state.next += 1;
    }
    fn get_type(&self) -> InstructionType {
        InstructionType::Noop
    }
    fn get_amount(&self) -> i32 {
        self.amount
    }
}
impl Noop {
    fn from_str(amount_str: &str) -> Box<dyn Instruction> {
        Noop::new(amount_str.parse::<i32>().unwrap())
    }
    fn new(amount: i32) -> Box<dyn Instruction> {
        Box::new(Noop {
            amount
        })
    }
}

#[derive(Clone, Copy)]
struct Accumulate { 
    amount: i32
}
impl Instruction for Accumulate {
    fn execute(&self, state: &mut State) {
        state.acc += self.amount;
        state.next += 1;
    }
    fn get_type(&self) -> InstructionType {
        InstructionType::Accumulate
    }
    fn get_amount(&self) -> i32 {
        self.amount
    }
}
impl Accumulate {
    fn from_str(amount_str: &str) -> Box<dyn Instruction> {
        Accumulate::new(amount_str.parse::<i32>().unwrap())
    }
    fn new(amount: i32) -> Box<dyn Instruction> {
        Box::new(Accumulate {
            amount
        })
    }
}

struct Jump {
    amount: i32
}
impl Instruction for Jump {
    fn execute(&self, state: &mut State) {
        let next = (state.next as i32) + self.amount;
        if next < 0 {
            panic!("Next instruction is less than zero: {}", next);
        }
        state.next = next as usize;
    }
    fn get_type(&self) -> InstructionType {
        InstructionType::Jump
    }
    fn get_amount(&self) -> i32 {
        self.amount
    }
}
impl Jump {
    fn from_str(amount_str: &str) -> Box<dyn Instruction> {
        Jump::new(amount_str.parse::<i32>().unwrap())
    }
    fn new(amount: i32) -> Box<dyn Instruction> {
        Box::new(Jump {
            amount
        })
    }
}

fn parse_instruction(line: &String) -> Box<dyn Instruction> {
    let tokens = line.split(' ').collect::<Vec<&str>>();
    match tokens[0] {
        "nop" => Noop::from_str(tokens[1]),
        "jmp" => Jump::from_str(tokens[1]),
        "acc" => Accumulate::from_str(tokens[1]),
        _ => panic!("Invalid instruction code: {}", line)
    }
}

fn run_program(instructions: &Vec<Box<dyn Instruction>>, state: State) -> State {
    let mut state = state;

    while !state.is_loop() && state.next < instructions.len() {
        let next = state.next;
        let instruction =  &instructions[next];
        instruction.execute(&mut state);
        state.executed.insert(next);
    }

    state
}

fn fix_instructions(instructions: Vec<Box<dyn Instruction>>, failed_state: State) -> State {
    let mut tried_jump_tos = HashSet::<usize>::new();
    loop {
        let jump_to = (0..instructions.len())
            .filter(|index| !failed_state.executed.contains(&index))
            .filter(|index| !tried_jump_tos.contains(index))
            .find(|index| run_program(&instructions, State::new(*index)).next == instructions.len())
            .unwrap();
        
        tried_jump_tos.insert(jump_to);

        let mut state = State::new(0);
        let mut found = false;
        while !state.is_loop() {
            let next = state.next;
            let instruction =  &instructions[next];

            if instruction.get_type() == InstructionType::Noop && (next as i32 + instruction.get_amount()) == jump_to as i32 {
                found = true;
                break;
            } else if instruction.get_type() == InstructionType::Jump && next + 1 == jump_to {
                found = true;
                break;
            } else {
                instruction.execute(&mut state);
                state.executed.insert(next);
            }
        }

        if !found {
            continue
        }

        state.next = jump_to;
        return run_program(&instructions, state);
    }
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).unwrap();

    let instructions = BufReader::new(file).lines()
        .flat_map(|line| line.ok())
        .map(|line| parse_instruction(&line))
        .collect::<Vec<Box<dyn Instruction>>>();

    println!("have {} instructions", instructions.len());

    let state = run_program(&instructions, State::new(0));
    println!("Part One: accumulator is {} before executing {}", state.acc, state.next);

    let state = fix_instructions(instructions, state);
    println!("Part Two: accumulator is {} before executing {}", state.acc, state.next);
}
