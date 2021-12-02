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
    let numbers = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .flat_map(|line| line.parse::<i32>())
        .collect::<Vec<i32>>();
    
    let mut last = numbers[0];
    let mut increases = 0;
    for number in numbers {
        if number > last {
            increases += 1;
        }
        last = number;
    }

    println!("Part 1: Increases: {}", increases);
}

fn part_two(file_name: &str) {
    let mut numbers = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .flat_map(|line| line.parse::<i32>());
    
    let mut queue = vec![
        numbers.next().unwrap(), 
        numbers.next().unwrap(), 
        numbers.next().unwrap()
    ];

    let mut increases = 0;
    let mut last = queue.iter().sum::<i32>();
    let mut insert = 0;
    
    numbers.for_each(|num| {
        let new_sum = last - queue[insert] + num;
        queue[insert] = num;
        insert = (insert + 1) % 3;
        if new_sum > last {
            increases += 1;
        }
        last = new_sum;
    });

    println!("Part 2: Increases: {}", increases);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}