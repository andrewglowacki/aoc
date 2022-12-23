use std::collections::{HashMap, BTreeSet};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Debug)]
enum Operation {
    Literal(i64),
    Add,
    Subtract,
    Divide,
    Multiply
}

struct Monkey {
    operation: Operation,
    left: Option<String>,
    right: Option<String>
}

impl Monkey {
    fn new(operation: Operation, left: Option<String>, right: Option<String>) -> Monkey {
        Monkey {
            operation,
            left,
            right
        }
    }
}

struct Riddle {
    monkeys:  HashMap<String, Monkey>
}

impl Riddle {
    fn new() -> Riddle {
        Riddle {
            monkeys: HashMap::new()
        }
    }

    fn has_input(&self, name: &String) -> bool {
        if name == "humn" {
            true
        } else {
            let monkey = self.monkeys.get(name).unwrap();
            if let Some(left) = &monkey.left {
                if self.has_input(left) || self.has_input(monkey.right.as_ref().unwrap()) {
                    return true;
                }
            }
            false
        }
    }

    fn solve_human(&self) -> i64 {
        let root = self.monkeys.get("root").unwrap();

        let left_name = root.left.as_ref().unwrap();
        let right_name = root.right.as_ref().unwrap();

        let (solve_name, match_name) = match self.has_input(left_name) {
            true => (left_name, right_name),
            false => (right_name, left_name)
        };

        let mut answers = HashMap::new();
        let match_value = self.eval_monkey(match_name, &mut answers);
        self.solve_for(solve_name, &mut answers, match_value)
    }

    fn solve(&self) -> i64 {
        let mut answers = HashMap::new();
        self.eval_monkey(&"root".to_owned(), &mut answers)
    }

    fn solve_for(&self, name: &String, answers: &mut HashMap<String, i64>, result: i64) -> i64 {
        let monkey = self.monkeys.get(name).unwrap();

        if name == "humn" {
            return result;
        }

        let left_name = monkey.left.as_ref().unwrap();
        let right_name = monkey.right.as_ref().unwrap();

        let (solve_name, match_name) = match self.has_input(left_name) {
            true => (left_name, right_name),
            false => (right_name, left_name)
        };

        let operand_value = self.eval_monkey(match_name, answers);

        let match_value = if left_name == solve_name {
            match monkey.operation {
                Operation::Add => result - operand_value,
                Operation::Subtract => result + operand_value,
                Operation::Multiply => result / operand_value,
                Operation::Divide => result * operand_value,
                _ => panic!("Encountered unexpected literal")
            }
        } else {
            match monkey.operation {
                Operation::Add => result - operand_value,
                Operation::Subtract => operand_value - result,
                Operation::Multiply => result / operand_value,
                Operation::Divide => operand_value / result,
                _ => panic!("Encountered unexpected literal")
            }
        };

        self.solve_for(solve_name, answers, match_value)
    }

    fn eval_monkey(&self, name: &String, answers: &mut HashMap<String, i64>) -> i64 {
        if let Some(result) = answers.get(name) {
            *result
        } else {
            let monkey = self.monkeys.get(name).unwrap();
            let value = if let Operation::Literal(value) = monkey.operation {
                value
            } else {
                let left_name = monkey.left.as_ref().unwrap();
                let right_name = monkey.right.as_ref().unwrap();
                let left = self.eval_monkey(&left_name, answers);
                let right = self.eval_monkey(&right_name, answers);
                let value = match monkey.operation {
                    Operation::Add => left + right,
                    Operation::Subtract => left - right,
                    Operation::Multiply => left * right,
                    Operation::Divide => left / right,
                    _ => panic!("impossible situation")
                };
                value
            };
            answers.insert(name.to_owned(), value);
            value
        }
    }
    
    fn add(&mut self, line: String) {
        let name = line[0..4].to_owned();
        let operation_str = &line[6..];

        let (operation, left, right) = match operation_str.contains(" ") {
            false => {
                let value = operation_str.parse::<i64>().unwrap();
                (Operation::Literal(value), None, None)
            },
            true => {
                let left = operation_str[0..4].to_owned();
                let right = operation_str[7..].to_owned();
                (match &operation_str[5..6] {
                    "+" => Operation::Add,
                    "-" => Operation::Subtract,
                    "*" => Operation::Multiply,
                    "/" => Operation::Divide,
                    _ => panic!("Invalid operator: {}", &operation_str[5..6])
                }, Some(left), Some(right))
            }
        };

        self.monkeys.insert(name, Monkey::new(operation, left, right));
    }
}

fn part_one(file_name: &str) {
    let mut riddle = Riddle::new();

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .for_each(|line| riddle.add(line));
    
    let riddle = riddle;

    let answer = riddle.solve();

    println!("Part 1: {}", answer);
}

fn part_two(file_name: &str) {
    let mut riddle = Riddle::new();

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .for_each(|line| riddle.add(line));
    
    let answer = riddle.solve_human();
    
    println!("Part 2: {}", answer);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
