use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn part_one(file_name: &str) {
    let mut horizontal = 0;
    let mut depth = 0;

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| {
            let pieces = line.split(" ").collect::<Vec<&str>>();
            let command = pieces[0].to_owned();
            let amount = pieces[1].parse::<i32>().unwrap();
            (command, amount)
        })
        .for_each(|(command, amount)| {
            match command.as_str() {
                "up" => depth -= amount,
                "down" => depth += amount,
                "forward" => horizontal += amount,
                _ => panic!("Invalid command: {}", command)
            };
        });
    
    println!("Part 1: {} * {} = {}", horizontal, depth, (horizontal * depth));
}

fn part_two(file_name: &str) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| {
            let pieces = line.split(" ").collect::<Vec<&str>>();
            let command = pieces[0].to_owned();
            let amount = pieces[1].parse::<i32>().unwrap();
            (command, amount)
        })
        .for_each(|(command, amount)| {
            match command.as_str() {
                "up" => aim -= amount,
                "down" => aim += amount,
                "forward" => { 
                    horizontal += amount;
                    depth += amount * aim;
                },
                _ => panic!("Invalid command: {}", command)
            };
        });
    
    println!("Part 2: {} * {} = {}", horizontal, depth, (horizontal * depth));
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
