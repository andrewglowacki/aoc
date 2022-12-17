use std::collections::{HashMap, HashSet, BTreeMap};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct IdSource {
    ids: HashMap<String, usize>
}

impl IdSource {
    fn new() -> IdSource {
        IdSource {
            ids: HashMap::new()
        }
    }
    fn get(&mut self, name: &str) -> usize {
        if let Some(id) = self.ids.get(name) {
            *id
        } else {
            self.ids.insert(name.to_owned(), self.ids.len());
            self.ids.len() - 1
        }
    }
}

struct Valve {
    id: usize,
    rate: u32,
    tunnels: Vec<usize>
}

impl Valve {
    fn parse(line: String, ids: &mut IdSource) -> Valve {
        let pieces = line.split(" ").collect::<Vec<_>>();

        let id = ids.get(pieces[1]);

        let rate = pieces[4];
        let rate = rate[5..rate.len() - 1].parse::<u32>().unwrap();

        let tunnels = (9..pieces.len())
            .map(|i| pieces[i])
            .map(|piece| match piece.len() == 2 {
                true => piece.to_owned(),
                false => piece[0..piece.len() - 1].to_owned()
            })
            .map(|name| ids.get(name.as_str()))
            .collect::<Vec<_>>();

        Valve { 
            id,
            rate,
            tunnels
        }
    }
}

struct Network {
    valves: HashMap<usize, Valve>,
    valves_by_rate: BTreeMap<u32, Vec<usize>>,
    distances: HashMap<(usize, usize), u32>,
    ids: IdSource
}

impl Network {
    fn new() -> Network {
        Network {
            valves: HashMap::new(),
            valves_by_rate: BTreeMap::new(),
            distances: HashMap::new(),
            ids: IdSource::new()
        }
    }

    fn add_valve(&mut self, line: String) {
        let valve = Valve::parse(line, &mut self.ids);
        let rate = valve.rate;
        let id = valve.id.to_owned();
        self.valves.insert(valve.id, valve);
        
        if let Some(valves) = self.valves_by_rate.get_mut(&rate) {
            valves.push(id);
        } else {
            self.valves_by_rate.insert(rate, vec![id]);
        }
    }

    fn compute_distances(&mut self) {
        let valves = self.valves.keys().collect::<Vec<_>>();
        for i in 0..valves.len() - 1 {
            let from = valves[i];
            for j in i + 1..valves.len() {
                let to = valves[j];
                let distance = self.get_distance_to(*from, *to);
                self.distances.insert((from.to_owned(), to.to_owned()), distance);
                self.distances.insert((to.to_owned(), from.to_owned()), distance);
            }
        }
    }

    fn get_distance_to(&self, from: usize, to: usize) -> u32 {
        let mut visited = HashSet::<usize>::new();
        visited.insert(from);
        
        let mut options = HashSet::<usize>::new();

        self.valves.get(&from)
            .unwrap().tunnels
            .iter()
            .for_each(|valve| {
                options.insert(*valve);
            });
        
        let mut distance = 0;

        while options.len() > 0 {
            let mut next_options = HashSet::new();
            distance += 1;
            for option in options {
                if option == to {
                    return distance;
                } else if visited.insert(option) {
                    self.valves.get(&option)
                        .unwrap()
                        .tunnels
                        .iter()
                        .filter(|option| !visited.contains(option))
                        .for_each(|option| {
                            next_options.insert(*option);
                        });
                }
            }
            options = next_options;
        }

        panic!("Shortest path not found from {} to {}", from, to);
    }

    fn determine_max_pressure(&self) -> u32 {
        let mut opened = HashSet::<usize>::new();
        let mut max = 0;

        let start = *self.ids.ids.get("AA").unwrap();

        for to in self.valves.keys() {
            let this_max = self.max_pressure_from(start, *to, 30, &mut opened);
            max = max.max(this_max);
        }

        max
    }

    fn max_pressure_from(&self, from: usize, to: usize, minutes: u32, opened: &mut HashSet<usize>) -> u32 {
        let mut minutes = minutes;
        if from == to {
            minutes -= 1;
        } else {
            let cost_to_open = self.distances.get(&(from, to)).unwrap() + 1;
            if cost_to_open > minutes {
                return 0;
            } else {
                minutes -= cost_to_open;
            }
        }

        let added = self.valves.get(&to).unwrap().rate * minutes;
        if minutes == 1 {
            return added;
        }

        let mut next_max = 0;

        opened.insert(to);
        for next_to in self.valves.keys() {
            if opened.contains(next_to) {
                continue;
            }
            let this_max = self.max_pressure_from(to, *next_to, minutes, opened);
            next_max = next_max.max(this_max);
        }
        opened.remove(&to);

        added + next_max
    }

}

fn part_one(file_name: &str) {
    let mut network = Network::new();

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .for_each(|line| network.add_valve(line));
    
    network.compute_distances();

    let max = network.determine_max_pressure();

    println!("Part 1: {}", max);
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
