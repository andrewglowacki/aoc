use std::collections::HashMap;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

#[derive(Debug)]
enum Operator {
    INP(usize, usize),
    ADD(usize, Operand, Operand),
    MUL(usize, Operand, Operand),
    DIV(usize, Operand, Operand),
    MOD(usize, Operand, Operand),
    EQL(usize, Operand, Operand)
}

impl Operator {
    fn eval(&self, input: &Vec<i64>) -> i64 {
        match self {
            INP(_, a) => input[*a],
            ADD(_, a, b) => a.get(input) + b.get(input),
            MUL(_, a, b) => a.get(input) * b.get(input),
            DIV(_, a, b) => a.get(input) / b.get(input),
            MOD(_, a, b) => a.get(input) % b.get(input),
            EQL(_, a, b) => (a.get(input) == b.get(input)) as i64
        }
    }
    fn print(&self, indent: String) {
        if let INP(id, a) = self {
            println!("{}input[{}] #{}", indent, a, id);
        } else {
            let (a, b, op, id) = match self {
                ADD(id, a, b) => (a, b, "add", id),
                MUL(id, a, b) => (a, b, "mul", id),
                DIV(id, a, b) => (a, b, "div", id),
                MOD(id, a, b) => (a, b, "mod", id),
                EQL(id, a, b) => (a, b, "eql", id),
                _ => panic!("Unknown operator")
            };
            println!("{}( #{}", indent, id);
            a.print(indent.clone() + "  ");
            println!("{}  {} #{}", indent, op, id);
            b.print(indent.clone() + "  ");
            println!("{}) #{}", indent, id);
        }
    }
    fn get_type(&self) -> &'static str {
        match self {
            INP(_, _) => "inp",
            ADD(_, _, _) => "add",
            MUL(_, _, _) => "mul",
            DIV(_, _, _) => "div",
            MOD(_, _, _) => "mod",
            EQL(_, _, _) => "eql"
        }
    }
    fn id(&self) -> usize {
        match self {
            INP(id, _) => *id,
            ADD(id, _, _) => *id,
            MUL(id, _, _) => *id,
            DIV(id, _, _) => *id,
            MOD(id, _, _) => *id,
            EQL(id, _, _) => *id
        }
    }
    fn combine_inputs(a_inputs: &Vec<BTreeMap<u8, u8>>, b_inputs: &Vec<BTreeMap<u8, u8>>) -> Vec<BTreeMap<u8, u8>> {
        if a_inputs.is_empty() || b_inputs.is_empty() {
            return Vec::with_capacity(0);
        }
        let mut results = Vec::new();
        for a_input in a_inputs {
            for b_input in b_inputs {
                let mut new_input = BTreeMap::new();
                let mut matches = true;
                for (index, a_value) in a_input.iter() {
                    if let Some(b_value) = b_input.get(&index) {
                        if a_value != b_value {
                            matches = false;
                            break;
                        }
                    }
                    // add inputs in a but not b, or ones that are in both
                    new_input.insert(*index, *a_value);
                }
                if !matches {
                    continue;
                }
                // add inputs in b but not a
                for (index, b_value) in b_input.iter() {
                    if !a_input.contains_key(index) {
                        new_input.insert(*index, *b_value);
                    }
                }
                results.push(new_input);
            }
        }
        results
    }
    fn combine_operator_values<F>(monad: &mut Monad, a: &Operator, b: &Operator, combiner: F) -> BTreeSet<i64>
        where F: Fn(i64, i64) -> i64 
    {
        let a_values = a.get_possible_values(monad);
        let b_values = b.get_possible_values(monad);
        let mut results = BTreeSet::new();
        for a_value in a_values {
            for b_value in b_values.iter() {
                results.insert(combiner(a_value, *b_value));
            }
        }
        results
    }
    fn get_possible_values_generic<F>(monad: &mut Monad, id: &usize, a: &Operand, b: &Operand, combiner: F) -> BTreeSet<i64> 
        where F: Fn(i64, i64) -> i64 
    {
        if let Some(values) = monad.get_output_values(id) {
            return values;
        }
        let values = match (a, b) {
            (CONST(a), OPERATOR(b)) => {
                b.get_possible_values(monad).iter()
                    .map(|v| combiner(*a, *v))
                    .collect::<BTreeSet<_>>()
            },
            (OPERATOR(a), CONST(b)) => {
                a.get_possible_values(monad).iter()
                    .map(|v| combiner(*v, *b))
                    .collect::<BTreeSet<_>>()
            },
            (OPERATOR(a), OPERATOR(b)) => Operator::combine_operator_values(monad, a, b, combiner),
            u => panic!("Unexpected pattern: {:?}", u)
        };
        monad.set_output_values(*id, values.clone());
        values
    }
    fn get_possible_values(&self, monad: &mut Monad) -> BTreeSet<i64> {
        match self {
            INP(_, _) => (1..10).into_iter().collect::<BTreeSet<_>>(),
            ADD(id, a, b) => Operator::get_possible_values_generic(monad, id, a, b, |a, b| a + b),
            MUL(id, a, b) => Operator::get_possible_values_generic(monad, id, a, b, |a, b| a * b),
            DIV(id, a, b) => Operator::get_possible_values_generic(monad, id, a, b, |a, b| a / b),
            MOD(id, a, b) => Operator::get_possible_values_generic(monad, id, a, b, |a, b| a % b),
            EQL(_, _, _) => {
                let mut values = BTreeSet::new();
                values.insert(0);
                values.insert(1);
                values
            }
        }
    }
    fn merge_two_operator_values<F>(monad: &mut Monad, a: &Operator, b: &Operator, get_b_value: F) -> Vec<BTreeMap<u8, u8>>
        where F: Fn(i64) -> i64 
    {
        a.get_possible_values(monad).into_iter()
            .map(|value| (value, get_b_value(value)))
            .flat_map(|(a_val, b_val)| {
                let a_inputs = a.find_input_values_for(monad, a_val);
                let b_inputs = b.find_input_values_for(monad, b_val);
                Operator::combine_inputs(&a_inputs, &b_inputs)
            })
            .collect::<Vec<_>>()
    }

    fn find_input_values_for(&self, monad: &mut Monad, value: i64) -> Vec<BTreeMap<u8, u8>> {
        if let Some(values) = monad.get_input_values(self, value) {
            return values;
        }
        let values = match self {
            INP(_, index) => {
                match value > 0 && value < 10 {
                    true => { 
                        let mut values = BTreeMap::new();
                        values.insert(*index as u8, value as u8); 
                        vec![values]
                    },
                    false => Vec::with_capacity(0)
                }
            },
            ADD(_, a, b) => {
                match (a, b) {
                    (CONST(a), OPERATOR(b)) => b.find_input_values_for(monad, value - a),
                    (OPERATOR(a), CONST(b)) => a.find_input_values_for(monad, value - b),
                    (OPERATOR(a), OPERATOR(b)) => Operator::merge_two_operator_values(monad, a, b, |a| value - a),
                    u => panic!("Unexpected pattern: {:?}", u)
                }
            },
            MUL(_, a, b) => {
                match (a, b) {
                    (CONST(a), OPERATOR(b)) => b.find_input_values_for(monad, value / a),
                    (OPERATOR(a), CONST(b)) => a.find_input_values_for(monad, value / b),
                    (OPERATOR(a), OPERATOR(b)) => {
                        a.get_possible_values(monad).into_iter()
                            .flat_map(|a_val| {
                                let a_inputs = a.find_input_values_for(monad, a_val);
                                match a_val == 0 {
                                    true => b.get_possible_values(monad).into_iter()
                                        .filter(|b_val| b_val * a_val == value)
                                        .flat_map(|b_val| Operator::combine_inputs(&a_inputs, &b.find_input_values_for(monad, b_val)))
                                        .collect::<Vec<_>>(),
                                    false => Operator::combine_inputs(&a_inputs, &b.find_input_values_for(monad, value / a_val))
                                }
                            })
                            .collect::<Vec<_>>()
                    },
                    u => panic!("Unexpected pattern: {:?}", u)
                }
            }
            DIV(_, a, b) => {
                match (a, b) {
                    // A / b = value | A = value*b | A / value = b
                    (CONST(a), OPERATOR(b)) => {
                        if value == 0 {
                            // not possible to get a result of zero with a non-zero numerator
                            Vec::with_capacity(0)
                        } else {
                            b.find_input_values_for(monad, a / value)
                        }
                    },
                    // a / B = value | a = value * B
                    (OPERATOR(a), CONST(b)) => a.find_input_values_for(monad, b * value),
                    (OPERATOR(a), OPERATOR(b)) => {
                        b.get_possible_values(monad).into_iter()
                            .flat_map(|b_val| {
                                let b_inputs = b.find_input_values_for(monad, b_val);
                                let a_inputs = a.find_input_values_for(monad, b_val * value);
                                Operator::combine_inputs(&a_inputs, &b_inputs)
                            })
                            .collect::<Vec<_>>()
                    },
                    u => panic!("Unexpected pattern: {:?}", u)
                }
            },
            MOD(_, a, b) => {
                match (a, b) {
                    (CONST(a), OPERATOR(b)) => {
                        b.get_possible_values(monad).into_iter()
                            .filter(|check| a % check == value)
                            .flat_map(|value| b.find_input_values_for(monad, value))
                            .collect::<Vec<_>>()
                    },
                    (OPERATOR(a), CONST(b)) => {
                        a.get_possible_values(monad).into_iter()
                            .filter(|check| check % b == value)
                            .flat_map(|value| a.find_input_values_for(monad, value))
                            .collect::<Vec<_>>()
                    },
                    (OPERATOR(a), OPERATOR(b)) => {
                        let a_values = a.get_possible_values(monad);
                        let b_values = b.get_possible_values(monad);
                        a_values.iter()
                            .flat_map(|a_val| {
                                let a_inputs = a.find_input_values_for(monad, *a_val);
                                b_values.iter()
                                    .filter(|b_val| a_val % *b_val == value)
                                    .flat_map(|b_val| {
                                        let b_inputs = b.find_input_values_for(monad, *b_val);
                                        Operator::combine_inputs(&a_inputs, &b_inputs)
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>()
                    }
                    u => panic!("Unexpected pattern: {:?}", u)
                }
            },
            EQL(_, a, b) => {
                match (a, b) {
                    (CONST(a), OPERATOR(b)) => {
                        match value == 1 {
                            true => b.find_input_values_for(monad, *a),
                            false => {
                                let values = b.get_possible_values(monad);
                                values.into_iter()
                                    .filter(|value| value != a )
                                    .flat_map(|value| b.find_input_values_for(monad, value))
                                    .collect::<Vec<_>>()
                            }
                        }
                    },
                    (OPERATOR(a), CONST(b)) => {
                        match value == 1 {
                            true => a.find_input_values_for(monad, *b),
                            false => {
                                let values = a.get_possible_values(monad);
                                values.into_iter()
                                    .filter(|value| value != b )
                                    .flat_map(|value| a.find_input_values_for(monad, value))
                                    .collect::<Vec<_>>()
                            }
                        }
                    },
                    (OPERATOR(a), OPERATOR(b)) => {
                        let a_values = a.get_possible_values(monad);
                        let b_values = b.get_possible_values(monad);
                        match value == 1 {
                            true => {
                                a_values.intersection(&b_values)
                                    .flat_map(|value| {
                                        let a_inputs = a.find_input_values_for(monad, *value);
                                        let b_inputs = b.find_input_values_for(monad, *value);
                                        Operator::combine_inputs(&a_inputs, &b_inputs)
                                    })
                                    .collect::<Vec<_>>()
                            },
                            false => {
                                a_values.iter()
                                    .flat_map(|a_val| {
                                        let a_inputs = a.find_input_values_for(monad, *a_val);
                                        b_values.iter()
                                            .filter(|b_val| *b_val != a_val)
                                            .flat_map(|b_val| {
                                                let b_inputs = b.find_input_values_for(monad, *b_val);
                                                Operator::combine_inputs(&a_inputs, &b_inputs)
                                            })
                                            .collect::<Vec<_>>()
                                    })
                                    .collect::<Vec<_>>()
                            }
                        }
                    },
                    u => panic!("Unexpected pattern: {:?}", u)
                }
            }
        };
        monad.set_input_values(self, value, values.to_vec());
        values
    }
}

impl Display for Operator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            INP(_, a) => write!(fmt, "input[{}]", a),
            ADD(_, a, b) => write!(fmt, "({} + {})", a, b),
            MUL(_, a, b) => write!(fmt, "({} * {})", a, b),
            DIV(_, a, b) => write!(fmt, "({} / {})", a, b),
            MOD(_, a, b) => write!(fmt, "({} % {})", a, b),
            EQL(_, a, b) => write!(fmt, "({} = {})", a, b),
        }
    }
}

#[derive(Debug, Clone)]
enum Operand {
    CONST(i64),
    OPERATOR(Rc<Operator>)
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
    operator_count: usize,
    input_count: usize,
    variables: Vec<Operand>,
    pre_computed_input_values: HashMap<(usize, i64), Vec<BTreeMap<u8, u8>>>,
    pre_computed_output_values: HashMap<usize, BTreeSet<i64>>
}

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

impl Monad {
    fn new() -> Monad {
        Monad {
            operator_count: 0,
            input_count: 0,
            variables: (0..4).map(|_| CONST(0))
                .collect::<Vec<_>>(),
            pre_computed_input_values: HashMap::new(),
            pre_computed_output_values: HashMap::new()
        }
    }
    
    fn get_output_values(&self, id: &usize) -> Option<BTreeSet<i64>> {
        if let Some(inputs) = self.pre_computed_output_values.get(&id) {
            Some(inputs.clone())
        } else {
            None
        }
    }

    fn set_output_values(&mut self, id: usize, values: BTreeSet<i64>) {
        print!("Adding {} output values for {} count is {} - sample: ", values.len(), id, self.pre_computed_output_values.len());
        println!("{:?} - {:?}", values.iter().take(3).collect::<Vec<_>>(), values.iter().rev().take(3).collect::<Vec<_>>());
        self.pre_computed_output_values.insert(id, values);
    }

    fn get_input_values(&self, operator: &Operator, value: i64) -> Option<Vec<BTreeMap<u8, u8>>> {
        if let Some(inputs) = self.pre_computed_input_values.get(&(operator.id(), value)) {
            Some(inputs.to_vec())
        } else {
            None
        }
    }

    fn set_input_values(&mut self, operator: &Operator, value: i64, values: Vec<BTreeMap<u8, u8>>) {
        println!("Adding {} input values for ({}, {}) count is {}", values.len(), operator.id(), value, self.pre_computed_input_values.len());
        self.pre_computed_input_values.insert((operator.id(), value), values);
    }

    fn parse(&mut self, file_name: &str) {
        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .for_each(|line| {
                print!("parsing {}", line);
                let id = self.parse_operator(line);
                println!(" - {}", id);
            });
    }
    fn parse_operator(&mut self, line: String) -> usize {
        let pieces = line.split_ascii_whitespace()
            .collect::<Vec<_>>();
        let a = pieces[1].chars().next().unwrap();
        let var_index = a as usize - 'w' as usize;
        if pieces[0] == "inp" {
            let input = INP(self.operator_count, self.input_count);
            self.variables[var_index] = OPERATOR(Rc::new(input));
            self.input_count += 1;
            self.operator_count += 1;
            self.operator_count - 1
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
                    CONST(Monad::binary_operator(pieces[0], a, b, 0).eval(&Vec::with_capacity(0))),
                ("mul", CONST(0), _) => CONST(0),
                ("mul", _, CONST(0)) => CONST(0),
                ("mul", CONST(1), b) => b.clone(),
                ("mul", a, CONST(1)) => a.clone(),
                ("div", CONST(0), _) => CONST(0),
                ("div", a, CONST(1)) => a.clone(),
                ("add", CONST(0), b) => b.clone(),
                ("add", a, CONST(0)) => a.clone(),
                ("eql", OPERATOR(l), CONST(r)) if (*r >= 10 || *r <= 0) && l.get_type() == "inp" => CONST(0),
                ("eql", CONST(l), OPERATOR(r)) if (*l >= 10 || *l <= 0) && r.get_type() == "inp" => CONST(0),
                _ => {
                    let id = self.operator_count;
                    self.operator_count += 1;
                    OPERATOR(Rc::new(Monad::binary_operator(pieces[0], a, b, id)))
                }
            };
            if let OPERATOR(operator) = result {
                let id = operator.id();
                self.variables[var_index] = OPERATOR(operator);
                id
            } else {
                self.variables[var_index] = result;
                usize::MAX
            }
        }
    }
    fn binary_operator(symbol: &str, a: Operand, b: Operand, id: usize) -> Operator {
        match symbol {
            "add" => ADD(id, a, b),
            "mul" => MUL(id, a, b),
            "div" => DIV(id, a, b),
            "mod" => MOD(id, a, b),
            "eql" => EQL(id, a, b),
            _ => panic!("Invalid operator: {}", symbol)
        }
    }
}

fn part_one(file_name: &str) {
    let mut monad = Monad::new();
    monad.parse(file_name);

    println!("Finding inputs...");
    let values = match monad.variables[3].clone() {
        OPERATOR(operator) => operator.find_input_values_for(&mut monad, 0),
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
