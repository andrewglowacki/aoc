use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, Component};
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

const X: usize = 0;
const M: usize = 1;
const A: usize = 2;
const S: usize = 3;
const MAX: u32 = 4000;
const MIN: u32 = 0;

#[derive(Clone)]
enum Result {
    Accept,
    Reject,
    GoTo(String),
    Next
}

#[derive(Clone)]
enum Requirement {
    Greater(usize, u32),
    Less(usize, u32)
}

enum Operation {
    Greater(usize, u32, Result),
    Less(usize, u32, Result),
    Result(Result)
}

impl Operation {
    fn string_to_component(component_str: &str) -> usize {
        match component_str {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => panic!("Unexpected character in operation string {}", component_str)
        }
    }
    fn parse_result(result: &str) -> Result {
        match result {
            "A" => Result::Accept,
            "R" => Result::Reject,
            x => Result::GoTo(x.to_owned())
        }
    }
    fn parse_greater_or_less(operation: &str, op_index: usize) -> (usize, u32, Result) {
        let label_sep = operation.find(':').unwrap();
        let component = Operation::string_to_component(&operation[0..op_index]);
        let amount = operation[op_index + 1..label_sep].parse::<u32>().unwrap();
        let result = Operation::parse_result(&operation[label_sep + 1..]);
        (component, amount, result)
    }
    fn parse(operation: &str) -> Operation {
        if let Some(index) = operation.find('>') {
            let (component, amount, goto) = Operation::parse_greater_or_less(operation, index);
            Operation::Greater(component, amount, goto)
        } else if let Some(index) = operation.find('<') {
            let (component, amount, goto) = Operation::parse_greater_or_less(operation, index);
            Operation::Less(component, amount, goto)
        } else {
            Operation::Result(Operation::parse_result(operation))
        }
    }
    fn evaluate(&self, part: &Part) -> Result {
        match self {
            Operation::Greater(component, amount, result) => {
                if part.value(*component) > *amount {
                    result.clone()
                } else {
                    Result::Next
                }
            },
            Operation::Less(component, amount, result) => {
                if part.value(*component) < *amount {
                    result.clone()
                } else {
                    Result::Next
                }
            },
            Operation::Result(result) => result.clone()
        }
    }
}

struct Workflow {
    name: String,
    steps: Vec<Operation>
}

impl Workflow {
    fn parse(line: String) -> Workflow {
        let name_end = line.find('{').unwrap();
        let name = line[0..name_end].to_owned();
        let steps = line[name_end + 1..line.len() - 1].split(',')
            .map(|operation| Operation::parse(operation))
            .collect::<Vec<_>>();

        Workflow { name, steps }
    }
    fn evaluate(&self, part: &Part) -> Result {
        for operation in &self.steps {
            let result = operation.evaluate(part);
            match result {
                Result::Next => (),
                _ => return result
            }
        }
        panic!("Hit end of steps without a result in {} for {:?}", self.name, part);
    }
}

#[derive(Clone)]
struct Range {
    from: u32,
    to: u32
}

impl Range {
    fn new(from: u32, to: u32) -> Range {
        Range { from, to }
    }
    fn full() -> Range {
        Range::new(MIN, MAX)
    }
    fn merge(&mut self, other: &Range) -> bool {
        if self.from > other.to || self.to < other.from {
            false
        } else {
            self.to = self.to.min(other.to);
            self.from = self.from.max(other.from);
            true
        }
    }
}

struct PartSpec {
    components: Vec<Range>
}

impl PartSpec {
    fn new() -> PartSpec {
        PartSpec {
            components: vec![Range::full(); 4]
        }
    }
    fn add(&mut self, component: usize, range: Range) -> bool {
        let mut current = self.components[component];
        current.merge(range)
    }
}

struct Evaluator {
    start: usize,
    series: Vec<Workflow>,
    lookup: HashMap<String, usize>,
    dest_to_source: HashMap<String, Vec<String>>
}

impl Evaluator {
    fn new(series: Vec<Workflow>) -> Evaluator {
        let mut lookup = HashMap::new();

        for i in 0..series.len() {
            let name = &series[i].name;
            lookup.insert(name.to_owned(), i);
        }

        let start = lookup["in"];
        let mut dest_to_source = HashMap::<String, Vec<String>>::new();

        for workflow in series.iter() {
            workflow.steps.iter()
                .flat_map(|step| {
                    match step {
                        Operation::Result(Result::GoTo(dest)) => Some(dest),
                        Operation::Less(_, _, Result::GoTo(dest)) => Some(dest),
                        Operation::Greater(_, _, Result::GoTo(dest)) => Some(dest),
                        _ => None
                    }
                })
                .map(String::to_owned)
                .for_each(|dest| {
                    let name = workflow.name.to_owned();
                    if let Some(sources) = dest_to_source.get_mut(&dest) {
                        sources.push(name);
                    } else {
                        dest_to_source.insert(dest, vec![name]);
                    }
                });
        }


        Evaluator { series, lookup, start, dest_to_source }
    }

    fn accept(&self, part: &Part) -> bool {
        let mut current = &self.series[self.start];
        loop {
            let next = match current.evaluate(part) {
                Result::Accept => return true,
                Result::Reject => return false,
                Result::GoTo(label) => label,
                Result::Next => panic!("Next is not applicable here")
            };
            current = &self.series[self.lookup[&next]];
        }
    }

    fn gather_accept_requirements(
        &self, 
        workflow: &String, 
        requirements: &mut Vec<Requirement>, 
        acceptable_specs: &mut Vec<PartSpec>) 
    {
        let workflow = *self.lookup.get(workflow).unwrap();
        let workflow = &self.series[workflow];
        for step in workflow.steps.iter() {
            let orig_len = requirements.len();
            match step {
                Operation::Greater(component, amount, result) => {
                    requirements.push(Requirement::Greater(*component, *amount));
                    match result {
                        Result::Accept => acceptable_specs.push(Evaluator::merge_requirements(requirements)),
                        Result::Reject => (),
                        Result::GoTo(next) => self.gather_accept_requirements(next, requirements, acceptable_specs),
                        _ => panic!("Invalid result encountered")
                    }
                    requirements.push(Requirement::Less(*component, *amount + 1));
                },
                Operation::Less(component, amount, result) => {
                    requirements.push(Requirement::Less(*component, *amount));
                    match result {
                        Result::Accept => acceptable_specs.push(Evaluator::merge_requirements(requirements)),
                        Result::Reject => (),
                        Result::GoTo(next) => self.gather_accept_requirements(next, requirements, acceptable_specs),
                        _ => panic!("Invalid result encountered")
                    }
                    while requirements.len() > orig_len {
                        requirements.pop().unwrap();
                    }
                    requirements.push(Requirement::Greater(*component, *amount - 1));
                },
                Operation::Result(result) => {
                    match result {
                        Result::Accept => acceptable_specs.push(Evaluator::merge_requirements(requirements)),
                        Result::Reject => (),
                        Result::GoTo(next) => self.gather_accept_requirements(next, requirements, acceptable_specs),
                        _ => panic!("Invalid result encountered")
                    }
                    return;
                }
            }
        }
    }

    fn merge_requirements(requirements: &Vec<Requirement>) -> Option<PartSpec> {
        let mut spec = PartSpec::new();

        for requirement in requirements {
            let (component, range) = match requirement {
                Requirement::Greater(component, amount) => {
                    (component, Range::new(amount, MAX))
                },
                Requirement::Less(component, amount) => {
                    (component, Range::new(MIN, amount))
                }
            };

            if !spec.merge(component, range) {
                return None;
            }

        }
        Some(spec)
    }

    fn count_acceptable_parts(&self) -> u64 {
        let mut acceptable_specs = Vec::new();
        let mut requirements = Vec::new();
        self.gather_accept_requirements(&"in".to_owned(), &mut requirements, &mut acceptable_specs);
        0
    }
}

#[derive(Debug)]
struct Part {
    components: Vec<u32>
}

impl Part {
    fn parse(line: String) -> Part {
        let components = line[1..line.len() - 1]
            .split(',')
            .map(|piece| {
                let key_value = piece.split('=').collect::<Vec<_>>();
                (key_value[0], key_value[1].parse::<u32>().unwrap())
            })
            .collect::<HashMap<&str, u32>>();
        
        let components = vec![
            components["x"],
            components["m"],
            components["a"],
            components["s"]
        ];

        Part { components }
    }
    fn value(&self, index: usize) -> u32 {
        self.components[index]
    }

    fn sum(&self) -> u32 {
        self.components.iter().sum::<u32>()
    }
}

fn parse_workflows_and_parts(file_name: &str) -> (Evaluator, Vec<Part>) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());

    let workflows = (&mut lines).take_while(|line| !line.is_empty())
        .map(Workflow::parse)
        .collect::<Vec<_>>();
    
    let parts = lines.map(Part::parse)
        .collect::<Vec<_>>();

    (Evaluator::new(workflows), parts)
}

fn part_one(file_name: &str) {
    let (evaluator, parts) = parse_workflows_and_parts(file_name);

    let total = parts.iter()
        .filter(|part| evaluator.accept(part))
        .map(|part| part.sum())
        .sum::<u32>();

    println!("Part 1: {}", total);
}

fn part_two(file_name: &str) {
    let (evaluator, _) = parse_workflows_and_parts(file_name);
    let acceptable = evaluator.count_acceptable_parts();
    println!("Part 2: {}", acceptable);
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");

    println!("Done!");
}
