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
    fn distance(&self, other: &Point) -> Vector {
        let diff = self.diff(other);
        let product = self.dot(other);
        let angle = ((product / (self.magnitude() * other.magnitude())).acos() * 1000.0).round() as u32;
        Vector {
            angle,
            magnitude: diff.magnitude().round() as u32
        }
    }
    fn diff(&self, other: &Point) -> Point {
        Point::coords(
            self.x - other.x, 
            self.y - other.y, 
            self.z - other.z
        )
    }
    fn dot(&self, other: &Point) -> f64 {
        ((self.x * other.x) + (self.y * other.y) + (self.z * other.z)) as f64
    }
    fn magnitude(&self) -> f64 {
        (((self.x * self.x) + (self.y * self.y) + (self.z * self.z)) as f64).sqrt()
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Vector {
    magnitude: u32,
    angle: u32
}

struct Scanner {
    label: String,
    points: Vec<Point>,
    distances: HashMap<Vector, (Point, Point)>
}

impl Scanner {
    fn new(label: String, points: Vec<Point>) -> Scanner {
        let mut distances = HashMap::<Vector, (Point, Point)>::new();
        for i in 0..points.len() {
            let from = &points[i];
            for j in i + 1..points.len() {
                let to = &points[j];
                let distance = from.distance(to);
                assert_eq!(true, distances.insert(distance, (*from, *to)).is_none());
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
            .for_each(|(a, b)| {
                transform(a);
                transform(b);
            });
    }

    fn _print(&self) {
        println!("{}", self.label);
        let distances = self.distances.iter()
            .collect::<BTreeMap<_,_>>();
        distances.iter().for_each(|(distance, (a, b))|  {
            println!("{:?} -> {}, {}", distance, a, b)
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
            .map(|(distance, points)| (*distance, *points))
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
            
            let (my_from, my_to) = &my_points;
            let (other_from, other_to) = &other_points;
            
            let other_diff = other_from.diff(other_to);
            let mut my_diff = my_from.diff(&my_to);

            let mut count = 0;
            while my_diff != other_diff {
                permutations[count](&mut my_diff);
                count += 1;
                // panics if we run out of permutations
            }

            for i in 0..count {
                self.transform_points(|point| permutations[i](point));
            }

            // determine translation amount
            let translation = {
                if my_from.diff(other_from) == my_to.diff(other_to) {
                    other_from.diff(my_from)
                } else if my_from.diff(other_to) == my_to.diff(other_from) {
                    other_to.diff(my_from)
                } else {
                    panic!("Neither translation matches between: {:?} -> {:?}, {:?} -> {:?}", 
                        my_from, my_to, other_from, other_to);
                }
            };

            self.transform_points(|point| point.translate(&translation));
        }
        true
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
