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
                // let width = line.len();
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

    fn _print(&self) {
        let width = self.columns.len();
        for y in 0..self.rows.len() {
            let row = self.rows[y];
            let mut row_str = String::with_capacity(width);
            for x in 0..width {
                let c = match ((row >> x) & 1) > 0 {
                    true => '#',
                    false => '.'
                };
                row_str.insert(0, c);
            }
            println!("{}", row_str);
        }
        println!("");
    }

    fn find_mirror_flip(&mut self, x: usize, y: usize, exclude_row: usize, exclude_col: usize) -> usize {
        //  ><
        // xxxxxx 0
        // xxxxxx 1
        // 012345 2 
        // xxxxxx 3
        // xxxxxx 4
        //
        // row_len = 6
        // col_len = 5
        // x = 1
        // y = 2
        // row_mask = 1 << 4 = 010000
        // col_mask = 1 << 2 = 000100
        //
        let row_mask = 1 << ((self.columns.len() - x) - 1);
        let col_mask = 1 << ((self.rows.len() - y) - 1);

        let orig_row = self.rows[y];
        let orig_col = self.columns[x];

        self.rows[y] = orig_row ^ row_mask;
        self.columns[x] = orig_col ^ col_mask;

        // println!("After flip at ({}, {})", x, y);
        // self._print();

        let summary = self.summarize(exclude_row, exclude_col);
        if summary > 0 {
            return summary;
        }

        self.rows[y] = orig_row;
        self.columns[x] = orig_col;

        return 0;
    }

    fn find_mirror_flip_summary(&mut self) -> usize {
        let exclude_col = Map::find_mirror(&self.columns, usize::MAX)
            .unwrap_or(usize::MAX);
        let exclude_row = Map::find_mirror(&self.rows, usize::MAX)
            .unwrap_or(usize::MAX);

        let orig_summary = self.summarize(usize::MAX, usize::MAX);

        for y in 0..self.rows.len() {
            for x in 0..self.columns.len() {
                let summary = self.find_mirror_flip(x, y, exclude_row, exclude_col);
                if summary > 0 {
                    return summary;
                }
            }
        }
        panic!("Alternate not found for original: {}", orig_summary);
    }

    fn summarize(&self, exclude_row: usize, exclude_col: usize) -> usize {
        let column_sum = Map::find_mirror(&self.columns, exclude_col)
            .map(|index| index + 1)
            .unwrap_or(0);
        let row_sum = Map::find_mirror(&self.rows, exclude_row)
            .map(|index| (index + 1) * 100)
            .unwrap_or(0);
        column_sum + row_sum
    }

    fn find_mirror(within: &Vec<u32>, exclude: usize) -> Option<usize> {
        let mid = within.len() / 2;

        let found = (0..mid).rev()
            .filter(|i| within[*i] == within[i + 1])
            .filter(|i| *i != exclude)
            .find(|i| {
                let i = *i;
                let limit = (i + 1).min((within.len() - i) - 1);
                (1..limit).all(|c| within[i - c] == within[i + c + 1])
            });
        
        if found.is_some() {
            return found;
        }

        (mid..within.len() - 1)
            .filter(|i| within[*i] == within[i + 1])
            .filter(|i| *i != exclude)
            .find(|i| {
                let i = *i;
                let limit = (i + 1).min((within.len() - i) - 1);
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
    let total = maps.iter()
        .map(|map| map.summarize(usize::MAX, usize::MAX))
        .sum::<usize>();
    println!("Part 1: {}", total);
}

fn part_two(file_name: &str) {
    let mut maps = parse_maps(file_name);
    let total = maps.iter_mut()
        .map(|map| map.find_mirror_flip_summary())
        .sum::<usize>();
    println!("Part 2: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
