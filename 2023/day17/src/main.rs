use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Map {
    blocks: Vec<Vec<u32>>
}

#[derive(Ord, Eq, PartialEq, PartialOrd)]
struct Step {
    heat_loss: u32,
    point: (i32, i32),
    prev: (i32, i32),
    straight_count: usize,
}

impl Step {
    fn start() -> Step {
        let mut step = Step {
            heat_loss: 0,
            point: (1, 1),
            prev: (0, 1),
            straight_count: 0
        };
        step
    }
    fn is_straight(&self, (x, y): (i32, i32)) -> bool {
        let (prev_x, prev_y) = self.prev;
        (prev_x - x) == 0 || (prev_y - y) == 0
    }
    fn visit(&self, map: &Map, visited: &mut HashSet<(i32, i32)>, candidates: &mut BTreeSet<Step>) {
        let (x, y) = self.point;

        let neighbors = vec![
            (x + 1, y),
            (x - 1, y),
            (x, y - 1),
            (x, y + 1)
        ];

        neighbors.into_iter()
            .map(|(x, y)| (x, y, self.is_straight((x, y))))
            .filter(|(_, _, is_straight)| self.straight_count < 3 || !is_straight)
            .filter(|(x, y, _)| !visited.contains(&(x, y)))
            .map(|(x, y, is_straight)|)
    }
}

impl Map {
    fn parse(file_name: &str) -> Map {
        let mut blocks = Vec::new();
        blocks.push(Vec::new());

        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| {
                let mut row = Vec::new();
                row.push(u32::MAX);
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .for_each(|heat| row.push(heat));
                row.push(u32::MAX);
                row
            })
            .for_each(|row| blocks.push(row));

        blocks.push(vec![u32::MAX; blocks[1].len()]);
        blocks[0] = blocks[blocks.len() - 1].to_vec();
        
        // blocks have a border around them of u32::MAX 
        // so we don't have to deal with negatives
        Map { blocks }
    }

    fn add_neighbors(&self, step: Step, visited: &mut HashSet<(usize, usize)>, candidates: &mut BTreeSet<Step>) {
    }

    fn find_min_heat_loss(&self) -> u32 {
        let mut candidates = BTreeSet::<Step>::new();
        candidates.insert(Step::start());

        let end_y = self.blocks.len() - 2;
        let end_x = self.blocks[0].len() - 2;

        let mut visited = HashSet::<(i32, i32)>::new();

        while let Some(step) = candidates.pop_first() {
            
        }

        panic!("end not found!");
    }
}

fn part_one(file_name: &str) {
    let map = Map::parse(file_name);
    let heat_loss = map.find_min_heat_loss();
    println!("Part 1: {}", heat_loss);
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("sample.txt");
    part_two("sample.txt");

    println!("Done!");
}
