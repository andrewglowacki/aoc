use std::collections::HashMap;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

const MODEL_DIGITS: usize = 14;

#[derive(Debug)]
enum Operator {
    INP(usize, usize),
    ADD(usize, Operand, Operand),
    MUL(usize, Operand, Operand),
    DIV(usize, Operand, Operand),
    MOD(usize, Operand, Operand),
    EQL(usize, Operand, Operand)
}

#[derive(Clone)]
enum RefOperand {
    RefConst(i64),
    Ref(usize)
}

#[derive(Clone)]
enum RefOperator {
    RefInp(usize),
    RefAdd(RefOperand, RefOperand),
    RefMul(RefOperand, RefOperand),
    RefDiv(RefOperand, RefOperand),
    RefMod(RefOperand, RefOperand),
    RefEql(RefOperand, RefOperand)
}

use RefOperand::*;
use RefOperator::*;

struct Deconstruction {
    operators: Vec<RefOperator>
}

impl Deconstruction {
    fn new(operators: BTreeMap<usize, RefOperator>) -> Deconstruction {
        Deconstruction {
            operators: operators.values().cloned().collect::<Vec<_>>()
        }
    }
    fn execute<F>(results: &Vec<BTreeSet<i64>>, a: &RefOperand, b: &RefOperand, combiner: F) -> BTreeSet<i64> 
        where F: Fn(i64, i64) -> i64 {
        match (a, b) {
            (RefConst(c), Ref(r)) | (Ref(r), RefConst(c)) => {
                results[*r].iter().map(|v| combiner(*v, *c))
                    .collect::<BTreeSet<_>>()
            },
            (Ref(a), Ref(b)) => {
                let mut my_results = BTreeSet::new();
                for a_val in results[*a].iter() {
                    for b_val in results[*b].iter() {
                        my_results.insert(combiner(*a_val, *b_val));
                    }
                }
                my_results
            },
            _ => panic!("Invalid combo")
        }
    }
    fn evaluate(&self) -> Vec<i64> {
        let mut results = Vec::<BTreeSet<i64>>::new();
        let mut values = 0;
        for operator in self.operators.iter() {
            println!("Executing {} of {} with {} values thus far", results.len() + 1, self.operators.len(), values);
            let these_results = match operator {
                RefInp(_) => {
                    (0..10).into_iter().skip(1)
                        .collect::<BTreeSet<_>>()
                },
                RefAdd(a, b) => Deconstruction::execute(&results, a, b, |a, b| a + b),
                RefMul(a, b) => Deconstruction::execute(&results, a, b, |a, b| a * b),
                RefDiv(a, b) => Deconstruction::execute(&results, a, b, |a, b| a / b),
                RefMod(a, b) => Deconstruction::execute(&results, a, b, |a, b| a % b),
                RefEql(a, b) => Deconstruction::execute(&results, a, b, |a, b| match a == b { true => 1, false => 0 })
            };
            values += these_results.len();
            results.push(these_results);
        }

        self.find_greatest_inputs(results)
    }

    fn retain_results_match<F>(my_id: usize, results: &mut Vec<BTreeSet<i64>>, a: &RefOperand, b: &RefOperand, combiner: F)
        where F: Fn(i64, i64) -> i64 
    {
        match (a, b) {
            (RefConst(c), Ref(r)) | (Ref(r), RefConst(c)) => {
                let my_results = &results[my_id];
                let new_results = results[*r].iter().filter(|value| {
                    let combined = combiner(**value, *c);
                    my_results.contains(&combined)
                })
                .copied()
                .collect::<BTreeSet<_>>();
                results[*r] = new_results;
            },
            (Ref(a), Ref(b)) => {
                let my_results = &results[my_id];
                let a_results = &results[*a];
                let b_results = &results[*b];

                let new_a_results = a_results.iter().filter(|a_val| {
                    b_results.iter().find(|b_val| {
                        my_results.contains(&combiner(**a_val, **b_val))
                    })
                    .is_some()
                })
                .copied()
                .collect::<BTreeSet<_>>();

                let new_b_results = b_results.iter().filter(|b_val| {
                    a_results.iter().find(|a_val| {
                        my_results.contains(&combiner(**a_val, **b_val))
                    })
                    .is_some()
                })
                .copied()
                .collect::<BTreeSet<_>>();

                println!("For {} reduced a {} from {} to {}", my_id, a, results[*a].len(), new_a_results.len());
                println!("For {} reduced b {} from {} to {}", my_id, b, results[*b].len(), new_b_results.len());
                results[*a] = new_a_results;
                results[*b] = new_b_results;
            },
            _ => panic!("Invalid combo")
        }
    }

    fn find_greatest_inputs(&self, mut results: Vec<BTreeSet<i64>>) -> Vec<i64> {
        let mut index = results.len() - 1;
        assert_eq!(true, results[index].contains(&0));
        results[index].clear();
        results[index].insert(0);
        let mut inputs = Vec::new();
        for operator in self.operators.iter().rev() {
            println!("Retaining results for {}", index);
            match operator {
                RefInp(_) => { inputs.push(*results[index].iter().max().unwrap()); },
                RefAdd(a, b) => Deconstruction::retain_results_match(index, &mut results, a, b, |a, b| a + b),
                RefMul(a, b) => Deconstruction::retain_results_match(index, &mut results, a, b, |a, b| a * b),
                RefDiv(a, b) => Deconstruction::retain_results_match(index, &mut results, a, b, |a, b| a / b),
                RefMod(a, b) => Deconstruction::retain_results_match(index, &mut results, a, b, |a, b| a % b),
                RefEql(a, b) => Deconstruction::retain_results_match(index, &mut results, a, b, |a, b| match a == b { true => 1, false => 0 })
            }
            index -= 1;
        }
        inputs.reverse();
        inputs
    }
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
    fn deconstruct(&self, output: &mut BTreeMap<usize, RefOperator>) {
        if !output.contains_key(&self.id()) {
            let (operator, a, b) = match self {
                INP(_, index) => (RefInp(*index), CONST(0), CONST(0)),
                ADD(_, a, b) => (RefAdd(a.to_ref(), b.to_ref()), a.clone(), b.clone()),
                MUL(_, a, b) => (RefMul(a.to_ref(), b.to_ref()), a.clone(), b.clone()),
                DIV(_, a, b) => (RefDiv(a.to_ref(), b.to_ref()), a.clone(), b.clone()),
                MOD(_, a, b) => (RefMod(a.to_ref(), b.to_ref()), a.clone(), b.clone()),
                EQL(_, a, b) => (RefEql(a.to_ref(), b.to_ref()), a.clone(), b.clone()),
            };
            output.insert(self.id(), operator);

            if let OPERATOR(a) = a {
                a.deconstruct(output);
            }
            if let OPERATOR(b) = b {
                b.deconstruct(output);
            }
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
    fn to_ref(&self) -> RefOperand {
        match self {
            CONST(value) => RefConst(*value),
            OPERATOR(operator) => Ref(operator.id())
        }
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
                .collect::<Vec<_>>()
        }
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

    let deconstruction = if let OPERATOR(operator) = &monad.variables[3] {
        let mut output = BTreeMap::new();
        operator.deconstruct(&mut output);
        Deconstruction::new(output)
    } else {
        panic!("Invalid operator variable");
    };

    let results = deconstruction.evaluate();

    println!("Part 1: {:?}", results);
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
