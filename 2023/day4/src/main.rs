use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Scratchcard {
    id: usize,
    winning: HashSet<u32>,
    yours: HashSet<u32>
}

impl Scratchcard {
    fn parse(line: String) -> Scratchcard {

        let parts = line.split(":")
            .map(|line| line.trim())
            .collect::<Vec<_>>();

        let id = parts[0].split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut number_parts = parts[1].split('|')
            .map(|part| part.trim())
            .map(|part| {
                part.split_ascii_whitespace()
                    .map(|number| number.parse::<u32>().unwrap())
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        let yours = number_parts.pop().unwrap();
        let winning = number_parts.pop().unwrap();

        Scratchcard { id, winning, yours }
    }

    fn get_your_winning_numbers(&self) -> HashSet<u32> {
        self.yours.iter()
            .filter(|number| self.winning.contains(number))
            .copied()
            .collect()
    }

    fn get_points(&self) -> u32 {
        let winning = self.get_your_winning_numbers();
        let count = winning.len() as u32;
        if count == 0 {
            return 0;
        }
        let base: u32 = 2;
        base.pow(count - 1)
    }

    fn add_card_counts(&self, card_count: &mut HashMap<usize, u32>) {
        let wins = self.get_your_winning_numbers().len();

        let my_count = *card_count.get(&self.id).unwrap();

        for i in 1..wins + 1 {
            let next_count = card_count.get_mut(&(self.id + i)).unwrap();
            *next_count += my_count;
        }
    }
}

fn part_one(file_name: &str) {
    let total = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Scratchcard::parse(line))
        .map(|scratchcard| scratchcard.get_points())
        .sum::<u32>();
    
    println!("Part 1: {}", total);
}

fn part_two(file_name: &str) {
    let mut card_count = HashMap::<usize, u32>::new();

    let scratchcards = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Scratchcard::parse(line))
        .collect::<Vec<_>>();

    for i in 0..scratchcards.len() {
        card_count.insert(i + 1, 1);
    }

    scratchcards.iter().for_each(|scratchcard| {
        scratchcard.add_card_counts(&mut card_count)
    });

    let total = card_count.values().sum::<u32>();
    
    println!("Part 2: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
