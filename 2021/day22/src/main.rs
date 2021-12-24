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
    fn overlaps(&self, other: &Range) -> bool {
        self.from <= other.to && self.to >= other.from
    }
    fn length(&self) -> u64 {
        max(0, (self.to - self.from) + 1) as u64
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
    fn overlaps(&self, other: &Cuboid) -> bool {
        self.dimensions.iter()
            .zip(other.dimensions.iter())
            .find(|(a, b)| !a.overlaps(b))
            .is_none()
    }
    fn volume(&self) -> u64 {
        self.dimensions.iter()
            .map(|range| range.length())
            .product::<u64>()
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

    fn subtract(&self, other: &Cuboid, add_to: &mut Vec<Cuboid>) {
        let mut my_x = self.dimensions[0].clone();
        let my_y = self.dimensions[1];
        let mut my_z = self.dimensions[2].clone();
        let other_x = &other.dimensions[0];
        let other_y = &other.dimensions[1];
        let other_z = &other.dimensions[2];

        if my_z.to > other_z.to {
            add_to.push(Cuboid::from_dims(my_x, my_y, Range::new(other_z.to + 1, my_z.to)));
            my_z.to = other_z.to;
        }
        if my_z.from < other_z.from {
            add_to.push(Cuboid::from_dims(my_x, my_y, Range::new(my_z.from, other_z.from - 1)));
            my_z.from = other_z.from;
        }
        if my_x.to > other_x.to {
            add_to.push(Cuboid::from_dims(Range::new(other_x.to + 1, my_x.to), my_y, my_z));
            my_x.to = other_x.to;
        }
        if my_x.from < other_x.from {
            add_to.push(Cuboid::from_dims(Range::new(my_x.from, other_x.from - 1), my_y, my_z));
            my_x.from = other_x.from;
        }
        if my_y.to > other_y.to {
            add_to.push(Cuboid::from_dims(my_x, Range::new(other_y.to + 1, my_y.to), my_z));
        }
        if my_y.from < other_y.from {
            add_to.push(Cuboid::from_dims(my_x, Range::new(my_y.from, other_y.from - 1), my_z));
        }
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
    fn remove_old_from_new(old_cubes: &mut Vec<Cuboid>, new_cube: Cuboid) {
        let mut new_cubes = vec![new_cube];
        
        for old_cube in old_cubes.iter() {
            let mut split_new_cubes = Vec::new();

            // subtract this old cube from all of the new cubes
            for new_cube in new_cubes {
                if old_cube.overlaps(&new_cube) {
                    new_cube.subtract(old_cube, &mut split_new_cubes);
                } else {
                    split_new_cubes.push(new_cube);
                }
            }

            new_cubes = split_new_cubes;
        }

        // add the new cubes
        new_cubes.into_iter().for_each(|cube| old_cubes.push(cube));

    }

    fn set_cubes(&mut self, on: bool, new_cuboid: Cuboid) {
        if on {
            Reactor::remove_old_from_new(&mut self.cubes, new_cuboid);
        } else {
            let mut new_cubes = Vec::<Cuboid>::new();
            while let Some(cube) = self.cubes.pop() {
                if cube.overlaps(&new_cuboid) {
                    cube.subtract(&new_cuboid, &mut new_cubes);
                } else {
                    new_cubes.push(cube);
                }
            }
            self.cubes = new_cubes;
        }
    }

    fn calc_lit_count(&self) -> u64 {
        let mut volume_total = 0;
        for cube in self.cubes.iter() {
            let this_volume = cube.volume();
            volume_total += this_volume;
        }
        volume_total
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
            reactor.set_cubes(on, cuboid);
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
            reactor.set_cubes(on, cuboid);
            index += 1;
        });
    
    let lit = reactor.calc_lit_count();

    println!("Part 2: {}", lit);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
