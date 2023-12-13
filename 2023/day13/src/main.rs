use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Map {
    rows: Vec<u32>,
    columns: Vec<u32>,
}

impl Map {
    fn parse(lines: Vec<String>) -> Map {
        let mut columns = Vec::<Vec<bool>>::new();

        for _ in 0..lines[0].len() {
            columns.push(Vec::new());
        }

        let rows = lines.iter()
            .map(|line| {
                line.char_indices()
                    .map(|(i, c)| {
                        columns[i].push(c == '#');
                        let number = match c {
                            '#' => 1 << i,
                            '.' => 0,
                            _ => panic!("Unexpected character: {}", c)
                        };
                        number as u32
                    })
                    .reduce(|acc, item| acc | item)
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let columns = columns.into_iter()
            .map(|column| {
                let mut i = 0;
                column.into_iter()
                    .map(|present| {
                        let result = match present {
                            true => 1 << i,
                            false => 0
                        };
                        i += 1;
                        result
                    })
                    .reduce(|acc, item| acc | item)
                    .unwrap()
            })
            .collect::<Vec<_>>();

        Map { rows, columns }
    }

    fn summarize(&self) -> u32 {
        let column_sum = Map::find_mirror(&self.columns)
            .map(|index| index as u32 + 1)
            .unwrap_or(0);
        // println!("column_sum is {}", column_sum);
        let row_sum = Map::find_mirror(&self.rows)
            .map(|index| (index as u32 + 1) * 100)
            .unwrap_or(0);
        // println!("row_sum is {}", row_sum);
        column_sum + row_sum
    }

    fn find_mirror(within: &Vec<u32>) -> Option<usize> {
        let mid = within.len() / 2;

        // println!("mid={} ", mid);
        // println!("will test i={:?}", (1..mid).collect::<Vec<_>>());
        let found = (0..mid).rev()
            .filter(|i| within[*i] == within[i + 1])
            .find(|i| {
                let i = *i;
                let limit = (i + 1).min((within.len() - i) - 1);
                // println!("checking i={}, limit={}", i, limit);
                // (1..limit).for_each(|c| println!("compare {} vs {}", within[i - c], within[i + c + 1]));
                (1..limit).all(|c| within[i - c] == within[i + c + 1])
            });
        
        if found.is_some() {
            return found;
        }
        // println!("not found yet");
        // println!("will test i={:?}", (mid..within.len() - 1).collect::<Vec<_>>());

        (mid..within.len() - 1)
            .filter(|i| within[*i] == within[i + 1])
            .find(|i| {
                let i = *i;
                let limit = (i + 1).min((within.len() - i) - 1);
                // println!("checking i={}, limit={}", i, limit);
                // (1..limit).for_each(|c| println!("compare {} vs {}", within[i - c], within[i + c + 1]));
                (1..limit).all(|c| within[i - c] == within[i + c + 1])
            })
    }
}

fn parse_maps(file_name: &str) -> Vec<Map> {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .peekable();

    let mut maps = Vec::new();

    while lines.peek().is_some() {
        let map_lines = (&mut lines)
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>();
        maps.push(Map::parse(map_lines));
    }

    maps
}

fn part_one(file_name: &str) {
    let maps = parse_maps(file_name);
    // let mut i = 0;
    let total = maps.iter()
        // .inspect(|map| {
        //     println!("");
        //     println!("Map {}", i + 1);
        //     i += 1;
        //     map.lines.iter().for_each(|line| println!("{}", line));
        // })
        .map(|map| map.summarize())
        .sum::<u32>();
    println!("Part 1: {}", total);
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
