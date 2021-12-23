use std::mem::swap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(PartialEq, Hash, Clone)]
struct Range {
    from: i32,
    to: i32
}

impl Range {
    fn new(from: i32, to: i32) -> Range {
        Range {
            from,
            to
        }
    }
    fn parse(range_str: &str) -> Range {
        let from_end = range_str.find('.').unwrap();
        let to_start = range_str.rfind('.').unwrap() + 1;
        let from = range_str[0..from_end].parse::<i32>().unwrap();
        let to = range_str[to_start..].parse::<i32>().unwrap();
        Range::new(from, to)
    }
    fn from(range: std::ops::Range<i32>) -> Range {
        Range::new(range.start, range.end)
    }
    fn remove_touch_points(&mut self, other: &Range) {
        if self.from == other.to {
            self.from -= 1;
        } else if self.to == other.from {
            self.to += 1;
        }
    }
    fn overlaps(&self, other: &Range, or_touches: bool) -> bool {
        match or_touches {
            true => self.from <= other.to && self.to >= other.from,
            false => self.from < other.to && self.to > other.from
        }
    }
    fn contains(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }
    fn split(&self, other: &Range) -> Vec<Range> {
        let mut points = BTreeSet::new();
        points.insert(self.from);
        points.insert(self.to);
        points.insert(other.from);
        points.insert(other.to);
        
        /*
         * Turn:
         * |-----|
         *    |------|
         * Into:
         * |--|  |---|
         *    |--|
         */

        let mut ranges = Vec::with_capacity(points.len() - 1);
        let mut points = points.into_iter();
        let mut prev = points.next().unwrap();
        while let Some(next) = points.next() {
            ranges.push(Range::new(prev, next));
            prev = next;
        }
        ranges
    }
}

#[derive(PartialEq, Hash)]
struct Cuboid {
    dimensions: Vec<Range>
}

impl Cuboid {
    fn parse(line: &str) -> Cuboid {
        let parts = line.split(",").collect::<Vec<_>>();
        Cuboid {
            dimensions: vec![
                Range::parse(parts[0]),
                Range::parse(parts[1]),
                Range::parse(parts[2])
            ]
        }
    }
    fn from_dims(x: Range, y: Range, z: Range) -> Cuboid {
        Cuboid { dimensions: vec![x, y, z] }
    }
    fn contains(&self, other: &Cuboid) -> bool {
        self.dimensions.iter()
            .zip(other.dimensions.iter())
            .find(|(a,b)| !a.contains(b))
            .is_none()
    }
    fn remove_touch_points(&mut self, other: &Cuboid) {
        self.dimensions.iter_mut()
            .zip(other.dimensions.iter())
            .for_each(|(a, b)| a.remove_touch_points(b));
    }
    fn overlaps(&self, other: &Cuboid, or_touches: bool) -> bool {
        self.dimensions.iter()
            .zip(other.dimensions.iter())
            .find(|(a, b)| !a.overlaps(b, or_touches))
            .is_none()
    }
    fn split(&self, other: &Cuboid) -> (Vec<Cuboid>, Vec<Cuboid>) {

        /*
         * Split overlapping cubes resulting in:
         * _________    __________
         * | _____ |    |_|____|_|
         * | |   | | => | |    | |
         * | |___| |    |_|____|_|
         * |_______|    |_|____|_|
         * 
         * or:
         * ________         ____________
         * |   ____|___     |___|___|___|
         * |   |       | => |   |   |   |
         * |___|       |    |___|___|___|
         *     |_______|    |___|___|___|
         */

        let splits = self.dimensions.iter()
            .zip(other.dimensions.iter())
            .map(|(a, b)| a.split(&b))
            .collect::<Vec<_>>();
        
        let mut my_cuboids = Vec::new();
        let mut other_cuboids = Vec::new();
        for x in splits[0].iter() {
            for y in splits[1].iter() {
                for z in splits[2].iter() {
                    let cuboid = Cuboid::from_dims(x.clone(), y.clone(), z.clone());
                    if self.contains(&cuboid) {
                        my_cuboids.push(cuboid)
                    } else if other.contains(&cuboid) {
                        other_cuboids.push(cuboid);
                    }
                }
            }
        }

        (my_cuboids, other_cuboids)
    }
    fn remove(&self, other: &Cuboid) -> Vec<Cuboid> {
        let splits = self.dimensions.iter()
            .zip(other.dimensions.iter())
            .map(|(a, b)| a.split(&b))
            .collect::<Vec<_>>();
        
        let mut new_cuboids = Vec::new();
        for x in splits[0].iter() {
            for y in splits[1].iter() {
                for z in splits[2].iter() {
                    let mut cuboid = Cuboid::from_dims(x.clone(), y.clone(), z.clone());
                    if self.contains(&cuboid) && !other.contains(&cuboid) {
                        cuboid.remove_touch_points(other);
                        new_cuboids.push(cuboid)
                    }
                }
            }
        }

        new_cuboids
    }
}

fn parse_line(line: String) -> (bool, Cuboid) {
    let sep = line.find(' ').unwrap();
    let cuboid = Cuboid::parse(&line[sep + 1..]);
    match sep {
        // on
        2 => (true, cuboid),
        // off
        3 => (false, cuboid),
        // ????
        _ => panic!("Invalid on/off flag in: {}", line)
    }
}

struct Reactor {
    cubes: Vec<Cuboid>
}

impl Reactor {
    fn new() -> Reactor {
        Reactor {
            cubes: Vec::new()
        }
    }
    fn set_cubes(&mut self, on: bool, new_cuboid: Cuboid) {
        
        let mut new_cubes = Vec::<Cuboid>::new();
        if on {
            let mut new_cuboids = vec![new_cuboid];
            while let Some(cube) = self.cubes.pop() {
                let mut left_cubes = vec![cube];
                let mut any_overlap = false;
                while left_cubes.len() > 0 && new_cuboids.len() > 0 {
                    let left_cube = left_cubes.pop().unwrap();
                    let new_cube = new_cuboids.pop().unwrap();
                    if left_cube.overlaps(&new_cube, false) {
                        let (existing, added) = left_cube.split(&new_cube);
                        left_cubes = existing;
                        new_cuboids = added;
                    }
                }
            }
        } else {
            while let Some(cube) = self.cubes.pop() {
                if cube.overlaps(&new_cuboid, true) {
                    cube.remove(&new_cuboid).into_iter()
                        .for_each(|cube| new_cubes.push(cube));
                } else {
                    new_cubes.push(cube);
                }
            }
        }

        self.cubes = new_cubes;
    }
}

fn part_one(file_name: &str) {
    let mut reactor = Reactor::new();

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| parse_line(line))
        .for_each(|(on, cuboid)| reactor.set_cubes(on, cuboid));
    
    println!("Part 1: {}", "incomplete");
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
