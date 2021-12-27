use std::fmt::Formatter;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

#[derive(Clone, Debug)]
enum Operator {
    INP(usize),
    ADD(Operand, Operand),
    MUL(Operand, Operand),
    DIV(Operand, Operand),
    MOD(Operand, Operand),
    EQL(Operand, Operand)
}

impl Operator {
    fn eval(&self, input: &Vec<i64>) -> i64 {
        match self {
            INP(a) => input[*a],
            ADD(a, b) => a.get(input) + b.get(input),
            MUL(a, b) => a.get(input) * b.get(input),
            DIV(a, b) => a.get(input) / b.get(input),
            MOD(a, b) => a.get(input) % b.get(input),
            EQL(a, b) => (a.get(input) == b.get(input)) as i64
        }
    }
    fn print(&self, indent: String) {
        if let INP(a) = self {
            println!("{}input[{}]", indent, a);
        } else {
            let (a, b, op) = match self {
                ADD(a, b) => (a, b, "add"),
                MUL(a, b) => (a, b, "mul"),
                DIV(a, b) => (a, b, "div"),
                MOD(a, b) => (a, b, "mod"),
                EQL(a, b) => (a, b, "eql"),
                _ => panic!("Unknown operator")
            };
            println!("{}(", indent);
            a.print(indent.clone() + "  ");
            println!("{}  {}", indent, op);
            b.print(indent.clone() + "  ");
            println!("{})", indent);
        }
    }
}

impl Display for Operator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            INP(a) => write!(fmt, "input[{}]", a),
            ADD(a, b) => write!(fmt, "({} + {})", a, b),
            MUL(a, b) => write!(fmt, "({} * {})", a, b),
            DIV(a, b) => write!(fmt, "({} / {})", a, b),
            MOD(a, b) => write!(fmt, "({} % {})", a, b),
            EQL(a, b) => write!(fmt, "({} = {})", a, b),
        }
    }
}

#[derive(Clone, Debug)]
enum Operand {
    CONST(i64),
    OPERATOR(Box<Operator>)
}

impl Operand {
    fn print(&self, indent: String) {
        match self {
            CONST(value) => println!("{}{}", indent, value),
            OPERATOR(operator) => operator.print(indent)
        };
    }
}

impl Display for Operand {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            CONST(value) => write!(fmt, "{}", *value),
            OPERATOR(operator) => write!(fmt, "{}", *operator)
        }
    }
}

impl Operand {
    fn get(&self, input: &Vec<i64>) -> i64 {
        match self {
            CONST(value) => *value,
            OPERATOR(operator) => operator.eval(input)
        }
    }
}

use Operand::*;
use Operator::*;

struct Monad {
    input_count: usize,
    variables: Vec<Operand>
}

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

impl Monad {
    fn new() -> Monad {
        Monad {
            input_count: 0,
            variables: (0..4).map(|_| CONST(0))
                .collect::<Vec<_>>()
        }
    }
    fn parse(&mut self, file_name: &str) {
        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .for_each(|line| self.parse_operator(line));
    }
    fn parse_operator(&mut self, line: String) {
        println!("parsing: {}", line);
        let pieces = line.split_ascii_whitespace()
            .collect::<Vec<_>>();
        let a = pieces[1].chars().next().unwrap();
        let var_index = a as usize - 'w' as usize;
        if pieces[0] == "inp" {
            let input = INP(self.input_count);
            self.variables[var_index] = OPERATOR(Box::new(input));
            self.input_count += 1;
        } else {
            let a = self.variables[var_index].clone();
            let b = match pieces[2].parse::<i64>() {
                Ok(result) => CONST(result),
                _ => {
                    let v = pieces[2].chars().next().unwrap();
                    let v = v as usize - 'w' as usize;
                    self.variables[v].clone()
                }
            };

            let result = match (pieces[0], &a, &b) {
                (_, CONST(_), CONST(_)) => 
                    CONST(Monad::binary_operator(pieces[0], a, b).eval(&Vec::with_capacity(0))),
                ("mul", CONST(0), _) => CONST(0),
                ("mul", _, CONST(0)) => CONST(0),
                ("mul", CONST(1), b) => b.clone(),
                ("mul", a, CONST(1)) => a.clone(),
                ("div", CONST(0), _) => CONST(0),
                ("div", a, CONST(1)) => a.clone(),
                ("add", CONST(0), b) => b.clone(),
                ("add", a, CONST(0)) => a.clone(),
                _ => OPERATOR(Box::new(Monad::binary_operator(pieces[0], a, b)))
            };
            if let OPERATOR(result) = result {
                let result = match *result {
                    EQL(OPERATOR(l), CONST(r)) if r >= 10 => {
                        match *l {
                            INP(_) => CONST(0),
                            other => OPERATOR(Box::new(EQL(OPERATOR(Box::new(other)), CONST(r))))
                        }
                    },
                    EQL(CONST(l), OPERATOR(r)) if l >= 10 => {
                        match *r {
                            INP(_) => CONST(0),
                            other => OPERATOR(Box::new(EQL(CONST(l), OPERATOR(Box::new(other)))))
                        }
                    },
                    other => OPERATOR(Box::new(other))
                };
                self.variables[var_index] = result;
            } else {
                self.variables[var_index] = result;
            }
        }
    }
    fn binary_operator(symbol: &str, a: Operand, b: Operand) -> Operator {
        match symbol {
            "add" => ADD(a, b),
            "mul" => MUL(a, b),
            "div" => DIV(a, b),
            "mod" => MOD(a, b),
            "eql" => EQL(a, b),
            _ => panic!("Invalid operator: {}", symbol)
        }
    }
}

fn part_one(file_name: &str) {
    let mut monad = Monad::new();
    monad.parse(file_name);

    println!("z:");
    monad.variables[3].print(String::from(""));
    
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
