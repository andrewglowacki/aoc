use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Debug)]
enum Rock {
    Round(usize),
    Square(usize)
}

struct Platform {
    rocks: Vec<Vec<Rock>>,
    height: usize
}

impl Platform {
    fn parse(file_name: &str) -> Platform {

        let mut rocks = Vec::new();
        let mut height = 0;

        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .for_each(|line| {
                if rocks.is_empty() {
                    for _ in 0..line.len() {
                        rocks.push(Vec::new());
                    }
                }
                line.char_indices().for_each(|(x, c)| {
                    match c {
                        '#' => rocks[x].push(Rock::Square(height)),
                        'O' => rocks[x].push(Rock::Round(height)),
                        _ => ()
                    }
                });
                height += 1;
            });

        Platform { rocks, height }
    }

    fn calc_load(&self) -> usize {
        let width = self.rocks.len();

        let mut load = 0;

        for x in 0..width {
            let mut y = 0;
            for rock in &self.rocks[x] {
                match rock {
                    Rock::Round(_) => {
                        load += self.height - y;
                        y += 1;
                    },
                    Rock::Square(orig_y) => {
                        y = orig_y + 1;
                    }
                }
            }
        }

        load
    }
}

fn part_one(file_name: &str) {
    let platform = Platform::parse(file_name);
    let load = platform.calc_load();
    println!("Part 1: {}", load);
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");

    println!("Done!");
}
