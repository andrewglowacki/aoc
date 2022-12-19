use std::collections::{HashSet, HashMap, BTreeSet};
use std::fs::File;
use std::ops::Range;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}


fn parse_point(line: String) -> (i32, i32, i32) {
    let parts = line.split(",")
        .map(|part| part.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    (parts[0], parts[1], parts[2])
}

fn x(point: &(i32, i32, i32), x: i32) -> (i32, i32, i32) {
    let mut copy = point.to_owned();
    copy.0 += x;
    copy
}

fn y(point: &(i32, i32, i32), y: i32) -> (i32, i32, i32) {
    let mut copy = point.to_owned();
    copy.1 += y;
    copy
}

fn z(point: &(i32, i32, i32), z: i32) -> (i32, i32, i32) {
    let mut copy = point.to_owned();
    copy.2 += z;
    copy
}

fn add_non_touching_side(point: (i32, i32, i32), points: &HashSet<(i32, i32, i32)>, sides: &mut HashMap<(i32, i32, i32), u32>) {
    if !points.contains(&point) {
        if let Some(count) = sides.get_mut(&point) {
            *count += 1;
        } else {
            sides.insert(point, 1);
        }
    }
}

fn get_non_touching_sides(point: &(i32, i32, i32), points: &HashSet<(i32, i32, i32)>, sides: &mut HashMap<(i32, i32, i32), u32>) {
    add_non_touching_side(x(point, 1), points, sides);
    add_non_touching_side(x(point, -1), points, sides);
    add_non_touching_side(y(point, 1), points, sides);
    add_non_touching_side(y(point, -1), points, sides);
    add_non_touching_side(z(point, 1), points, sides);
    add_non_touching_side(z(point, -1), points, sides);
}

fn part_one(file_name: &str) {
    let points = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| parse_point(line))
        .collect::<HashSet<_>>();
    
    let mut sides = HashMap::new();

    points.iter().for_each(|point| get_non_touching_sides(point, &points, &mut sides));
    
    let total = sides.values().sum::<u32>();

    println!("Part 1: {}", total);
}

fn add_point_to_map(point: (i32, i32), third: i32, map: &mut HashMap<(i32, i32), BTreeSet<i32>>) {
    if let Some(set) = map.get_mut(&point) {
        set.insert(third);
    } else {
        let mut set = BTreeSet::new();
        set.insert(third);
        map.insert(point, set);
    }
}

enum Type {
    Rock,
    Unknown,
    Interior,
    Exterior
}

struct Droplet {
    x_range: Range<i32>,
    y_range: Range<i32>,
    z_range: Range<i32>,
    types: HashMap<(i32, i32, i32), Type>
}

impl Droplet {
    fn create(points: &HashSet<(i32, i32, i32)>) -> Droplet {
        let mut x_min = i32::MAX;
        let mut x_max = 0;
        let mut y_min = i32::MAX;
        let mut y_max = 0;
        let mut z_min = i32::MAX;
        let mut z_max = 0;

        let mut types = HashMap::new();
        
        points.iter().for_each(|(x, y, z)| {
            x_max = x_max.max(*x);
            x_min = x_min.min(*x);
            y_max = y_max.max(*y);
            y_min = y_min.min(*y);
            z_max = z_max.max(*z);
            z_min = z_min.min(*z);
        });

        let x_range = x_min..x_max + 1;
        let y_range = y_min..y_max + 1;
        let z_range = z_min..z_max + 1;

        for z in z_range.to_owned() {
            for y in y_range.to_owned() {
                for x in x_range.to_owned() {
                    let point = (x, y, z);
                    if points.contains(&point) {
                        types.insert(point, Type::Rock);
                    } else {
                        types.insert(point, Type::Unknown);
                    }
                }
            }
        }

        Droplet {
            x_range, 
            y_range,
            z_range, 
            types
        }
    }

    fn find_interior_points(&mut self, sides: &HashMap<(i32, i32, i32), u32>) {
        for (point, _) in sides {
            if self.types.contains_key(point) {
                continue;
            }


        }
    }
}

fn part_two(file_name: &str) {
    let points = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| parse_point(line))
        .collect::<HashSet<_>>();
        
    let mut sides = HashMap::new();

    points.iter().for_each(|point| get_non_touching_sides(point, &points, &mut sides));

    let mut droplet = Droplet::create(&points);

    droplet.find_interior_points(&sides);
    
    let total = sides.values().sum::<u32>();

    println!("Part 2: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
