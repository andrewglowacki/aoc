use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn read_initial_fishes(file_name: &str) -> Vec<u64> {
    let line = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .next()
        .unwrap();
    
    let mut fishes = vec![0; 9];
    
    line.split(",")
        .flat_map(|number_str| number_str.parse::<usize>())
        .for_each(|number| fishes[number] += 1);

    fishes
}

fn part_one(file_name: &str) {
    let mut fishes = read_initial_fishes(file_name);
    
    for _ in 0..80 {
        let new_fishes = fishes[0];
        fishes.rotate_left(1);
        fishes[6] += new_fishes;
    }
    
    println!("Part 1: {}", fishes.iter().sum::<u64>());
}

fn part_two(file_name: &str) {
    let mut fishes = read_initial_fishes(file_name);
    
    for _ in 0..256 {
        let new_fishes = fishes[0];
        fishes.rotate_left(1);
        fishes[6] += new_fishes;
    }
    
    println!("Part 2: {}", fishes.iter().sum::<u64>());
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
