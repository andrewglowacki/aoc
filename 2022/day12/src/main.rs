use std::collections::{BTreeSet, HashSet, HashMap};
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
    elevations: Vec<Vec<u32>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize
}

struct Step {
    pos: (usize, usize),
    prev: (usize, usize),
    next: BTreeSet<(usize, usize)>
}

impl Step {
    fn new(pos: (usize, usize), prev: (usize, usize), next: BTreeSet<(usize, usize)>) -> Step {
        Step {
            pos,
            prev,
            next
        }
    }
}

impl Map {
    fn from_lines(file_name: &str) -> Map {
        let mut start = (0, 0);
        let mut end = (0, 0);

        let mut y = 0;

        let elevations = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| {
                let mut x = 0;
                let new_line = line.chars().map(|c| {
                    let result = match c {
                        'S' => {
                            start = (x, y);
                            0
                        },
                        'E' => {
                            end = (x, y);
                            ('z' as u32 - 'a' as u32) + 1
                        },
                        _ => (c as u32 - 'a' as u32) + 0
                    };
                    x += 1;
                    result
                }).collect::<Vec<_>>();
                y += 1;
                new_line
            })
            .collect::<Vec<_>>();

        let height =  elevations.len();
        let width = elevations[0].len();

        Map {
            elevations,
            start,
            end,
            width,
            height
        }
    }

    fn get_elevation(&self, point: &(usize, usize)) -> u32 {
        let (x, y) = point;
        self.elevations[*y][*x]
    }

    fn add_if_clear(&self, prev_elevation: u32, point: (usize, usize), add_to: &mut BTreeSet<(usize, usize)>, visited: &HashSet<(usize, usize)>) {
        if !visited.contains(&point) && self.get_elevation(&point) <= prev_elevation + 1 {
            add_to.insert(point);
        }
    }

    fn get_next_points(&self, from: &(usize, usize), visited: &HashSet<(usize, usize)>) -> BTreeSet<(usize, usize)> {
        let (x, y) = *from;
        let from_elevation = self.get_elevation(from);
        let mut points = BTreeSet::new();
        if x > 0 {
            self.add_if_clear(from_elevation, (x - 1, y), &mut points, visited);
        }
        if x < self.width - 1 {
            self.add_if_clear(from_elevation, (x + 1, y), &mut points, visited);
        }
        if y > 0 {
            self.add_if_clear(from_elevation, (x, y - 1), &mut points, visited);
        }
        if y < self.height - 1 {
            self.add_if_clear(from_elevation, (x, y + 1), &mut points, visited);
        }
        points
    }

    fn find_steps_to_end(&self) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut steps = HashMap::<(usize, usize), Step>::new();

        visited.insert(self.start);

        let initial_next = self.get_next_points(&self.start, &mut visited);
        initial_next.iter().for_each(|point| {
            visited.insert(*point);
        });

        let start = Step::new(self.start, self.start, initial_next);
        steps.insert(self.start, start);

        let mut to_check = vec![self.start];
        let mut result: Option<Step> = None;

        while to_check.len() > 0 && result.is_none() {
            let mut next_check = Vec::new();
            for pos in to_check.iter() {
                let step = steps.get(pos).unwrap();
                for next in step.next.iter() {
                    let next_points = self.get_next_points(next, &visited);
                    next_points.iter().for_each(|point| {
                        visited.insert(*point);
                    });
                    if next_points.len() > 0 {
                        let next_step = Step::new(*next, *pos, next_points);
                        if next_step.next.contains(&self.end) {
                            result = Some(next_step);
                            break;
                        }
                        next_check.push(next_step);
                    }
                }
                if result.is_some() {
                    break;
                }
            }

            to_check.clear();
            next_check.into_iter().for_each(|step| {
                to_check.push(step.pos);
                steps.insert(step.pos, step);
            });
        }

        let mut path = Vec::new();
        if let Some(prev_step) = result {
            let mut prev_step = &prev_step;
            path.push(self.end);

            while prev_step.pos != prev_step.prev {
                path.push(prev_step.pos);
                prev_step = steps.get(&prev_step.prev).unwrap();
            }

            path.reverse();
            Some(path.len())
        } else {
            None
        }
    }
}

fn part_one(file_name: &str) {
    let map = Map::from_lines(file_name);

    let steps = map.find_steps_to_end().unwrap();
    
    println!("Part 1: {}", steps);
}

fn part_two(file_name: &str) {
    let mut map = Map::from_lines(file_name);

    let mut starts = Vec::new();

    for y in 0..map.height {
        for x in 0..map.width {
            if map.get_elevation(&(x, y)) == 0 {
                starts.push((x, y));
            }
        }
    }

    let mut min = map.height * map.width;
    for start in starts {
        map.start = start;
        if let Some(steps) = map.find_steps_to_end() {
            if steps < min {
                min = steps;
            }
        }
    }
    
    println!("Part 2: {}", min);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
