use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Grid {
    rows: Vec<Vec<u32>>,
    width: usize,
    height: usize
}

impl Grid {
    fn from_file(file_name: &str) -> Grid {
        let rows = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| line.chars()
                .flat_map(|c| c.to_digit(10))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        let width = rows[0].len();
        let height = rows.len();

        Grid {
            rows,
            width,
            height
        }
    }

    fn is_low_point(&self, r: usize, c: usize) -> bool {
        let check_left = c != 0;
        let check_right = c != self.width - 1;
        let check_top = r != 0;
        let check_bottom = r != self.height - 1;

        let row = &self.rows[r];
        let current = row[c];

        if check_left && row[c - 1] <= current {
            false
        } else if check_right && row[c + 1] <= current {
            false
        } else if check_top && self.rows[r - 1][c] <= current {
            false
        } else if check_bottom && self.rows[r + 1][c] <= current {
            false
        } else {
            true
        }
    }

    fn calc_basin_size(&self, r: usize, c: usize) -> u32 {
        let mut points = HashSet::new();
        self.calc_basin_size_recursive(r, c, &mut points);
        points.len() as u32
    }
    
    fn calc_basin_size_recursive(&self, r: usize, c: usize, points: &mut HashSet<(usize, usize)>) {
        let current = self.rows[r][c];
        if current != 9 && points.insert((r, c)) {
            if r != 0 && self.rows[r - 1][c] > current {
                self.calc_basin_size_recursive(r - 1, c, points);
            }
            if r != self.height - 1 && self.rows[r + 1][c] > current {
                self.calc_basin_size_recursive(r + 1, c, points);
            }
            if c != 0 && self.rows[r][c - 1] > current {
                self.calc_basin_size_recursive(r, c - 1, points);
            }
            if c != self.width - 1 && self.rows[r][c + 1] > current {
                self.calc_basin_size_recursive(r, c + 1, points);
            }
        }
    }
}

fn part_one(file_name: &str) {
    let grid = Grid::from_file(file_name);
    let mut risk = 0;
    for r in 0..grid.height {
        for c in 0..grid.width {
            if grid.is_low_point(r, c) {
                risk += 1 + grid.rows[r][c];
            }
        }
    }

    println!("Part 1: {}", risk);
}

fn part_two(file_name: &str) {
    let grid = Grid::from_file(file_name);

    let mut basins = vec![];

    for r in 0..grid.height {
        for c in 0..grid.width {
            if grid.is_low_point(r, c) {
                let basin_size = grid.calc_basin_size(r, c);
                basins.push(basin_size);
            }
        }
    }

    basins.sort();

    let solution = basins.iter()
        .skip(basins.len() - 3)
        .fold(1, |total, size| total * size);
    
    println!("Part 2: {}", solution);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");
    part_two("sample.txt");

    println!("Done!");
}
