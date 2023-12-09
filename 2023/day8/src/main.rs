use std::char::REPLACEMENT_CHARACTER;
use std::collections::{HashMap, BTreeMap, BTreeSet};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

enum Turn {
    Left,
    Right
}

struct Map {
    lookup: HashMap<String, usize>,
    turns: Vec<Turn>,
    nodes: HashMap<usize, (usize, usize)>
}

fn to_id(lookup: &mut HashMap<String, usize>, id_str: &str) -> usize {
    if let Some(id) = lookup.get(id_str) {
        *id
    } else {
        let next = lookup.len();
        lookup.insert(id_str.to_owned(), next);
        next
    }
}

fn parse_node(lookup: &mut HashMap<String, usize>, line: String) -> (usize, (usize, usize)) {
    let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
    let from = to_id(lookup, parts[0]);
    let left = to_id(lookup, &parts[2][1..4]);
    let right = to_id(lookup, &parts[3][0..3]);
    (from, (left, right))
}

impl Map {
    fn parse(file_name: &str) -> Map {
        let mut lines = get_file_lines(file_name)
            .flat_map(|line| line.ok());

        let turns = lines.next()
            .unwrap()
            .chars()
            .map(|c| match c {
                'L' => Turn::Left,
                'R' => Turn::Right,
                x => panic!("Unexpected turn character: {}", x)
            })
            .collect::<Vec<_>>();

        let mut lookup = HashMap::<String, usize>::new();

        let nodes = lines.skip(1)
            .map(|line| parse_node(&mut lookup, line))
            .collect::<HashMap<_,_>>();

        Map { lookup, nodes, turns }
    }

    fn get_distance_to_end(&self, start: usize, end: usize, mut index: u64) -> Option<u64> {
        let start_index = index;
        let mut current = start;
        let num_turns = self.turns.len() as u64;
        let limit_end  = if index > 0 {
            index * 2
        } else {
            u64::MAX
        };
        while current != end && index <= limit_end {
            let (left, right) = self.nodes.get(&current).unwrap();
            let next = match self.turns[(index % num_turns) as usize] {
                Turn::Left => left,
                Turn::Right => right
            };
            current = *next;
            index += 1;
        }
        if current != end {
            None
        } else {
            Some(index - start_index)
        }
    }

    fn find_repeat(&self, start: usize) -> u64 {
        let mut index = 0;
        let num_turns = self.turns.len() as u64;
        let mut last_seen = HashMap::<usize, u64>::new();
        let mut current = start;
        let give_up = num_turns * num_turns;
        while index < give_up {
            let (left, right) = self.nodes.get(&current).unwrap();
            let next = match self.turns[(index % num_turns) as usize] {
                Turn::Left => left,
                Turn::Right => right
            };
            index += 1;
            current = *next;
            if let Some(last_index) = last_seen.get_mut(next) {
                if (index - *last_index) % num_turns == 0 {
                    return *last_index;
                } else {
                    *last_index = index;
                }
            } else {
                last_seen.insert(*next, index);
            }
        }

        panic!("No repeat found for {}", start);
    }

}

fn part_one(file_name: &str) {
    let map = Map::parse(file_name);
    let start = map.lookup.get("AAA").unwrap();
    let end = map.lookup.get("ZZZ").unwrap();
    let count = map.get_distance_to_end(*start, *end, 0);
    println!("Part 1: {}", count.unwrap());
}

fn part_two(file_name: &str) {
    let map = Map::parse(file_name);
    
    let starts = map.lookup.iter()
        .filter(|(name, _)| name.ends_with('A'))
        .map(|(_, id)| *id)
        .collect::<Vec<_>>();

    let repeats = starts.iter()
        .map(|start| map.find_repeat(*start))
        .collect::<Vec<_>>();

    let mut stage_one = Vec::<u64>::new();
    for i in (0..6).step_by(2) {
        let mut left = repeats[i];
        let left_add = left;
        let mut right = repeats[i + 1];
        let right_add = right;
    
        while left != right {
            if left < right {
                left += left_add;
            } else {
                right += right_add;
            }
        }
        stage_one.push(left);
        println!("{}", left);
    }
    
    let mut stage_two = Vec::<u64>::new();
    for i in 0..2 {
        let mut left = stage_one[i];
        let left_add = left;
        let mut right = stage_one[i + 1];
        let right_add = right;
    
        while left != right {
            if left < right {
                left += left_add;
            } else {
                right += right_add;
            }
        }
        println!("{}", left);
        stage_two.push(left);
    }

    let mut left = stage_two[0];
    let left_add = left;
    let mut right = stage_two[1];
    let right_add = right;

    while left != right {
        if left < right {
            left += left_add;
        } else {
            right += right_add;
        }
    }
    println!("{}", left);

    // 16343x = z
    // 20221y = z
    // 16343x - 20221y = 0
    
    println!("{:?}", repeats);

    // println!("Part 2: {}", turns.first().unwrap().0);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
