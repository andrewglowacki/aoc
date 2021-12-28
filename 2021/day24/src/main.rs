use std::collections::BTreeSet;
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
    fn combine_operator_values<F>(a: &Operator, b: &Operator, combiner: F) -> BTreeSet<i64>
        where F: Fn(i64, i64) -> i64 
    {
        let a_values = a.get_possible_values();
        let b_values = b.get_possible_values();
        let mut results = BTreeSet::new();
        for a_value in a_values {
            for b_value in b_values.iter() {
                results.insert(combiner(a_value, *b_value));
            }
        }
        results
    }
    fn get_possible_values_generic<F>(a: &Operand, b: &Operand, combiner: F) -> BTreeSet<i64> 
        where F: Fn(i64, i64) -> i64 
    {
        match (a, b) {
            (CONST(a), OPERATOR(b)) => {
                b.get_possible_values().iter()
                    .map(|v| combiner(*a, *v))
                    .collect::<BTreeSet<_>>()
            },
            (OPERATOR(a), CONST(b)) => {
                a.get_possible_values().iter()
                    .map(|v| combiner(*v, *b))
                    .collect::<BTreeSet<_>>()
            },
            (OPERATOR(a), OPERATOR(b)) => Operator::combine_operator_values(a, b, combiner),
            u => panic!("Unexpected pattern: {:?}", u)
        }
    }
    fn get_possible_values(&self) -> BTreeSet<i64> {
        match self {
            INP(_) => (1..10).into_iter().collect::<BTreeSet<_>>(),
            ADD(a, b) => Operator::get_possible_values_generic(a, b, |a, b| a + b),
            MUL(a, b) => Operator::get_possible_values_generic(a, b, |a, b| a * b),
            DIV(a, b) => Operator::get_possible_values_generic(a, b, |a, b| a / b),
            MOD(a, b) => Operator::get_possible_values_generic(a, b, |a, b| a % b),
            EQL(_, _) => {
                let mut values = BTreeSet::new();
                values.insert(0);
                values.insert(1);
                values
            }
        }
    }
    fn merge_two_operator_values<F>(a: &Operator, b: &Operator, get_b_value: F) -> BTreeSet<Vec<(usize, usize)>>
        where F: Fn(i64) -> i64 
    {
        a.get_possible_values().into_iter()
            .map(|value| (value, get_b_value(value)))
            .flat_map(|(a_val, b_val)| {
                let mut a_inputs = a.find_input_values_for(a_val);
                let b_inputs = b.find_input_values_for(b_val);
                a_inputs.retain(|value| b_inputs.contains(value));
                a_inputs
            })
            .collect::<BTreeSet<_>>()
    }

    fn find_input_values_for(&self, value: i64) -> BTreeSet<Vec<(usize, usize)>> {
        match self {
            INP(index) => {
                match value > 0 && value < 10 {
                    true => { 
                        let mut values = BTreeSet::new();
                        values.insert(vec![(*index, value as usize)]); 
                        values
                    },
                    false => BTreeSet::new()
                }
            },
            ADD(a, b) => {
                match (a, b) {
                    (CONST(a), OPERATOR(b)) => b.find_input_values_for(value - a),
                    (OPERATOR(a), CONST(b)) => a.find_input_values_for(value - b),
                    (OPERATOR(a), OPERATOR(b)) => Operator::merge_two_operator_values(a, b, |a| value - a),
                    u => panic!("Unexpected pattern: {:?}", u)
                }
            },
            MUL(a, b) => {
                match (a, b) {
                    (CONST(a), OPERATOR(b)) => b.find_input_values_for(value / a),
                    (OPERATOR(a), CONST(b)) => a.find_input_values_for(value / b),
                    (OPERATOR(a), OPERATOR(b)) => Operator::merge_two_operator_values(a, b, |a| value / a),
                    u => panic!("Unexpected pattern: {:?}", u)
                }
            }
            DIV(a, b) => {
                match (a, b) {
                    // A / b = value | A = value*b | A / value = b
                    (CONST(a), OPERATOR(b)) => b.find_input_values_for(a / value),
                    (OPERATOR(a), CONST(b)) => a.find_input_values_for(b / value),
                    (OPERATOR(a), OPERATOR(b)) => Operator::merge_two_operator_values(a, b, |a| a / value),
                    u => panic!("Unexpected pattern: {:?}", u)
                }
            },
            MOD(a, b) => {
                match (a, b) {
                    (CONST(a), OPERATOR(b)) => {
                        b.get_possible_values().into_iter()
                            .filter(|check| a % check == value)
                            .flat_map(|value| b.find_input_values_for(value))
                            .collect::<BTreeSet<_>>()
                    },
                    (OPERATOR(a), CONST(b)) => {
                        a.get_possible_values().into_iter()
                            .filter(|check| check % b == value)
                            .flat_map(|value| a.find_input_values_for(value))
                            .collect::<BTreeSet<_>>()
                    },
                    (OPERATOR(a), OPERATOR(b)) => {
                        let a_values = a.get_possible_values();
                        let b_values = b.get_possible_values();
                        a_values.iter()
                            .flat_map(|a_val| {
                                let a_inputs = a.find_input_values_for(*a_val);
                                let mut results = BTreeSet::<Vec<(usize, usize)>>::new();
                                for b_val in b_values.iter() {
                                    if a_val % b_val == value {
                                        let b_inputs = b.find_input_values_for(*b_val);
                                        b_inputs.into_iter()
                                            .filter(|b_input| a_inputs.contains(b_input))
                                            .for_each(|common| { results.insert(common); });
                                    }
                                }
                                results
                            })
                            .collect::<BTreeSet<_>>()
                    }
                    u => panic!("Unexpected pattern: {:?}", u)
                }
            },
            EQL(a, b) => {
                match (a, b) {
                    (CONST(a), OPERATOR(b)) => {
                        match value == 1 {
                            true => b.find_input_values_for(*a),
                            false => {
                                let values = b.get_possible_values();
                                values.into_iter()
                                    .filter(|value| value != a )
                                    .flat_map(|value| b.find_input_values_for(value))
                                    .collect::<BTreeSet<_>>()
                            }
                        }
                    },
                    (OPERATOR(a), CONST(b)) => {
                        match value == 1 {
                            true => a.find_input_values_for(*b),
                            false => {
                                let values = a.get_possible_values();
                                values.into_iter()
                                    .filter(|value| value != b )
                                    .flat_map(|value| a.find_input_values_for(value))
                                    .collect::<BTreeSet<_>>()
                            }
                        }
                    },
                    (OPERATOR(a), OPERATOR(b)) => {
                        let a_values = a.get_possible_values();
                        let b_values = b.get_possible_values();
                        match value == 1 {
                            true => {
                                a_values.intersection(&b_values)
                                    .flat_map(|value| {
                                        let mut a_inputs = a.find_input_values_for(*value);
                                        let b_inputs = b.find_input_values_for(*value);
                                        a_inputs.retain(|input| b_inputs.contains(input));
                                        a_inputs
                                    })
                                    .collect::<BTreeSet<_>>()
                            },
                            false => {
                                let mut results = BTreeSet::new();
                                for a_val in a_values {
                                    let a_inputs = a.find_input_values_for(a_val);
                                    for b_val in b_values.iter() {
                                        if *b_val != a_val {
                                            b.find_input_values_for(value).into_iter()
                                                .filter(|input| a_inputs.contains(input))
                                                .for_each(|input| { results.insert(input); });
                                        }
                                    }
                                }
                                results
                            }
                        }
                    },
                    u => panic!("Unexpected pattern: {:?}", u)
                }
            }
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

    println!("Finding inputs...");
    let values = match &monad.variables[3] {
        OPERATOR(operator) => operator.find_input_values_for(0),
        x => panic!("Unexpected top level variable: {:?}", x)
    };
    
    println!("Part 1: {:?}", values);
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
