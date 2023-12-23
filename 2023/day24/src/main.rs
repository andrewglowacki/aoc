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

struct Vector {
    x: i32,
    y: i32,
    z: i32
}

impl Vector {
    fn new(x: i32, y: i32, z: i32) -> Vector {
        Vector { x, y, z }
    }
    fn is_xy_same(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Hailstone {
    position: Vector,
    velocity: Vector
}

impl Hailstone {
    fn parse(line: String) -> Hailstone {
        let mut parts = line.split(" @ ");
        let positions = parts.next().unwrap()
            .split(", ")
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let velocities = parts.next().unwrap()
            .split(", ")
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let position = Vector::new(positions[0], positions[1], positions[2]);
        let velocity = Vector::new(velocities[0], velocities[1], velocities[2]);

        Hailstone { position, velocity }
    }

    fn find_xy_intersection(&self, other: &Hailstone) -> Option<(f64, f64)> {
        if self.velocity.is_xy_same(&other.velocity) {
            if self.position.is_xy_same(&other.position) {
                None
            } else {
                Some((self.position.x as f64, self.position.y as f64))
            }
        } else {
            // xX + yY = aA + bB
            // 
            // from 7 to 27
            // Hailstone A: 19, 13, 30 @ -2, 1, -2
            // Hailstone B: 18, 19, 22 @ -1, -1, -2
            // Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).
            // 
            // 19 + -2x = 18 + -1x
            // 19 + x = 18
            // x = -1
            //
            // 13 + y = 19 -y
            // 13 + 2y = 19
            // 2y = 19 - 13
            // 2y = 6
            // y = 3
            // 
            // 
            None
        }
    }
}

fn count_intersections(hailstones: Vec<Hailstone>) -> usize {
    let mut intersections = HashSet::new();

    for i in 0..hailstones.len() {
        let left = &hailstones[i];
        for j in i + 1..hailstones.len() {
            let right = &hailstones[j];
            if let Some((x, y)) = left.find_xy_intersection(&right) {
                intersections.insert(i);
                intersections.insert(j);
            }
        }
    }

    intersections.len()
}

fn part_one(file_name: &str) {
    let hailstones = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Hailstone::parse(line))
        .collect::<Vec<_>>();

    let intersections = count_intersections(hailstones);
    
    println!("Part 1: {}", intersections);
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("sample.txt");
    part_two("sample.txt");

    println!("Done!");
}
