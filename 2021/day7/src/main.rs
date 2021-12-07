use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn read_positions(file_name: &str) -> Vec<i64> {
    let numbers = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .next()
        .unwrap()
        .split(",")
        .flat_map(|number| number.parse::<i64>())
        .collect::<Vec<_>>();
    
    let mut positions = vec![0; (*numbers.iter().max().unwrap() + 1) as usize];

    for number in numbers {
        positions[number as usize] += 1;
    }
        
    positions
}

fn part_one(file_name: &str) {
    let positions = read_positions(file_name);

    let max = positions.len();

    let mut min_cost = 1000000;
    let mut min_pos = 0;
    for i in 0..max {
        let mut cost: i64 = 0;
        let target = i as i64;
        for l in 0..i {
            cost += (target - (l as i64)) * positions[l];
        }
        for u in (i+1)..max {
            cost += ((u as i64) - target) * positions[u];
        }
        if cost < min_cost {
            min_cost = cost as i64;
            min_pos = i;
        }
    }
    
    println!("Part 1: {} at {}", min_cost, min_pos);
}

fn part_two(file_name: &str) {
    let positions = read_positions(file_name);

    let max = positions.len();

    let mut min_cost = 1000000000;
    let mut min_pos = 0;
    for i in 0..max {
        let mut cost: i64 = 0;
        let target = i as i64;
        for l in 0..i {
            let distance = target - (l as i64);
            let single_cost = (distance * (distance + 1)) / 2;
            cost += single_cost * positions[l];
        }
        for u in (i+1)..max {
            let distance = (u as i64) - target;
            let single_cost = (distance * (distance + 1)) / 2;
            cost += single_cost * positions[u];
        }
        if cost < min_cost {
            min_cost = cost as i64;
            min_pos = i;
        }
    }
    
    println!("Part 2: {} at {}", min_cost, min_pos);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
