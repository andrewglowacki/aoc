use std::collections::{HashMap, LinkedList, HashSet};
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

#[derive(PartialEq, Debug, Clone)]
enum Pipe {
    EastWest,
    NorthSouth,
    NorthToEast,
    NorthToWest,
    SouthToEast,
    SouthToWest,
    Start,
    Empty
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    source: (i32, i32)
}

impl Point {
    fn new((x, y): (i32, i32), source: (i32, i32)) -> Point {
        Point { x, y, source }
    }
    fn start((x, y): (i32, i32)) -> Point {
        Point::new((x, y), (x, y))
    }
}

const VALID_RIGHT: [Pipe; 3] = [Pipe::NorthToWest, Pipe::SouthToWest, Pipe::EastWest];
const VALID_LEFT: [Pipe; 3] = [Pipe::NorthToEast, Pipe::SouthToEast, Pipe::EastWest];
const VALID_UP: [Pipe; 3] = [Pipe::SouthToWest, Pipe::SouthToEast, Pipe::NorthSouth];
const VALID_DOWN: [Pipe; 3] = [Pipe::NorthToWest, Pipe::NorthToEast, Pipe::NorthSouth];

struct Map {
    pipes: HashMap<(i32, i32), Pipe>
}

impl Map {
    fn parse(file_name: &str) -> Map {
        let mut y = 0;
        let pipes = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .flat_map(|line| {
                let mut x = 0;
                let result = line.chars().flat_map(move |c| {
                    let pipe = match c {
                        '-' => Pipe::EastWest,
                        '|' => Pipe::NorthSouth,
                        'L' => Pipe::NorthToEast,
                        'J' => Pipe::NorthToWest,
                        '7' => Pipe::SouthToWest,
                        'F' => Pipe::SouthToEast,
                        'S' => Pipe::Start,
                        '.' => Pipe::Empty,
                        x => panic!("Unexpected character: {}", x)
                    };
                    let result = match pipe {
                        Pipe::Empty => None,
                        pipe => Some(((x, y), pipe))
                    };
                    x += 1;
                    result
                }).collect::<Vec<_>>();
                y += 1;
                result
            })
            .collect::<HashMap<_,_>>();
        Map { pipes }
    }

    fn get_adjacent(&self, from: &Point, adjacent: &mut Vec<Point>) {
        let x = from.x;
        let y = from.y;
        let current = self.pipes.get(&(x, y)).unwrap();
        // println!("Getting adjacent for {:?} - pipe is: {:?}", from, current);
        let points = match current {
            Pipe::EastWest => vec![
                ((x - 1, y), VALID_LEFT),
                ((x + 1, y), VALID_RIGHT)
            ],
            Pipe::NorthSouth => vec![
                ((x, y - 1), VALID_UP),
                ((x, y + 1), VALID_DOWN)
            ],
            Pipe::NorthToEast => vec![
                ((x, y - 1), VALID_UP),
                ((x + 1, y), VALID_RIGHT)
            ],
            Pipe::NorthToWest => vec![
                ((x, y - 1), VALID_UP),
                ((x - 1, y), VALID_LEFT)
            ],
            Pipe::SouthToEast => vec![
                ((x, y + 1), VALID_DOWN),
                ((x + 1, y), VALID_RIGHT)
            ],
            Pipe::SouthToWest => vec![
                ((x, y + 1), VALID_DOWN),
                ((x - 1, y), VALID_LEFT)
            ],
            Pipe::Start => vec![
                ((x - 1, y), VALID_LEFT),
                ((x + 1, y), VALID_RIGHT),
                ((x, y - 1), VALID_UP),
                ((x, y + 1), VALID_DOWN)
            ],
            unexpected => panic!("Unexpected pipe: {:?}", unexpected)
        };

        points.into_iter()
            .map(|(point, valid_next)| (Point::new(point, from.source), valid_next))
            .flat_map(|(point, valid_next)| self.pipes.get(&(point.x, point.y)).map(|pipe| (point, pipe, valid_next)))
            .filter(|(_, pipe, valid_next)| valid_next.contains(*pipe))
            .for_each(|(point, _, _)| {
                adjacent.push(point)
            });
    }

    fn build_backward_path(&self, start: (i32, i32), from: (i32, i32), visited: &HashMap<(i32, i32), Point>) -> LinkedList<(i32, i32)> {
        let mut next = from;
        let mut path = LinkedList::new();
        while next != start {
            path.push_front(next);
            let prev_point = visited.get(&next).unwrap();
            next = (prev_point.x, prev_point.y);
        }
        path.push_front(start);
        path
    }

    fn build_forward_path(&self, start: (i32, i32), from: (i32, i32), visited: &HashMap<(i32, i32), Point>, path: &mut LinkedList<(i32, i32)>) {
        let mut next = from;
        while next != start {
            path.push_back(next);
            let prev_point = visited.get(&next).unwrap();
            next = (prev_point.x, prev_point.y);
        }
    }

    fn find_loop(&self) -> Vec<(i32, i32)> {
        let start = *self.pipes.iter()
            .find_map(|(coord, pipe)| {
                match pipe {
                    Pipe::Start => Some(coord),
                    _ => None
                }
            })
            .unwrap();

        let mut visited = HashMap::<(i32, i32), Point>::new();

        let mut current = Vec::new();
        let start_coords = start;
        let start = Point::start(start);
        self.get_adjacent(&start, &mut current);
        visited.clear();
        let mut found = HashMap::<(i32, i32), usize>::new();

        current.iter_mut().for_each(|point| {
            point.source = (point.x, point.y);
            visited.insert(point.source, start.clone());
            found.insert((point.x, point.y), 1);
        });
        found.insert(start_coords, 0);

        let mut next_current = Vec::<Point>::new();
        let mut adjacent = Vec::<Point>::new();

        let mut steps = 2;
        while !current.is_empty() {
            while let Some(next) = current.pop() {
                self.get_adjacent(&next, &mut adjacent);
                while let Some(point) = adjacent.pop() {
                    if let Some(prev) = visited.get(&(point.x, point.y)) {
                        if prev.source != point.source && prev.source != start_coords {
                            let orig_coords = (next.x, next.y);
                            let mut path = self.build_backward_path(start_coords, orig_coords, &visited);
                            self.build_forward_path(start_coords, (point.x, point.y), &visited, &mut path);
                            return path.into_iter().collect::<Vec<_>>();
                        }
                    } else {
                        visited.insert((point.x, point.y), next.clone());
                        found.insert((point.x, point.y), steps);
                        next_current.push(point);
                    }
                }
            }
            swap(&mut current, &mut next_current);
            steps += 1;
        }

        panic!("End not found!");
    }

    fn count_enclosed_area(&self, loop_path: Vec<(i32, i32)>) -> usize {
        let loop_path = loop_path.into_iter().collect::<HashSet<_>>();

        let pipes = self.pipes.iter()
            .filter(|(coord, _)| loop_path.contains(coord))
            .map(|(coord, pipe)| (*coord, pipe.clone()))
            .collect::<HashMap<_,_>>();

        let width = loop_path.iter()
            .map(|(x, _)| x)
            .max()
            .unwrap();

        let height = loop_path.iter()
            .map(|(_, y)| y)
            .max()
            .unwrap();

        let mut count = 0;
        let mut inside = false;
        let mut start = Pipe::Empty;
        for y in 0..height + 1 {
            for x in 0..width + 1 {
                if let Some(pipe) = pipes.get(&(x, y)) {
                    match (&start, pipe) {
                        (_, Pipe::EastWest) => (),
                        (_, Pipe::NorthSouth) => inside = !inside,
                        (_, Pipe::Start) => inside = !inside,
                        (Pipe::NorthToEast, Pipe::NorthToWest) => inside = !inside,
                        (Pipe::NorthToEast, Pipe::SouthToWest) => (),
                        (Pipe::NorthToEast, _) => (),
                        (Pipe::NorthToWest, _) => inside = !inside,
                        (Pipe::SouthToEast, Pipe::NorthToWest) => (),
                        (Pipe::SouthToEast, Pipe::SouthToWest) => inside = !inside,
                        (Pipe::SouthToEast, _) => (),
                        (Pipe::SouthToWest, _) => inside = !inside,
                        (_, _) => inside = !inside 
                    }
                    if *pipe != Pipe::EastWest {
                        start = pipe.clone();
                    }
                } else if inside {
                    count += 1;
                }
            }
        }

        count
    }
}

fn part_one(file_name: &str) {
    let map = Map::parse(file_name);
    let path = map.find_loop();
    let steps = (path.len() / 2) + (path.len() % 2);
    println!("Part 1: {}", steps);
}

fn part_two(file_name: &str) {
    let map = Map::parse(file_name);
    let path = map.find_loop();
    let count = map.count_enclosed_area(path);
    println!("Part 2: {}", count);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
