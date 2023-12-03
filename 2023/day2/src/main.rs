use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

#[derive(Debug)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize
}

#[derive(Debug)]
struct Game {
    id: usize,
    max: Draw
}

impl Draw {
    fn parse(draw_str: &str) -> Draw {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        draw_str.split(",")
            .map(|ball| ball.trim().split(" ").collect::<Vec<_>>())
            .for_each(|ball| {
                let amount = ball[0].parse::<usize>().unwrap();
                match ball[1] {
                    "red" => red = amount,
                    "green" => green = amount,
                    "blue" => blue = amount,
                    _ => panic!("Invalid ball type: {}", ball[1])
                }
            });
        Draw { red, green, blue }
    }
    fn max(&mut self, other: &Draw) {
        self.red = self.red.max(other.red);
        self.green = self.green.max(other.green);
        self.blue = self.blue.max(other.blue);
    }
    fn zero() -> Draw {
        Draw {
            red: 0,
            green: 0, 
            blue: 0
        }
    }
}

impl Game {
    fn parse(line: String) -> Game {
        let id_end = line.find(":").unwrap();
        let id_start = line.find(" ").unwrap() + 1;
        let id = line[id_start..id_end].parse::<usize>().unwrap();
        let draws = line[id_end + 1..].split(";")
            .map(|draw_str| Draw::parse(draw_str.trim()))
            .collect::<Vec<_>>();
        let mut max = Draw::zero();
        draws.iter().for_each(|draw| max.max(draw));
        Game { id, max }
    }
    fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.max.red <= red &&
        self.max.green <= green &&
        self.max.blue <= blue
    }
    fn get_power(&self) -> u32 {
        (self.max.red * self.max.green * self.max.blue) as u32
    }
}

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn part_one(file_name: &str) {
    let result = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Game::parse(line))
        .filter(|game| game.is_possible(12, 13, 14))
        .map(|game| game.id)
        .sum::<usize>();
    
    println!("Part 1: {}", result);
}

fn part_two(file_name: &str) {
    let result = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Game::parse(line))
        .map(|game| game.get_power())
        .sum::<u32>();
    
    println!("Part 2: {}", result);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
