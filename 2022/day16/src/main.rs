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
    ids: HashMap<String, u64>
}

impl IdSource {
    fn new() -> IdSource {
        IdSource {
            ids: HashMap::new()
        }
    }
    fn get(&mut self, name: &str) -> u64 {
        if let Some(id) = self.ids.get(name) {
            *id
        } else {
            let id = 1 << self.ids.len();
            self.ids.insert(name.to_owned(), id);
            id
        }
    }
}

struct Valve {
    id: u64,
    rate: u32,
    tunnels: Vec<u64>
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
    valves: HashMap<u64, Valve>,
    valves_by_rate: BTreeMap<u32, Vec<u64>>,
    distances: HashMap<(u64, u64), u32>,
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
                let distance = self.compute_distance_to(*from, *to);
                self.distances.insert((from.to_owned(), to.to_owned()), distance);
                self.distances.insert((to.to_owned(), from.to_owned()), distance);
            }
        }
    }

    fn get_distance(&self, from: u64, to: u64) -> u32 {
        if from == to {
            0
        } else {
            *self.distances.get(&(from, to)).unwrap()
        }
    }

    fn compute_distance_to(&self, from: u64, to: u64) -> u32 {
        let mut visited = HashSet::<u64>::new();
        visited.insert(from);
        
        let mut options = HashSet::<u64>::new();

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
        let mut max = 0;

        let start = *self.ids.ids.get("AA").unwrap();

        for to in self.valves.keys() {
            let distance = self.get_distance(start, *to);
            if distance + 1 >= 30 {
                continue;
            }
            let this_max = self.max_pressure_from(*to, distance, 30, 0, max, 0);
            max = max.max(this_max);
        }

        max
    }

    fn max_pressure_from(
        &self, 
        to: u64, 
        distance: u32,
        minutes: u32, 
        pressure_thus_far: u32,
        max_thus_far: u32,
        opened: u64) -> u32 
    {
        let minutes = minutes - (distance + 1);

        let added = self.valves.get(&to).unwrap().rate * minutes;
        if minutes == 1 {
            return added;
        }

        let pressure_thus_far = pressure_thus_far + added;
        let mut max_thus_far = max_thus_far.max(pressure_thus_far);
        let mut final_pressure_thus_far = pressure_thus_far;

        let opened = opened | to;
        self.valves_by_rate.iter()
            .rev()
            .flat_map(|(_, values)| values)
            .filter(|next_to| opened & *next_to == 0)
            .for_each(|next_to| {
                let distance = self.get_distance(to, *next_to);
                if distance + 1 < minutes {
                    let this_max = self.max_pressure_from(*next_to, distance, minutes, pressure_thus_far, max_thus_far, opened);
                    final_pressure_thus_far = final_pressure_thus_far.max(this_max);
                    max_thus_far = max_thus_far.max(final_pressure_thus_far);
                }
            });

        final_pressure_thus_far
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
