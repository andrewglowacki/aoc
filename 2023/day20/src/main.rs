use std::collections::HashMap;
use std::fs::File;
use std::mem::swap;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(PartialEq)]
enum Function {
    Broadcast,
    FlipFlip,
    Conjunction,
    DeadEnd
}

struct Module {
    function: Function,
    destination_names: Vec<String>,
    destination_ids: Vec<usize>,
    name: String,
    id: usize,
    inputs: usize
}

impl Module {
    fn parse(line: String, id: usize) -> Module {
        let parts = line.split(" -> ").collect::<Vec<_>>();

        let destination_names = parts[1].split(", ")
            .map(|line| line.to_owned())
            .collect::<Vec<_>>();

        let (name, function) = match &parts[0][0..1] {
            "&" => (parts[0][1..].to_owned(), Function::Conjunction),
            "%" => (parts[0][1..].to_owned(), Function::FlipFlip),
            "b" => (parts[0].to_owned(), Function::Broadcast),
            _ => panic!("unexpected first character of name: {}", parts[0])
        };

        Module { 
            id, 
            name, 
            function, 
            destination_names, 
            destination_ids: Vec::new(),
            inputs: 0
        }
    }

    fn dead_end(name: String, id: usize) -> Module {
        Module {
            id,
            name,
            function: Function::DeadEnd,
            destination_names: Vec::new(),
            destination_ids: Vec::new(),
            inputs: 0
        }
    }

    fn send_pulse(&self, state: &mut State, pulse: bool) {
        self.destination_ids.iter().for_each(|id| {
            state.outgoing.push((self.id, *id, pulse));
        });
        // println!("{} --{}--> {:?}", self.name, pulse, self.destination_names);
        if pulse {
            state.high_pulses += self.destination_ids.len();
        } else {
            state.low_pulses += self.destination_ids.len();
        }
    }

    fn process(&self, state: &mut State, id: usize, high: bool) {
        if !high {
            state.low_pulse_count[self.id] += 1;
        }
        match self.function {
            Function::FlipFlip => {
                let current = state.flip_state[self.id];
                if !high {
                    let new_state = !current;
                    state.flip_state[self.id] = new_state;
                    self.send_pulse(state, new_state);
                }
            },
            Function::Conjunction => {
                let (assigned, count) = &mut state.conj_state[self.id];
                let mask = 1 << id;
                let prev_high = *assigned & mask == mask;
                if prev_high != high {
                    if high {
                        *assigned |= mask;
                        *count += 1;
                    } else {
                        *assigned ^= mask;
                        *count -= 1;
                    }
                }
                let count = *count;
                self.send_pulse(state, count != self.inputs);
            },
            Function::DeadEnd => (),
            _ => panic!("unexpected function")
        }
    }
}

struct State {
    low_pulses: usize,
    high_pulses: usize,
    flip_state: Vec<bool>,
    conj_state: Vec<(u64, usize)>,
    outgoing: Vec<(usize, usize, bool)>,
    outgoing_swap: Vec<(usize, usize, bool)>,
    low_pulse_count: Vec<usize>
}

impl State {
    fn new(modules: usize) -> State {
        State {
            low_pulses: 0,
            high_pulses: 0,
            flip_state: vec![false; modules],
            conj_state: vec![(0, 0); modules],
            low_pulse_count: vec![0; modules],
            outgoing: Vec::new(),
            outgoing_swap: Vec::new()
        }
    }
}

struct Network {
    modules: Vec<Module>,
    broadcast_ids: Vec<usize>,
    broadcaster: usize
}

impl Network {
    fn parse(file_name: &str) -> Network {
        let mut modules = Vec::new();
        let mut name_to_id = HashMap::new();

        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .for_each(|line| {
                let id = modules.len();
                let module = Module::parse(line, modules.len());
                name_to_id.insert(module.name.to_owned(), id);
                modules.push(module);
            });

        let missing = modules.iter()
            .flat_map(|module| module.destination_names.iter())
            .filter(|name| !name_to_id.contains_key(*name))
            .map(|name| name.to_owned())
            .collect::<Vec<_>>();

        missing.into_iter().for_each(|name| {
            let id = modules.len();
            let module = Module::dead_end(name, id);
            name_to_id.insert(module.name.to_owned(), id);
            modules.push(module);
        });

        let mut input_counts = HashMap::<usize, usize>::new();

        modules.iter_mut().for_each(|module| {
            module.destination_ids = 
                module.destination_names.iter()
                    .map(|name| name_to_id[name])
                    .collect::<Vec<_>>();
            
            module.destination_ids.iter().for_each(|id| {
                if let Some(count) = input_counts.get_mut(id) {
                    *count += 1;
                } else {
                    input_counts.insert(*id, 1);
                }
            });
        });

        for (id, count) in input_counts {
            let module = &mut modules[id];
            module.inputs = count;
        }

        let broadcaster = modules.iter()
            .find(|module| module.function == Function::Broadcast)
            .unwrap();

        let broadcast_ids = broadcaster.destination_ids.to_vec();
        let broadcaster = broadcaster.id;

        Network { modules, broadcast_ids, broadcaster: broadcaster }
    }

    fn push_button(&self, state: &mut State) {
        state.low_pulses += 1 + self.broadcast_ids.len();

        self.broadcast_ids.iter()
            .map(|id| (self.broadcaster, *id, false))
            .for_each(|outgoing| state.outgoing.push(outgoing));

        while !state.outgoing.is_empty() {
            swap(&mut state.outgoing, &mut state.outgoing_swap);
            while let Some((from, to, high)) = state.outgoing_swap.pop() {
                let module = &self.modules[to];
                module.process(state, from, high);
            }
        }
    }
    
    fn push_button_multi(&self, times: usize) -> usize {
        let mut state = State::new(self.modules.len());
        for _ in 0..times {
            self.push_button(&mut state);
        }
        state.low_pulses * state.high_pulses
    }
}

fn part_one(file_name: &str) {
    let network = Network::parse(file_name);
    let count = network.push_button_multi(1000);
    println!("Part 1: {}", count);
}

fn part_two(file_name: &str) {
    let network = Network::parse(file_name);
    let wait_for_id = network.modules.iter()
        .find(|module| module.name == "rx")
        .unwrap()
        .id;

    let mut state = State::new(network.modules.len());
    let mut pushes = 0;
    while state.low_pulse_count[wait_for_id] == 0 {
        network.push_button(&mut state);
        pushes += 1;
    }
    println!("Part 2: {}", pushes);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
