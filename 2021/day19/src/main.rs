use std::mem::swap;
use std::cmp::max;
use std::f64::consts::PI;
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    dims: Vec<i32>
}

impl Point {
    fn new(line: &String) -> Point {
        let numbers = line.split(",")
            .flat_map(|number| number.parse::<i32>())
            .collect::<Vec<_>>();
        Point {
            dims: numbers
        }
    }
    fn coords(x: i32, y: i32, z: i32) -> Point {
        Point { dims: vec![x, y, z] }
    }
    fn diff_magnitude(&self, other: &Point) -> u32 {
        self.diff(other).magnitude() as u32
    }
    fn angle(&self, other: &Point) -> u16 {
        let dot = self.dot_product(other) as f64;
        let my_magnitude = self.magnitude();
        let other_magnitude = other.magnitude();

        ((dot / (my_magnitude * other_magnitude)).acos() * (180.0 / PI)).round() as u16
    }
    fn x(&self) -> i32 {
        self.dims[0]
    }
    fn y(&self) -> i32 {
        self.dims[1]
    }
    fn z(&self) -> i32 {
        self.dims[2]
    }
    fn dot_product(&self, other: &Point) -> i32 {
        self.dims.iter()
            .zip(other.dims.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
    fn diff(&self, other: &Point) -> Point {
        Point::coords(
            self.x() - other.x(), 
            self.y() - other.y(), 
            self.z() - other.z()
        )
    }
    fn magnitude(&self) -> f64 {
        (self.dims.iter()
            .map(|d| d * d)
            .sum::<i32>() as f64)
            .sqrt()
    }
    fn translate(&mut self, amount: &Point) {
        for i in 0..3 {
            self.dims[i] -= amount.dims[i];
        }
    }
    fn taxicab_distance(&self, other: &Point) -> u64 {
        self.diff(other).dims.iter()
            .map(|d| d.abs() as u64)
            .sum::<u64>()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x(), self.y(), self.z())
    }
}

#[derive(Debug)]
struct Permutation {
    positions: Vec<usize>,
    negations: Vec<i32>
}

impl Permutation {
    fn new(a: &Point, b: &Point) -> Option<Permutation> {
        let a_components = a.dims.iter()
            .map(|d| d.abs())
            .collect::<HashSet<_>>();
        let b_components = b.dims.iter()
            .map(|d| d.abs())
            .collect::<HashSet<_>>();
        if a_components.len() != 3 || a_components != b_components {
            None
        } else {
            let mut positions = Vec::with_capacity(3);
            let mut negations = Vec::with_capacity(3);
            for i in 0..3 {
                let mut new_pos = 0;
                for j in 0..3 {
                    if a.dims[i].abs() == b.dims[j].abs() {
                        new_pos = j;
                        break;
                    }
                }
                if b.dims[new_pos] == 0 {
                    negations.push(1);
                } else {
                    negations.push(a.dims[i] / b.dims[new_pos]);
                }
                positions.push(new_pos);
            }
            Some(Permutation {
                positions,
                negations
            })
        }
    }

    fn apply(&self, point: &mut Point) {
        let mut new_dims = Vec::with_capacity(3);
        new_dims.push(0);
        new_dims.push(0);
        new_dims.push(0);
        
        new_dims[self.positions[0]] = point.dims[0] * self.negations[0];
        new_dims[self.positions[1]] = point.dims[1] * self.negations[1];
        new_dims[self.positions[2]] = point.dims[2] * self.negations[2];

        point.dims = new_dims;
    }
}

struct Scanner {
    label: String,
    points: Vec<Point>,
    distances: HashMap<(u32, u16), Vec<(Point, Point)>>,
    translation: Option<Point>
}

impl Scanner {
    fn new(label: String, points: Vec<Point>) -> Scanner {
        let mut distances = HashMap::<(u32, u16), Vec<(Point, Point)>>::new();
        for i in 0..points.len() {
            let from = &points[i];
            for j in i + 1..points.len() {
                let to = &points[j];
                let distance = from.diff_magnitude(to);
                let angle = from.angle(to);
                if let Some(points) = distances.get_mut(&(distance, angle)) {
                    points.push((from.clone(), to.clone()));
                } else {
                    distances.insert((distance, angle), vec![(from.clone(), to.clone())]);
                }
            }
        }
        Scanner {
            label,
            points,
            distances,
            translation: None
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
        distances.iter().for_each(|((diff, angle), points)|  {
            println!("({}, {}) -> {}", diff, angle, Scanner::print_points(points))
        });
        println!("");
    }

    fn determine_orientation(
        my_from: Point, my_to: Point, 
        other_from: Point, other_to: Point) -> Option<(Permutation, Point)>
    {
        let my_diff = my_from.diff(&my_to);
        let other_diff = other_from.diff(&other_to);

        let permutation = match Permutation::new(&my_diff, &other_diff) {
            Some(permutation) => permutation,
            None => {
                return None;
            }
        };

        let mut my_from = my_from.clone();
        let mut my_to = my_to.clone();

        permutation.apply(&mut my_from);
        permutation.apply(&mut my_to);

        // determine translation amount
        let translation = {
            let from_from = my_from.diff(&other_from);
            let from_to = my_from.diff(&other_to);
            if my_to.diff(&from_from) == other_to {
                from_from
            } else if my_to.diff(&from_to) == other_from {
                from_to
            } else {
                return None;
            }
        };

        Some((permutation, translation))
    }

    fn transform_if_minimum_common(&mut self, other: &Scanner, permutation: Permutation, translation: Point) -> bool {
        let mut points = self.points.to_vec();
        points.iter_mut().for_each(|point| {
            permutation.apply(point);
            point.translate(&translation);
        });

        let common_count = points.iter()
            .filter(|point| other.points.contains(point))
            .count();
        
        if common_count >= 12 {
            self.transform_points(|point| {
                permutation.apply(point);
                point.translate(&translation);
            });
            true
        } else {
            false
        }
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
    fn try_align(&mut self, other: &Scanner, use_angles: bool) -> bool {
        let common = match use_angles {
            true => {
                self.distances.iter() 
                    .filter(|((diff, angle), _)| {
                        (*angle > 0 && other.distances.contains_key(&(*diff, angle - 1))) || 
                        other.distances.contains_key(&(*diff, *angle)) || 
                        other.distances.contains_key(&(*diff, angle + 1))
                    })
                    .map(|(distance, points)| (*distance, points.to_vec()))
                    .collect::<Vec<_>>()
            }
            false => {
                self.distances.iter() 
                    .filter(|((diff, _), _)| {
                        other.distances.keys()
                            .find(|(other_diff, _)| diff == other_diff)
                            .is_some()
                    })
                    .map(|(distance, points)| (*distance, points.to_vec()))
                    .collect::<Vec<_>>()
            }
        };
        
        for ((distance, angle), my_points) in common {
            let mut other_points = Vec::new();
            match use_angles {
                true => {
                    let start_angle = max(0, (angle as i32) - 1) as u16;
                    for angle in start_angle..(angle + 2) {
                        other.distances.get(&(distance, angle)).map(|points| 
                            points.iter().for_each(|point| other_points.push(point)));
                    }
                }
                false => {
                    other.distances.iter()
                        .filter(|((other_distance, _), _)| *other_distance == distance)
                        .flat_map(|(_, points)| points) 
                        .for_each(|point| other_points.push(point));
                }
            }
            
            for (my_from, my_to) in my_points.iter() {
                for (other_from, other_to) in other_points.iter() {
                    if let Some((permutation, translation)) = Scanner::determine_orientation(
                        my_to.clone(), my_from.clone(), other_from.clone(), other_to.clone()) 
                    {
                        if self.transform_if_minimum_common(other, permutation, translation.clone()) {
                            self.translation = Some(translation);
                            return true;
                        }
                    } 
                    else if let Some((permutation, translation)) = Scanner::determine_orientation(
                        my_from.clone(), my_to.clone(), other_from.clone(), other_to.clone())
                    {
                        if self.transform_if_minimum_common(other, permutation, translation.clone()) {
                            self.translation = Some(translation);
                            return true;
                        }
                    }
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

    scanners
}

fn order_scanners(file_name: &str) -> Vec<Scanner> {
    let mut unordered = read_scanners(file_name)
        .into_iter()
        .collect::<VecDeque<_>>();
    
    let mut ordered = Vec::<Scanner>::new();

    let mut first = unordered.pop_front().unwrap();
    first.translation = Some(Point::coords(0, 0, 0));
    ordered.push(first);
    
    let mut use_angles = true;
    let mut misses = 0;
    while let Some(mut next) = unordered.pop_front() {
        let found = ordered.iter()
            .find(|scanner| next.try_align(scanner, use_angles))
            .is_some();
        match found {
            true => {
                ordered.push(next);
                misses = 0;
                use_angles = true;
            },
            false => {
                unordered.push_back(next);
                misses += 1;
            }
        }
        if misses >= unordered.len() {
            if use_angles {
                use_angles = false;
            } else {
                panic!("No match found after iterating through remaining
                    {} unordered with {} ordered", unordered.len(), ordered.len());
            }
        }
    }
    ordered
}

fn part_one(file_name: &str) {
    let ordered = order_scanners(file_name);

    let count = ordered.iter()
        .flat_map(|scanner| scanner.points.iter())
        .collect::<HashSet<_>>()
        .len();
    
    println!("Part 1: {}", count);
}

fn part_two(file_name: &str) {
    let ordered = order_scanners(file_name);

    let mut furthest = 0;
    for i in 0..ordered.len() {
        let one = &ordered[i];
        let one_offset = one.translation.as_ref().unwrap();
        for j in 0..ordered.len() {
            if i == j {
                continue;
            }
            let two = &ordered[j];
            let two_offset = two.translation.as_ref().unwrap();
            let distance = two_offset.taxicab_distance(one_offset);
            if distance > furthest {
                furthest = distance;
            }
        }
    }

    println!("Part 2: {}", furthest);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
