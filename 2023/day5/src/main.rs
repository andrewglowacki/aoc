use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

// const SOIL: usize = 1;
// const FERTILIZER: usize = 2;
// const WATER: usize = 3;
// const LIGHT: usize = 4;
// const TEMPERATURE: usize = 5;
// const HUMIDITY: usize = 6;
const LOCATION: usize = 7;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}
struct Mapping {
    source: u64,
    dest: u64,
    length: u64
}

impl Mapping {
    fn apply(&self, input: u64) -> u64 {
        if self.source + self.length > input {
            let offset = input - self.source;
            self.dest + offset
        } else {
            input
        }
    }
}

struct Almanac {
    mappings: Vec<BTreeMap<u64, Mapping>>
}

impl Almanac {
    fn convert(&self, seed: u64, to: usize) -> u64 {
        (0..to).into_iter().fold(seed, |result, i|{
            let mapper = &self.mappings[i];
            if let Some((_, mapping)) = mapper.range(..result + 1).last() {
                mapping.apply(result)
            } else {
                result
            }
        })
    }
}

fn parse_map(lines: Vec<String>) -> BTreeMap<u64, Mapping> {
    lines.into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number| number.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|numbers| {
            let dest = numbers[0];
            let source = numbers[1];
            let length = numbers[2];
            let mapping = Mapping { source, dest, length };
            (source, mapping)
        })
        .collect::<BTreeMap<_,_>>()
}

fn parse_input(file_name: &str) -> (Almanac, Vec<u64>) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());

    let seeds = lines.next().unwrap();
    lines.next().unwrap(); // skip blank line
    
    let seeds = seeds[seeds.find(':').unwrap() + 1..].trim();
    let seeds = seeds.split(" ")
        .map(|number| number.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    
    let mappings = (0..7).map(|_| {
        parse_map((&mut lines).take_while(|line| !line.is_empty())
            .skip(1)
            .collect::<Vec<_>>())
    })
    .collect::<Vec<_>>();

    let almanac = Almanac { mappings };

    (almanac, seeds)
}


fn part_one(file_name: &str) {
    let (almanac, seeds) = parse_input(file_name);

    let minimum = seeds.into_iter()
        .map(|seed| almanac.convert(seed, LOCATION))
        .min()
        .unwrap();

    println!("Part 1: {}", minimum);
}

fn part_two(file_name: &str) {
    let (almanac, seeds) = parse_input(file_name);

    let mut minimum = u64::MAX;

    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i];
        let length = seeds[i + 1];
        for seed in start..start + length {
            let result = almanac.convert(seed, LOCATION);
            if result < minimum {
                minimum = result;
            }
        }
    }
    
    println!("Part 2: {}", minimum);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
