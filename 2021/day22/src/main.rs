use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Formatter;
use std::fmt::Display;
use std::cmp::max;
use std::collections::BTreeSet;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(PartialEq, Hash, Clone, Debug, Eq, Copy)]
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
        let from_start = range_str.find('=').unwrap() + 1;
        let from_end = range_str.find('.').unwrap();
        let to_start = range_str.rfind('.').unwrap() + 1;

        let from = range_str[from_start..from_end].parse::<i32>().unwrap();
        let to = range_str[to_start..].parse::<i32>().unwrap();
        Range::new(from, to)
    }
    fn overlaps(&self, other: &Range, or_touches: bool) -> bool {
        match or_touches {
            true => self.from <= other.to && self.to >= other.from,
            false => self.from < other.to && self.to > other.from
        }
    }
    fn length(&self) -> u64 {
        max(0, (self.to - self.from) + 1) as u64
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

#[derive(PartialEq, Hash, Debug, Eq, Clone, Copy)]
enum Plane {
    XY { x: Range, y: Range, z: i32 },
    XZ { x: Range, y: i32, z: Range },
    YZ { x: i32, y: Range, z: Range }
}

impl Plane {
    fn new_xy(x: Range, y: Range, z: i32) -> Plane {
        Plane::XY { x, y, z }
    }
    fn new_xz(x: Range, y: i32, z: Range) -> Plane {
        Plane::XZ { x, y, z }
    }
    fn new_yz(x: i32, y: Range, z: Range) -> Plane {
        Plane::YZ { x, y, z }
    }
    fn get_area(&self) -> u64 {
        match self {
            Plane::XY { x, y, .. } => x.length() * y.length(),
            Plane::XZ { x, z, .. } => x.length() * z.length(),
            Plane::YZ { y, z, .. } => y.length() * z.length()
        }
    }
}

#[derive(PartialEq, Hash, Debug)]
struct Cuboid {
    dimensions: Vec<Range>
}

impl Display for Cuboid {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.dimensions[0], self.dimensions[1], self.dimensions[2])
    }
}
impl Display for Range {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.from, self.to)
    }
}

impl Cuboid {
    fn parse(line: &str) -> Cuboid {
        let parts = line.split(",").collect::<Vec<_>>();
        let x = Range::parse(parts[0]);
        let y = Range::parse(parts[1]);
        let z = Range::parse(parts[2]);
        Cuboid::from_dims(x, y, z)
    }
    fn from_dims(x: Range, y: Range, z: Range) -> Cuboid {
        Cuboid { 
            dimensions: vec![x, y, z]
        }
    }
    fn contains(&self, other: &Cuboid) -> bool {
        self.dimensions.iter()
            .zip(other.dimensions.iter())
            .find(|(a,b)| !a.contains(b))
            .is_none()
    }
    fn overlaps(&self, other: &Cuboid, or_touches: bool) -> bool {
        self.dimensions.iter()
            .zip(other.dimensions.iter())
            .find(|(a, b)| !a.overlaps(b, or_touches))
            .is_none()
    }
    fn inner_volume(&self) -> u64 {
        self.dimensions.iter()
            .map(|range| max(range.length() - 1, 0))
            .product::<u64>()
    }
    fn get_planes(&self) -> Vec<Plane> {
        let x = &self.dimensions[0];
        let y = &self.dimensions[1];
        let z = &self.dimensions[2];

        let mut planes = Vec::with_capacity(6);
        planes.push(Plane::new_xy(*x, *y, z.from));
        planes.push(Plane::new_xy(*x, *y, z.to));
        planes.push(Plane::new_xz(*x, y.from, *z));
        planes.push(Plane::new_xz(*x, y.to, *z));
        planes.push(Plane::new_yz(x.from, *y, *z));
        planes.push(Plane::new_yz(x.to, *y, *z));
        planes
    }
    fn add_points_to(&self, points: &mut BTreeSet<(i32, i32, i32)>) {
        let x_range = &self.dimensions[0];
        let y_range = &self.dimensions[1];
        let z_range = &self.dimensions[2];

        for x in x_range.from..x_range.to + 1 {
            for y in y_range.from..y_range.to + 1 {
                for z in z_range.from..z_range.to + 1 {
                    points.insert((x, y, z));
                }
            }
        }
    }
    fn remove_points_from(&self, points: &mut BTreeSet<(i32, i32, i32)>) {
        let x_range = &self.dimensions[0];
        let y_range = &self.dimensions[1];
        let z_range = &self.dimensions[2];

        for x in x_range.from..x_range.to + 1 {
            for y in y_range.from..y_range.to + 1 {
                for z in z_range.from..z_range.to + 1 {
                    points.remove(&(x, y, z));
                }
            }
        }
    }
    fn adjust_touching_plane(&mut self, plane: &Plane) {
        let (range, compare) = match plane {
            Plane::XY { z, .. } => (&mut self.dimensions[2], z),
            Plane::XZ { y, .. } => (&mut self.dimensions[1], y),
            Plane::YZ { x, .. } => (&mut self.dimensions[0], x)
        };

        match range.from == *compare {
            true => range.from += 1,
            false => range.to -= 1
        }
    }
    fn adjust_touching_planes(&mut self, all: &mut HashSet<Plane>) {
        self.get_planes()
            .drain(0..)
            .filter(|plane| !all.insert(plane.clone()))
            .for_each(|plane| self.adjust_touching_plane(&plane));
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
        
        let mut planes = HashSet::new();
        
        let mut my_cuboids = Vec::new();
        let mut other_cuboids = Vec::new();
        for x in splits[0].iter() {
            for y in splits[1].iter() {
                for z in splits[2].iter() {
                    let mut cuboid = Cuboid::from_dims(x.clone(), y.clone(), z.clone());
                    if self.contains(&cuboid) {
                        cuboid.adjust_touching_planes(&mut planes);
                        my_cuboids.push(cuboid);
                    } else if other.contains(&cuboid) {
                        cuboid.adjust_touching_planes(&mut planes);
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
        
        let mut exclude_planes = HashSet::new();
        let mut new_cuboids = Vec::new();
        for x in splits[0].iter() {
            for y in splits[1].iter() {
                for z in splits[2].iter() {
                    let cuboid = Cuboid::from_dims(x.clone(), y.clone(), z.clone());
                    if self.contains(&cuboid) {
                        if other.contains(&cuboid) {
                            other.get_planes()
                                .drain(0..)
                                .for_each(|plane| { exclude_planes.insert(plane); });
                        } else {
                            new_cuboids.push(cuboid)
                        }
                    }
                }
            }
        }

        new_cuboids.iter_mut().for_each(|cube| 
            cube.adjust_touching_planes(&mut exclude_planes));

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

fn bound_range(range: &Range) -> Option<Range> {
    let mut from = range.from;
    let mut to = range.to;

    if from < -50 {
        from = -50;
    }
    if to > 50 {
        to = 50;
    }

    match from <= to {
        true => Some(Range::new(from, to)),
        false => None
    }
}

fn bound_instruction(instruction: (bool, Cuboid)) -> Option<(bool, Cuboid)> {
    let (on, cube) = instruction;

    let x = bound_range(&cube.dimensions[0]);
    let y = bound_range(&cube.dimensions[1]);
    let z = bound_range(&cube.dimensions[2]);
    
    match (x, y, z) {
        (Some(x), Some(y), Some(z)) => Some((on, Cuboid::from_dims(x, y, z))),
        _ => None,
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
    fn split_completely(left_cubes: &mut Vec<Cuboid>, right_cubes: &mut Vec<Cuboid>) -> bool {
        let mut split = false;

        let mut new_left = None;
        let mut left_index = 0;

        for left_cube in left_cubes.iter_mut() {
            let mut new_right_cubes = Vec::new();
            while let Some(right_cube) = right_cubes.pop() {
                if left_cube.overlaps(&right_cube) {
                    let (split_left, split_right) = left_cube.split(&right_cube);
                    new_left = Some(split_left);
                    split_right.into_iter().for_each(|cube| new_right_cubes.push(cube));
                    split = true;
                    break;
                } else {
                    new_right_cubes.push(right_cube);
                }
            }

            // add remaining right cubes that we didn't use
            while let Some(right_cube) = right_cubes.pop() {
                new_right_cubes.push(right_cube);
            }

            *right_cubes = new_right_cubes;
            
            if split {
                break;
            }
            left_index += 1;
        }

        if let Some(mut split_left) = new_left {
            if split_left.len() == 0 {
                left_cubes.remove(left_index);
            } else {
                left_cubes[left_index] = split_left.pop().unwrap();
                split_left.drain(0..)
                    .for_each(|cube| left_cubes.push(cube));
            }
            true
        } else {
            false
        }
    }
    fn set_cubes(&mut self, on: bool, new_cuboid: Cuboid) {

        if on {
            let mut new_cubes = vec![new_cuboid];
            while Reactor::split_completely(&mut self.cubes, &mut new_cubes) { 
                // keep splitting the new cubes until we have not split anymore
            }
            // add all the new cubes
            new_cubes.into_iter().for_each(|cube| {
                self.cubes.push(cube)
            });
        } else {
            let mut new_cubes = Vec::<Cuboid>::new();
            while let Some(cube) = self.cubes.pop() {
                if cube.overlaps(&new_cuboid) {
                    let these_new_cubes = cube.remove(&new_cuboid);
                    these_new_cubes.into_iter()
                        .for_each(|cube| new_cubes.push(cube));
                } else {
                    new_cubes.push(cube);
                }
            }
            self.cubes = new_cubes;
        }

    }

    fn calc_lit_count(&self) -> u64 {
        let mut planes = HashSet::new();
        let mut total_volume = 0;
        for cube in self.cubes.iter() {
            let this_volume = cube.inner_volume();
            cube.get_planes().into_iter()
                .for_each(|plane| { planes.insert(plane); });
            total_volume += this_volume;
        }
        
        let plane_sum = planes.iter()
            .map(|plane| plane.get_area())
            .sum::<u64>();
        
        total_volume + plane_sum
    }
}

fn part_one(file_name: &str) {
    let mut reactor = Reactor::new();
    let mut all_points = BTreeSet::new();

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| parse_line(line))
        .flat_map(|instruction| bound_instruction(instruction))
        .take(4)
        .for_each(|(on, cuboid)| {
            match on {
                true => cuboid.add_points_to(&mut all_points),
                false => cuboid.remove_points_from(&mut all_points)
            };
            println!("Setting {} to {}", cuboid, on);
            reactor.set_cubes(on, cuboid);
            println!("Lit: brute: {} real: {}", all_points.len(), reactor.calc_lit_count());
        });
    
    let lit = reactor.calc_lit_count();

    println!("Part 1: {}", lit);
}

fn part_two(file_name: &str) {
    let mut reactor = Reactor::new();

    let mut index = 1;
    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| parse_line(line))
        .for_each(|(on, cuboid)| {
            println!("Setting #{} {} to {}", index, cuboid, on);
            reactor.set_cubes(on, cuboid);
            index += 1;
        });
    
    let lit = reactor.calc_lit_count();

    println!("Part 2: {}", lit);
}

fn main() {
    part_one("input.txt");
    // part_two("input.txt");

    println!("Done!");
}
