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

#[derive(Copy, Clone)]
enum Direction { X, Y }

#[derive(Copy, Clone)]
struct Fold {
    direction: Direction,
    offset: i32
}

impl Fold {
    fn new(direction: Direction, offset: i32) -> Fold {
        Fold {
            direction,
            offset
        }
    }
}

struct ActivationCode {
    points: HashSet<(i32, i32)>,
    folds: Vec<Fold>
}

impl ActivationCode {

    fn from_file(file_name: &str) -> ActivationCode {
        let mut lines = get_file_lines(file_name)
            .flat_map(|line| line.ok());
        
        let points = (&mut lines).take_while(|line| !line.is_empty())
            .map(|line| {
                let numbers = line.split(",")
                    .map(|number| number.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                (numbers[0], numbers[1])
            })
            .collect::<HashSet<_>>();
        
        let folds = lines.map(|line| {
                let mut pieces = line.split("=");
                let dir = pieces.next();
                let offset = pieces.next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                match dir {
                    Some("fold along x") => Fold::new(Direction::X, offset),
                    Some("fold along y") => Fold::new(Direction::Y, offset),
                    _ => panic!("Unknown line: {}", line)
                }
            })
            .collect::<Vec<_>>();
        
        ActivationCode { points, folds } 
    }

    fn fold(&mut self, fold: Fold) {
        match fold.direction {
            Direction::X => self.do_fold(fold.offset,
                    |(x, _)| *x, 
                    |(_, y), x| (x, *y)),
            Direction::Y => self.do_fold(fold.offset,
                    |(_, y)| *y, 
                    |(x, _), y| (*x, y)),
        }
    }

    fn do_fold<G,C>(&mut self, offset: i32, get_coord: G, create_point: C) where 
        G: Fn(&(i32, i32)) -> i32,
        C: Fn(&(i32, i32), i32) -> (i32, i32) {
        let mut new_points = self.points.iter()
            .filter(|point| get_coord(*point) < offset)
            .copied()
            .collect::<HashSet<_>>();
        
        self.points.iter()
            .filter(|point| get_coord(*point) > offset)
            .for_each(|point| {
                let v = get_coord(point) - offset;
                let v = offset - v;
                new_points.insert(create_point(point, v));
            });
        
        self.points = new_points;
    }

    fn print(&self) {
        let x_max = self.points.iter()
            .map(|(x, _)| *x)
            .max()
            .unwrap() + 1;
        let y_max = self.points.iter()
            .map(|(_, y)| *y)
            .max()
            .unwrap() + 1;
        
        for y in 0..y_max {
            for x in 0..x_max {
                match self.points.contains(&(x, y)) {
                    true => print!("#"),
                    false => print!(".")
                }
            }
            println!("");
        }
    }
    
}

fn part_one(file_name: &str) {
    let mut activation = ActivationCode::from_file(file_name);

    let fold = activation.folds[0];
    activation.fold(fold);

    println!("Part 1: {}", activation.points.len());
}

fn part_two(file_name: &str) {
    let mut activation = ActivationCode::from_file(file_name);
    
    let folds = activation.folds.to_vec();
    folds.into_iter().for_each(|fold| activation.fold(fold));

    println!("Part 2:");
    activation.print();
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");
    part_two("sample.txt");

    println!("Done!");
}
