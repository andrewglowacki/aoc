use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::HashSet;

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn translate(point: &mut (i32, i32), x: i32, y: i32) {
    point.0 += x;
    point.1 += y;
}

fn parse_point(line: &str) -> (i32, i32) {
    let mut chars = line.chars();
    let mut point = (0, 0);
    while let Some(c) = chars.next() {
        match c {
            'n' => match chars.next().unwrap() {
                'e' => translate(&mut point, 1, 1),
                'w' => translate(&mut point, -1, 1),
                _ => panic!("Invalid character: {}", line)
            },
            's' => match chars.next().unwrap() {
                'e' => translate(&mut point, 1, -1),
                'w' => translate(&mut point, -1, -1),
                _ => panic!("Invalid character: {}", line)
            },
            'e' => translate(&mut point, 2, 0),
            'w' => translate(&mut point, -2, 0),
            _ => panic!("Invalid character {} in: {}", c, line)
        }
    }
    point
}

const NEIGHBORS: [(i32, i32); 6] =  [
    (-1, 1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-2, 0),
    (2, 0)
];

fn count_adjacent(black_tiles: &HashSet<(i32, i32)>, x: i32, y: i32) -> u8 {
    NEIGHBORS.iter().map(|neighbor| {
        let neighbor = &(neighbor.0 + x, neighbor.1 + y);
        black_tiles.contains(neighbor) as u8
    })
    .sum()
}

fn check_white_tiles(old_tiles: &HashSet<(i32, i32)>, new_tiles: &mut HashSet<(i32, i32)>, center: &(i32, i32)) {
    let (center_x, center_y) = *center;
    for neighbor in NEIGHBORS.iter() {
        let x = neighbor.0 + center_x;
        let y = neighbor.1 + center_y;
        if old_tiles.contains(&(x, y)) {
            continue;
        }
        let adjacent = count_adjacent(old_tiles, x, y);
        if adjacent == 2 {
            new_tiles.insert((x, y));
        }
    }
}

fn flip_tiles(black_tiles: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_tiles = HashSet::new();
    for point in black_tiles.iter() {
        let (x, y) = *point;
        let adjacent = count_adjacent(&black_tiles, x, y);
        if adjacent == 1 || adjacent == 2 {
            new_tiles.insert((x, y));
        }
        check_white_tiles(&black_tiles, &mut new_tiles, point);
    }
    new_tiles
}

fn test_input(file_name: &str) {
    let mut black_tiles = HashSet::<(i32, i32)>::new();
    let points = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| parse_point(&line));
    
    for point in points {
        if !black_tiles.remove(&point) {
            black_tiles.insert(point);
        }
    }

    println!("For {}, there are {} tiles black side up", file_name, black_tiles.len());

    for _ in 0..100 {
        black_tiles = flip_tiles(black_tiles);
    }

    println!("For {}, after 100 days there are {} tiles black side up", file_name, black_tiles.len());
}

fn main() {
    test_input("sample.txt");
    test_input("input.txt");
}
