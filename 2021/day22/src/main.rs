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

#[derive(PartialEq, Hash, Clone, Debug)]
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
    fn inner_length(&self) -> u64 {
        max(0, (self.to - self.from) - 1) as u64
    }
    fn contains(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }
    fn contains_point(&self, point: i32) -> bool {
        self.from <= point && point <= self.to
    }
    fn constrain(&mut self, other: &Range) -> Range {
        let mut from = self.from;
        let mut to = self.to;

        if from < other.from {
            from = other.from;
        }
        if to > other.to {
            to = other.to;
        }

        Range::new(from, to)
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

#[derive(PartialEq, Hash, Debug)]
enum PlaneType {
    XY,
    XZ,
    YZ
}

#[derive(PartialEq, Hash, Debug)]
struct Plane {
    x: Range,
    y: Range,
    z: Range,
    plane_type: PlaneType
}

impl Plane {
    fn new_xy(x: Range, y: Range, z: i32) -> Plane {
        Plane {
            x,
            y,
            z: Range::new(z, z),
            plane_type: PlaneType::XY
        }
    }
    fn new_xz(x: Range, y: i32, z: Range) -> Plane {
        Plane {
            x,
            y: Range::new(y, y),
            z,
            plane_type: PlaneType::XZ
        }
    }
    fn new_yz(x: i32, y: Range, z: Range) -> Plane {
        Plane {
            x: Range::new(x, x),
            y,
            z,
            plane_type: PlaneType::YZ
        }
    }
    fn subtract(&self, other: &Plane) -> Vec<Plane> {
        let planes = Vec::with_capacity(3);
        match self.plane_type {
            PlaneType::XY => {
                let x_splits = self.x.split(&other.x);
                let y_splits = self.y.split(&other.y);
                for x in x_splits {
                    for y in y_splits {
                        if other.x.contains(&x) && other.y.contains(&y) {
                            continue;
                        }
                        planes.push(Plane::new_xy(x, y, self.z.from));
                    }
                }
            },
            PlaneType::XZ => {
                let x_splits = self.x.split(&other.x);
                let z_splits = self.z.split(&other.z);
                for x in x_splits {
                    for z in z_splits {
                        if other.x.contains(&x) && other.z.contains(&z) {
                            continue;
                        }
                        planes.push(Plane::new_xz(x, self.y.from, z));
                    }
                }
            },
            PlaneType::YZ => {
                let y_splits = self.y.split(&other.y);
                let z_splits = self.z.split(&other.z);
                for y in y_splits {
                    for z in z_splits {
                        if other.z.contains(&z) && other.y.contains(&y) {
                            continue;
                        }
                        planes.push(Plane::new_yz(self.x.from, y, z));
                    }
                }
            }
        }
        planes
    }
    fn overlaps(&self, other: &Plane) -> bool {
        match self.plane_type {
            PlaneType::XY => self.z.from == other.z.from && 
                self.x.overlaps(&other.x, true) &&
                self.y.overlaps(&other.y, true),
            PlaneType::XZ => self.y.from == other.y.from && 
                self.x.overlaps(&other.x, true) &&
                self.z.overlaps(&other.z, true),
            PlaneType::YZ => self.x.from == other.x.from && 
                self.z.overlaps(&other.z, true) &&
                self.y.overlaps(&other.y, true)
        }
    }
    fn is_constrained_to(&self, cube: &Cuboid) -> bool {
        match self.plane_type {
            PlaneType::XY => self.x == cube.dimensions[0]  && self.y == cube.dimensions[1],
            PlaneType::XZ => self.x == cube.dimensions[0]  && self.z == cube.dimensions[2],
            PlaneType::YZ => self.y == cube.dimensions[1]  && self.z == cube.dimensions[2]
        }
    }
    fn constrain(&self, cube: &Cuboid) -> Plane {
        match self.plane_type {
            PlaneType::XY => Plane::new_xy(
                self.x.constrain(&cube.dimensions[0]), 
                self.y.constrain(&cube.dimensions[1]),
                self.z.from),
            PlaneType::XZ => Plane::new_xz(
                self.x.constrain(&cube.dimensions[0]), 
                self.y.from,
                self.z.constrain(&cube.dimensions[2])),
            PlaneType::YZ => Plane::new_yz(
                self.x.from, 
                self.y.constrain(&cube.dimensions[1]),
                self.z.constrain(&cube.dimensions[2]))
        }
    }
}

#[derive(PartialEq, Hash, Debug)]
struct Cuboid {
    dimensions: Vec<Range>,
    xy: BTreeSet<Plane>,
    xz: BTreeSet<Plane>,
    yz: BTreeSet<Plane>
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
            dimensions: vec![x, y, z], 
            xy: vec![Plane::new_xy(x, y, z.from), Plane::new_xy(x, y, z.to)],
            xz: vec![Plane::new_xz(x, y.from, z), Plane::new_xz(x, y.to, z)],
            yz: vec![Plane::new_yz(x.from, y, z), Plane::new_yz(x.to, y, z)]
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
            .map(|range| range.inner_length())
            .product()
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
    fn contains_point(&self, (x, y, z): &(i32, i32, i32)) -> bool {
        self.dimensions[0].contains_point(*x) &&
        self.dimensions[1].contains_point(*y) &&
        self.dimensions[2].contains_point(*z)
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
    fn exclude_planes(planes: &mut BTreeSet<Plane>, others: &BTreeSet<Plane>) {
        let new_planes = BTreeSet::new();
        while let Some(my_plane) = planes.take() {
            let mut overlapped = false;
            others.iter()
                .filter(|other| my_plane.overlaps(other))
                .flat_map(|other| {
                    let subtracted = my_plane.subtract(other);
                    overlapped = true;
                    subtracted
                })
                .for_each(|plane| new_planes.push(plane));
            if !overlapped {
                new_planes.push(my_plane);
            }
        }
        *planes = new_planes;
    }
    fn exclude_all_planes(&mut self, other: &Cuboid) {
        Cuboid::exclude_planes(&mut self.xy, &other.xy);
        Cuboid::exclude_planes(&mut self.xz, &other.xz);
        Cuboid::exclude_planes(&mut self.yz, &other.yz);
    }
    fn add_planes(to: &mut BTreeSet<Plane>, from: &BTreeSet<Plane>, cube: &Cuboid) {
        let mut has_incomplete_plane = from.iter()
            .find(|plane| **plane.is_constrained_to(&constraint))
            .is_none();
        if has_incomplete_plane {
            from.iter()
                .flat_map(|plane| plane.constrain(&cube))
                .for_each(|plane| to.push(plane));
        }
    }
    fn add_all_planes(&mut self, other: &Cuboid) {
        Cuboid::add_planes(&mut self.xy, &other.xy, self);
        Cuboid::add_planes(&mut self.xz, &other.xz, self);
        Cuboid::add_planes(&mut self.yz, &other.yz, self);
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
                    let mut cuboid = Cuboid::from_dims(x.clone(), y.clone(), z.clone());
                    if self.contains(&cuboid) {
                        let new_excluded = filtered_excluded.iter()
                            .filter(|point| cuboid.contains_point(point))
                            .copied()
                            .collect::<BTreeSet<_>>();
                        cuboid.excluded = new_excluded;
                        my_cuboids.push(cuboid);
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
                        cuboid.exclude_all_planes(&other);
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
                if left_cube.overlaps(&right_cube, false) {
                    let (split_left, split_right) = left_cube.split(&right_cube);
                    new_left = Some(split_left);
                    split_right.into_iter().for_each(|cube| new_right_cubes.push(cube));
                    split = true;
                    break;
                } else {
                    if left_cube.overlaps(&right_cube, true) {
                        left_cube.add_all_planes(&right_cube);
                    }
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
                split_left.drain(0..split_left.len())
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
            new_cubes.into_iter().for_each(|cube| self.cubes.push(cube));
        } else {
            let mut new_cubes = Vec::<Cuboid>::new();
            while let Some(cube) = self.cubes.pop() {
                if cube.overlaps(&new_cuboid, true) {
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
        let mut lit_edge_points = BTreeSet::new();
        let mut inner_volume = 0;
        let mut index = 0;
        let count = self.cubes.len();
        for cube in self.cubes.iter() {
            index += 1;
            let this_inner = cube.inner_volume(); 
            inner_volume += this_inner;
            // edges overlap, so we have to distinct them
            cube.add_edge_points(&mut lit_edge_points);
            cube.excluded.iter().for_each(|point| { lit_edge_points.remove(point); });
            print!("\rCalculating lit for {} of {} = {}      ", index, count, inner_volume + lit_edge_points.len() as u64);
        }
        println!("");
        inner_volume + lit_edge_points.len() as u64
    }
}

fn part_one(file_name: &str) {
    let mut reactor = Reactor::new();
    let mut all_points = BTreeSet::new();

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| parse_line(line))
        .flat_map(|instruction| bound_instruction(instruction))
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
    // part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
