use std::collections::{BTreeSet, HashSet, HashMap};
use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Map {
    blocks: Vec<Vec<u32>>,
    width: i32,
    height: i32
}

struct VisitLog {
    visited: HashMap<(i32, i32), usize>
}

impl VisitLog {
    fn new() -> VisitLog {
        let mut visited = HashMap::new();
        
        visited.insert((0, 0), 0);

        VisitLog {
            visited
        }
    }
    fn contains(&self, point: &(i32, i32), straight_count: &usize) -> bool {
        self.visited.get(&point)
            .map(|existing_count| existing_count <= straight_count)
            .unwrap_or(false)
    }
    fn visit(&mut self, point: (i32, i32), straight_count: usize) {
        if let Some(count) = self.visited.get_mut(&point) {
            *count = (*count).min(straight_count);
        } else {
            self.visited.insert(point, straight_count);
        }
    }
}

#[derive(Ord, Eq, PartialEq, PartialOrd)]
struct Step {
    heat_loss: u32,
    point: (i32, i32),
    prev: (i32, i32),
    straight_count: usize,
    points: BTreeSet<(i32, i32)>
}

impl Step {
    fn start(map: &Map, candidates: &mut BTreeSet<Step>) {
        candidates.insert(
            Step {
                heat_loss: map.blocks[0][1],
                point: (1, 0),
                prev: (0, 0),
                straight_count: 1,
                points: BTreeSet::from_iter(vec![(0, 0), (1, 0)].into_iter())
            }
        );
        // candidates.insert(
        //     Step {
        //         heat_loss: map.blocks[1][0],
        //         point: (0, 1),
        //         prev: (0, 0),
        //         straight_count: 1,
        //         path: vec![(0, 0), (0, 1)]
        //     }
        // );
    }
    fn is_straight(&self, (x, y): &(i32, i32)) -> bool {
        // comparing this point to the current
        // step's previous point, we should be
        // able to determine whether or not this
        // is a straight move.
        let (prev_x, prev_y) = self.prev;
        (prev_x - x) == 0 || (prev_y - y) == 0
    }
    fn visit(&self, map: &Map, visited: &mut VisitLog, candidates: &mut BTreeSet<Step>) {
        let (x, y) = self.point;

        let neighbors = vec![
            (x + 1, y),
            (x - 1, y),
            (x, y - 1),
            (x, y + 1)
        ];

        visited.visit(self.point, self.straight_count);
        
        // let mut index = 0;
        // let points = self.path.iter()
        //     .map(|point| {
        //         let current = index;
        //         index += 1;
        //         (point, current)
        //     })
        //     .collect::<HashMap<_,_>>();
        // for y in 0..map.height {
        //     for x in 0..map.width {
        //         if let Some(index) = points.get(&(x, y)) {
        //             print!("{}", index % 10);
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!("");
        // }
        // print!("Visiting: {:?} with heat: {} and straights: {} - new_candidates:", self.path, self.heat_loss, self.straight_count);

        neighbors.into_iter()
            .filter(|(x, y)| *x >= 0 && *x < map.width && *y >= 0 && *y < map.height)
            .map(|(x, y)| {
                let straight_count = match self.is_straight(&(x, y)) {
                    true => self.straight_count + 1,
                    false => 1
                };
                (x, y, straight_count)
            })
            .filter(|(_, _, straights)| self.straight_count < 3 || *straights == 1)
            .filter(|(x, y, straights)| !visited.contains(&(*x, *y), &straights))
            .for_each(|(x, y, straight_count)| {
                let added_heat = map.blocks[y as usize][x as usize];
                let mut points = self.points.clone();
                points.insert((x, y));
                // print!(" ({}, {})", x, y);
                let step = Step {
                    heat_loss: self.heat_loss + added_heat,
                    point: (x, y),
                    prev: self.point,
                    straight_count,
                    points
                };
                candidates.insert(step);
            });
        // println!("");
    }
}

impl Map {
    fn parse(file_name: &str) -> Map {

        let blocks = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
            
        let height = blocks.len() as i32;
        let width = blocks[0].len() as i32;
        
        Map { blocks, width, height }
    }

    fn find_min_heat_loss(&self) -> u32 {
        let mut candidates = BTreeSet::<Step>::new();
        Step::start(&self, &mut candidates);

        let end_y = self.blocks.len() - 1;
        let end_x = self.blocks[0].len() - 1;
        let end = (end_x as i32, end_y as i32);
        let end = (8, 0);

        let mut visited = VisitLog::new();

        while let Some(step) = candidates.pop_first() {
            if step.point == end {
                // println!("Path: {:?}", step.path);
                return step.heat_loss;
            }
            step.visit(&self, &mut visited, &mut candidates);
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
    part_one("input.txt");
    part_two("sample.txt");

    println!("Done!");
}
