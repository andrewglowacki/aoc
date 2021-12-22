use std::fmt::Display;
use std::fmt::Formatter;
use std::collections::BTreeMap;
use std::collections::VecDeque;
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    fn new(line: &String) -> Point {
        let numbers = line.split(",")
            .flat_map(|number| number.parse::<i32>())
            .collect::<Vec<_>>();
        Point {
            x: numbers[0],
            y: numbers[1],
            z: numbers[2]
        }
    }
    fn coords(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
    fn diff_magnitude(&self, other: &Point) -> u32 {
        self.diff(other).magnitude()
    }
    fn diff(&self, other: &Point) -> Point {
        Point::coords(
            self.x - other.x, 
            self.y - other.y, 
            self.z - other.z
        )
    }
    fn magnitude(&self) -> u32 {
        (((self.x * self.x) + (self.y * self.y) + (self.z * self.z)) as f64).sqrt().round() as u32
    }
    fn rotate(&mut self) {
        let swap = self.x;
        self.x = -self.y;
        self.y = swap;
    }
    fn flip(&mut self) {
        self.z = -self.z;
    }
    fn swap_x_and_z(&mut self) {
        let swap = self.z;
        self.z = self.x;
        self.x = swap;
    }
    fn swap_y_and_z(&mut self) {
        let swap = self.z;
        self.z = self.y;
        self.y = swap;
    }
    fn translate(&mut self, amount: &Point) {
        self.x += amount.x;
        self.y += amount.y;
        self.z += amount.z;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

struct Scanner {
    label: String,
    points: Vec<Point>,
    distances: HashMap<u32, Vec<(Point, Point)>>
}

impl Scanner {
    fn new(label: String, points: Vec<Point>) -> Scanner {
        let mut distances = HashMap::<u32, Vec<(Point, Point)>>::new();
        for i in 0..points.len() {
            let from = &points[i];
            for j in i + 1..points.len() {
                let to = &points[j];
                let distance = from.diff_magnitude(to);
                if let Some(points) = distances.get_mut(&distance) {
                    points.push((*from, *to));
                } else {
                    distances.insert(distance, vec![(*from, *to)]);
                }
            }
        }
        Scanner {
            label,
            points,
            distances
        }
    }
    
    fn transform_points<F>(&mut self, transform: F) 
        where F: Fn(&mut Point) {
        self.points.iter_mut()
            .for_each(|point| transform(point));
        self.distances.values_mut()
            .flat_map(|points| points)
            .for_each(|(a, b)| {
                transform(a);
                transform(b);
            });
    }

    fn print_points(points: &Vec<(Point, Point)>) -> String {
        let mut formatted = String::new();

        formatted.push('[');
        points.iter().for_each(|(a,b)| {
            if formatted.len() != 1 {
                formatted.push(',');
                formatted.push(' ');
            }
            formatted += format!("{}->{}", a, b).as_str();
        });
        formatted.push(']');

        formatted
    }

    fn _print(&self) {
        println!("{}", self.label);
        let distances = self.distances.iter()
            .collect::<BTreeMap<_,_>>();
        distances.iter().for_each(|(distance, points)|  {
            println!("{} -> {}", distance, Scanner::print_points(points))
        });
        println!("");
    }

    // -618,-824,-621 -> -537,-823,-458
    // diff: -81,-1,-163
    // mag(diff): same
    // mag(l): 1202.72
    // mag(r): 1084.19
    // dot: 1294436
    // angle: 6.94
    // 
    // 686,422,578 -> 605,423,415
    // diff: 81,-1,163
    // mag(diff): same
    // mag(l): 991.34
    // mag(r): 846.86
    // dot: 833406
    // angle: 6.92
    // 
    fn try_align(&mut self, other: &Scanner) -> bool {
        let common = self.distances.iter()
            .filter(|(distance, _)| other.distances.contains_key(distance))
            .map(|(distance, points)| (*distance, points.to_vec()))
            .collect::<Vec<_>>();
        
        println!("Common is {} between {} and {}", common.len(), self.label, other.label);
        // if we have at least 12 common distances, we have at least 12 common points
        if common.len() < 12 {
            return false;
        }

        let permutations: Vec<Box<dyn Fn(&mut Point)>> = vec![
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate()),
            Box::new(|point| {
                point.rotate();
                point.flip();
            }),
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate()),
            Box::new(|point| {
                point.rotate();
                point.flip();
                point.swap_x_and_z();
            }),
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate()),
            Box::new(|point| {
                point.rotate();
                point.flip();
            }),
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate()),
            Box::new(|point| {
                point.rotate();
                point.flip();
                point.swap_y_and_z();
            }),
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate()),
            Box::new(|point| {
                point.rotate();
                point.flip();
            }),
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate()),
            Box::new(|point| point.rotate())
        ];

        for (distance, my_points) in common {
            let other_points = other.distances.get(&distance).unwrap();
            
            for (my_from, my_to) in my_points {
                for (other_from, other_to) in other_points {
                    let other_diff = other_from.diff(other_to);
                    let mut my_diff = my_from.diff(&my_to);
        
                    let mut count = 0;
                    while my_diff != other_diff && count < permutations.len() {
                        let permutation = &permutations[count];
                        permutation(&mut my_diff);
                        count += 1;
                    }
                    
                    if count >= permutations.len() {
                        continue;
                    }

                    let mut my_from = my_from;
                    let mut my_to = my_to;

                    for i in 0..count {
                        let permutation = &permutations[i];
                        permutation(&mut my_from);
                        permutation(&mut my_to);
                    }
        
                    // determine translation amount
                    let translation = {
                        if my_from.diff(other_from) == my_to.diff(other_to) {
                            other_from.diff(&my_from)
                        } else if my_from.diff(other_to) == my_to.diff(other_from) {
                            other_to.diff(&my_from)
                        } else {
                            println!("No translation found: {}->{} and {}->{}", my_from, my_to, other_from, other_to);
                            continue;
                        }
                    };

                    println!("offset of {} to {} is {}", self.label, other.label, translation);
        
                    for i in 0..count {
                        self.transform_points(|point| permutations[i](point));
                    }
        
                    self.transform_points(|point| point.translate(&translation));

                    println!("-----common-----");
                    self.points.iter()
                        .filter(|point| other.points.contains(point))
                        .for_each(|point| println!("{}", point));
                    println!("-----uncommon-----");
                    self.points.iter()
                        .filter(|point| !other.points.contains(point))
                        .for_each(|point| println!("{}", point));

                    return true;
                }
            }
        }
        false
    }
}

fn read_scanners(file_name: &str) -> Vec<Scanner> {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());

    let lines = &mut lines;

    let mut scanners = Vec::new();
    while let Some(label) = lines.next() {
        let points = lines.take_while(|line| !line.is_empty())
            .map(|line| Point::new(&line))
            .collect::<Vec<_>>();
        scanners.push(Scanner::new(label, points));
    }

    scanners.iter().for_each(|scanner| scanner._print());
    scanners
}

fn part_one(file_name: &str) {
    let mut unordered = read_scanners(file_name)
        .into_iter()
        .collect::<VecDeque<_>>();
    
    let mut ordered = Vec::<Scanner>::new();

    ordered.push(unordered.pop_front().unwrap());
    
    let mut misses = 0;
    while let Some(mut next) = unordered.pop_front() {
        let found = ordered.iter()
            .find(|scanner| next.try_align(scanner))
            .is_some();
        match found {
            true => {
                ordered.push(next);
                misses = 0;
            },
            false => {
                unordered.push_back(next);
                misses += 1;
            }
        }
        if misses > 0 && misses >= unordered.len() {
            panic!("No match found after iterating through remaining
                {} unordered with {} ordered", unordered.len(), ordered.len());
        }
    }

    let count = ordered.iter()
        .flat_map(|scanner| scanner.points.iter())
        .collect::<HashSet<_>>()
        .len();
    
    println!("Part 1: {}", count);
}

fn part_two(file_name: &str) {
    let _lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("sample.txt");
    part_two("input.txt");

    println!("Done!");
}
