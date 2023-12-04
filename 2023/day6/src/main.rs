use core::num;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Race {
    time: u64,
    distance: u64
}

impl Race {
    fn new(time: u64, distance: u64) -> Race {
        Race { time, distance }
    }
    /**
     * distance = Tp * (time - Tp)
     */
    fn get_win_count(&self) -> u64 {
        (1..self.time).into_iter()
            .map(|t| t * (self.time - t))
            .filter(|distance| *distance > self.distance)
            .count() as u64
    }
}

fn parse_races(file_name: &str) -> Vec<Race> {
    let numbers = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|number| number.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    numbers[0].iter()
        .zip(numbers[1].iter())
        .map(|(time, distance)| Race::new(*time, *distance))
        .collect::<Vec<_>>()
}

fn part_one(file_name: &str) {
    let races = parse_races(file_name);

    let product = races.into_iter()
        .map(|race| race.get_win_count())
        .product::<u64>();
    
    println!("Part 1: {}", product);
}

fn part_two(file_name: &str) {
    let numbers = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .chars()
                .filter(|c| *c != ' ')
                .collect::<String>()
        })
        .map(|number| number.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let time = numbers[0];
    let distance = numbers[1];
    let race = Race::new(time, distance);

    println!("Part 2: {}", race.get_win_count());
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
