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

#[derive(Debug)]
enum Type {
    Rock,
    Exterior
}

struct Droplet {
    x_min: i32,
    x_max: i32, 
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
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

        points.iter().for_each(|point| {
            types.insert(*point, Type::Rock);
        });

        Droplet {
            x_min, 
            x_max,
            y_min, 
            y_max,
            z_min,
            z_max,
            types
        }
    }

    fn add_side_types<F>(
        &mut self, 
        a_min: i32, 
        a_max: i32, 
        b_min: i32, 
        b_max: i32, 
        plane_min: i32, 
        plane_max: i32,
        point_maker: F) 
        where F: Fn(i32, i32, i32) -> (i32, i32, i32)
    {
        for a in a_min..a_max + 1 {
            for b in b_min..b_max + 1 {
                let point = point_maker(a, b, plane_min);
                if self.types.get(&point).is_none() {
                    self.types.insert(point, Type::Exterior);
                }
                
                let point = point_maker(a, b, plane_min - 1);
                self.types.insert(point, Type::Exterior);
                
                let point = point_maker(a, b, plane_max);
                if self.types.get(&point).is_none() {
                    self.types.insert(point, Type::Exterior);
                }
                
                let point = point_maker(a, b, plane_max + 1);
                self.types.insert(point, Type::Exterior);
            }
        }
    }

    fn is_exterior(&self, point: (i32, i32, i32)) -> bool {
        match self.types.get(&point) {
            Some(Type::Exterior) => true,
            _ => false
        }
    }

    fn connect_exteriors<F>(
        &mut self, 
        a_min: i32, a_max: i32, 
        b_min: i32, b_max: i32,
        c_min: i32, c_max: i32,
        point_maker: F)
        where F: Fn(i32, i32, i32) -> (i32, i32, i32)
    {
        for a in a_min..a_max + 1 {
            for b in b_min..b_max + 1 {
                for c in c_min..c_max + 1 {
                    let has_adjacent_exterior =
                        self.is_exterior(point_maker(a - 1, b, c)) ||
                        self.is_exterior(point_maker(a + 1, b, c)) ||
                        self.is_exterior(point_maker(a, b - 1, c)) ||
                        self.is_exterior(point_maker(a, b + 1, c)) ||
                        self.is_exterior(point_maker(a, b, c - 1)) ||
                        self.is_exterior(point_maker(a, b, c + 1));
                        
                    if has_adjacent_exterior {
                        let point = point_maker(a, b, c);
                        if !self.types.contains_key(&point) {
                            self.types.insert(point, Type::Exterior);
                        }
                    }                    
                }
            }
        }
        
        for a in a_min..a_max + 1 {
            let a = a_max - a;
            for b in b_min..b_max + 1 {
                let b = b_max - b;
                for c in c_min..c_max + 1 {
                    let c = c_max - c;
                    let has_adjacent_exterior =
                        self.is_exterior(point_maker(a - 1, b, c)) ||
                        self.is_exterior(point_maker(a + 1, b, c)) ||
                        self.is_exterior(point_maker(a, b - 1, c)) ||
                        self.is_exterior(point_maker(a, b + 1, c)) ||
                        self.is_exterior(point_maker(a, b, c - 1)) ||
                        self.is_exterior(point_maker(a, b, c + 1));
                        
                    if has_adjacent_exterior {
                        let point = point_maker(a, b, c);
                        if !self.types.contains_key(&point) {
                            self.types.insert(point, Type::Exterior);
                        }
                    }                    
                }
            }
        }
    }

    fn count_exterior_points(&mut self, sides: &HashMap<(i32, i32, i32), u32>) -> u32 {
        //
        // mark boundaries
        //
        
        // top and bottom
        self.add_side_types(
            self.x_min, self.x_max, 
            self.y_min, self.y_max, 
            self.z_min, self.z_max, 
            |x, y, z| (x, y, z));
        
        // left and right
        self.add_side_types(
            self.x_min, self.x_max, 
            self.z_min, self.z_max, 
            self.y_min, self.y_max, 
            |x, z, y| (x, y, z));
            
        // front and back
        self.add_side_types(
            self.z_min, self.z_max, 
            self.y_min, self.y_max, 
            self.x_min, self.x_max, 
            |z, y, x| (x, y, z));
      
        // find unmarked spaces adjacent to exteriors
        // these are also exteriors
        self.connect_exteriors(
            self.x_min, self.x_max, 
            self.y_min, self.y_max, 
            self.z_min, self.z_max, 
            |x, y, z| (x, y, z));
            
        self.connect_exteriors(
            self.x_min, self.x_max, 
            self.z_min, self.z_max, 
            self.y_min, self.y_max, 
            |x, z, y| (x, y, z));
            
        self.connect_exteriors(
            self.z_min, self.z_max, 
            self.y_min, self.y_max, 
            self.x_min, self.x_max, 
            |z, y, x| (x, y, z));
            
        self.connect_exteriors(
            self.z_min, self.z_max, 
            self.x_min, self.x_max, 
            self.y_min, self.y_max, 
            |z, y, x| (x, y, z));

        self.connect_exteriors(
            self.y_min, self.y_max, 
            self.x_min, self.x_max, 
            self.z_min, self.z_max, 
            |y, x, z| (x, y, z));

        self.connect_exteriors(
            self.y_min, self.y_max, 
            self.z_min, self.z_max, 
            self.x_min, self.x_max, 
            |y, x, z| (x, y, z));

        // filter the sides down to only exteriors and return the sum
        sides.iter()
            .filter(|(point, _)| self.is_exterior(**point))
            .map(|(_, count)| count)
            .sum()
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

    let total = droplet.count_exterior_points(&sides);

    for z in droplet.z_min - 1..droplet.z_max + 2 {
        println!("----- Z {} -----", z);
        for y in droplet.y_min - 1..droplet.y_max + 2 {
            for x in droplet.x_min - 1..droplet.x_max + 2 {
                if let Some(t) = droplet.types.get(&(x, y, z)) {
                    let symbol = match t {
                        Type::Rock => '#',
                        Type::Exterior => '~'
                    };
                    print!("{}", symbol);
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
    
    println!("Part 2: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
