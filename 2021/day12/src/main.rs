use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Cave {
    label: String,
    id: usize,
    big: bool,
    adjacent: Vec<usize>
}

impl Cave {
    fn new(id: usize, label: String) -> Cave {
        let first = label.chars().next().unwrap();
        let big = first.is_uppercase();

        Cave {
            id,
            label,
            big,
            adjacent: Vec::new()
        }
    }

    fn is_end(&self) -> bool {
        self.label == "end"
    }
    
    fn is_start(&self) -> bool {
        self.id == 0
    }
}

struct Network {
    caves: Vec<Cave>,
    label_to_id: HashMap<String, usize>
}

struct TravelLog {
    visited: HashSet<usize>,
    path: Vec<usize>,
    paths: Vec<Vec<usize>>,
    visited_twice: Option<usize>,
}

impl TravelLog {
    fn new(allow_two_visits: bool) -> TravelLog {
        let mut visited = HashSet::new();
        visited.insert(0);

        TravelLog {
            visited,
            path: Vec::new(),
            paths: Vec::new(),
            visited_twice: match allow_two_visits {
                true => None,
                false => Some(usize::MAX)
            }
        }
    }
    fn can_visit(&mut self, cave: &Cave) -> bool {
        if cave.is_start() {
            false
        } else if cave.big || self.visited.insert(cave.id) {
            true
        } else if self.visited_twice.is_none() {
            self.visited_twice = Some(cave.id);
            true
        } else {
            false
        }
    }
    fn un_visit(&mut self, cave: &Cave) {
        if !cave.big {
            if let Some(twice_id) = self.visited_twice {
                if twice_id == cave.id {
                    self.visited_twice = None;
                } else {
                    self.visited.remove(&cave.id);
                }
            } else {
                self.visited.remove(&cave.id);
            }
        }
    }
    fn found_end(&mut self) {
        let path = self.path.to_vec();
        self.paths.push(path);
    }
}

impl Network {
    fn create(&mut self, label: &str) {
        if !self.label_to_id.contains_key(label) {
            let id = self.caves.len();
            let cave = Cave::new(id, label.to_owned());
            self.label_to_id.insert(cave.label.clone(), id);
            self.caves.push(cave);
        }
    }

    fn add_adjacent(&mut self, label: &str, adjacent: &str) {
        let id = self.label_to_id.get(label).unwrap();
        let adj_id = self.label_to_id.get(adjacent).unwrap();
        let cave = &mut self.caves[*id];
        cave.adjacent.push(*adj_id);
    }

    fn from_file(file_name: &str) -> Network {
        let mut network = Network {
            caves: Vec::new(),
            label_to_id: HashMap::new()
        };

        // make sure start is first
        network.create("start");

        // populate the network
        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .for_each(|line| {
                let mut pieces = line.split("-");
                let from = pieces.next().unwrap();
                let to = pieces.next().unwrap();

                network.create(from);
                network.create(to);

                network.add_adjacent(from, to);
                network.add_adjacent(to, from);
            });
        
        network
    }

    fn find_end(&self, cave: &Cave, log: &mut TravelLog) {
        log.path.push(cave.id);
        for id in &cave.adjacent {
            let to_visit = &self.caves[*id];
            if !log.can_visit(to_visit) {
                continue;
            }
            if to_visit.is_end() {
                log.found_end();
            } else {
                self.find_end(to_visit, log);
            }
            log.un_visit(&to_visit);
        }
        log.path.pop();
    }

    fn start(&self) -> &Cave {
        &self.caves[0]
    }

    fn _print(&self) {
        for cave in &self.caves {
            self._print_cave(&cave);
        }
    }

    fn _print_cave(&self, cave: &Cave) {
        println!("{}: {} [", cave.id, cave.label);
        for id in &cave.adjacent {
            let label = &self.caves[*id].label;
            println!("  {}: {}", id, label);
        }
        println!("]");
    }
}

fn part_one(file_name: &str) {
    let network = Network::from_file(file_name);

    let start = network.start();
    let mut log = TravelLog::new(false);
    network.find_end(start, &mut log);
    
    println!("Part 1: {}", log.paths.len());
}

fn part_two(file_name: &str) {
    let network = Network::from_file(file_name);

    let start = network.start();
    let mut log = TravelLog::new(true);
    network.find_end(start, &mut log);
    
    println!("Part 2: {}", log.paths.len());
}

fn main() {
    part_one("input.txt");

    part_two("sample.txt");
    part_two("sample2.txt");
    part_two("sample3.txt");
    part_two("input.txt");

    println!("Done!");
}
