use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn parse(string: &str) -> Point {
        let mut pieces = string.split(",");
        let x = pieces.next().unwrap().parse::<i32>().unwrap();
        let y = pieces.next().unwrap().parse::<i32>().unwrap();
        Point { x, y }
    }

    fn add_point(x: i32, y: i32, add_to: &mut BTreeMap<i32, BTreeSet<i32>>) {
        if let Some(y_set) = add_to.get_mut(&x) {
            y_set.insert(y);
        } else {
            let mut y_set = BTreeSet::new();
            y_set.insert(y);
            add_to.insert(x, y_set);
        }
    }

    fn get_points(&self, dest: &Point, add_to: &mut BTreeMap<i32, BTreeSet<i32>>) {
        let x_len = (self.x - dest.x).abs();
        let y_len = (self.y - dest.y).abs();

        let x_start = self.x.min(dest.x);
        let y_start = self.y.min(dest.y);

        match x_len == 0 {
            true => (y_start..y_start + y_len + 1)
                .into_iter()
                .map(|y| (self.x, y))
                .for_each(|(x, y)| Point::add_point(x, y, add_to)),
            false => (x_start..x_start + x_len + 1)
                .into_iter()
                .map(|x| (x, self.y))
                .for_each(|(x, y)| Point::add_point(x, y, add_to))
        };
    }
}

struct Rocks {
    x_map: BTreeMap<i32, BTreeSet<i32>>,
    y_floor: i32
}

impl Rocks {
    fn from_file(file_name: &str) -> Rocks {
        let mut x_map = BTreeMap::new();
        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .for_each(|line| {
                line.split(" -> ")
                    .map(Point::parse)
                    .reduce(|prev, current| {
                        prev.get_points(&current, &mut x_map);
                        current
                    });
            });
        
        let y_floor = *x_map.values()
            .flat_map(|y_set| y_set)
            .max()
            .unwrap() + 2;

        Rocks { 
            x_map,
            y_floor
        }
    }

    fn is_open(&self, x: i32, y: i32) -> bool {
        if let Some(y_set) = self.x_map.get(&x) {
            if let Some(next_y) = y_set.range(y..).next() {
                *next_y != y
            } else {
                true
            }
        } else {
            true
        }
    }

    fn add_sand(&mut self) -> bool {
        let mut x = 500;
        let mut y = 0;
        
        loop {
            if let Some(y_set) = self.x_map.get(&x) {
                if let Some(next_y) = y_set.range(y..).next() {
                    y = *next_y;
                } else {
                    return false;
                };

                if self.is_open(x - 1, y) {
                    x -= 1;
                } else if self.is_open(x + 1, y) {
                    x += 1;
                } else {
                    y -= 1;
                    break;
                }
            } else {
                return false;
            }
        }

        self.x_map.get_mut(&x)
            .unwrap()
            .insert(y);
        
        true
    }

    fn add_sand_with_floor(&mut self) -> bool {
        let mut x = 500;
        let mut y = 0;
        
        loop {
            if let Some(y_set) = self.x_map.get(&x) {
                if let Some(next_y) = y_set.range(y..).next() {
                    y = *next_y;
                } else {
                    y = self.y_floor - 1;
                    break;
                };

                if self.is_open(x - 1, y) {
                    x -= 1;
                } else if self.is_open(x + 1, y) {
                    x += 1;
                } else {
                    y -= 1;
                    break;
                }
            } else {
                y = self.y_floor - 1;
                break;
            }
        }

        if x == 500 && y == 0 {
            return false;
        }

        if let Some(y_set) = self.x_map.get_mut(&x) {
            y_set.insert(y);
        } else {
            let mut y_set = BTreeSet::new();
            y_set.insert(y);
            self.x_map.insert(x, y_set);
        }
        true
    }

    fn fill_with_sand(&mut self) -> u32 {
        let mut sand = 0;
        while self.add_sand() {
            sand += 1;
        }
        sand
    }

    fn fill_with_sand_with_floor(&mut self) -> u32 {
        let mut sand = 0;
        while self.add_sand_with_floor() {
            sand += 1;
        }
        sand + 1
    }
}

fn part_one(file_name: &str) {
    let mut rocks = Rocks::from_file(file_name);

    let added = rocks.fill_with_sand();
    
    println!("Part 1: {}", added);
}

fn part_two(file_name: &str) {
    let mut rocks = Rocks::from_file(file_name);

    let added = rocks.fill_with_sand_with_floor();
    
    println!("Part 2: {}", added);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
