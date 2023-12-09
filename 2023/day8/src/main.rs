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

    fn get_distance_to_end(&self, start: usize, end: usize) -> usize {
        let mut index = 0;
        let mut current = start;
        while current != end {
            let (left, right) = self.nodes.get(&current).unwrap();
            let next = match self.turns[index % self.turns.len()] {
                Turn::Left => left,
                Turn::Right => right
            };
            current = *next;
            index += 1;
        }
        index
    }

}

fn part_one(file_name: &str) {
    let map = Map::parse(file_name);
    let start = map.lookup.get("AAA").unwrap();
    let end = map.lookup.get("ZZZ").unwrap();
    let count = map.get_distance_to_end(*start, *end);
    println!("Part 1: {}", count);
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");

    println!("Done!");
}
