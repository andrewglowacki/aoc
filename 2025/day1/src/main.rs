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
    let turns = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| match line.chars().next().unwrap() {
            'L' => -1 * (line[1..line.len()].parse::<i32>().unwrap() % 100),
            'R' => line[1..line.len()].parse::<i32>().unwrap(),
            _   => panic!("Unexpected line start"),
        })
        .collect::<Vec<i32>>();

    let mut count = 0;
    let mut current = 50;

    for number in turns {
        current = current + number;
        if current < 0 {
            current = current + 100;
        } else {
            current = current % 100;
        }
        if current == 0 {
            count += 1;
        }
    }
    
    println!("Part 1: {}", count);
}

fn part_two(file_name: &str) {
    let turns = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| match line.chars().next().unwrap() {
            'L' => -1 * line[1..line.len()].parse::<i32>().unwrap(),
            'R' => line[1..line.len()].parse::<i32>().unwrap(),
            _   => panic!("Unexpected line start"),
        })
        .collect::<Vec<i32>>();

    let mut count = 0;
    let mut current = 50;

    for number in turns {
        if number == 0 {
            continue;
        }
        current = current + number;
        if current < 0 {
            let zero = current * -1;
            count += 1 + (zero / 100);
            if current == number {
                count -= 1;
            }
            current = (-1 * (zero % 100)) + 100;
        } else if current >= 100 {
            let zero = current - 100;
            count += 1 + (zero / 100);
            current = current % 100;
        } else if current == 0 {
            count += 1;
        }
    }
    
    println!("Part 2: {}", count);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
