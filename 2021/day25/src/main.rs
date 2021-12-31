use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Cucumber {
    NONE,
    EAST,
    SOUTH
}

use Cucumber::*;

struct Trench {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cucumber>>,
}

impl Trench {
    fn from_file(file_name: &str) -> Trench {
        let grid = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| line.chars()
                .map(|c| match c {
                    '>' => EAST,
                    'v' => SOUTH,
                    '.' => NONE,
                    _ => panic!("Invalid character {}", c)
                })
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        let height = grid.len();
        let width = grid[0].len();

        println!("Grid is {} x {}", height, width);

        Trench {
            height,
            width,
            grid
        }
    }

    fn execute_step(&mut self) -> bool {
        let mut changed = false;

        let mut new_grid = self.grid.to_vec();

        // move east
        for r in 0..self.height {
            let row = &self.grid[r];
            let mut c = 0;
            while c < self.width {
                let cur = row[c];
                let next = row[(c + 1) % self.width];
                if cur == EAST && next == NONE {
                    new_grid[r][c] = NONE;
                    new_grid[r][(c + 1) % self.width] = EAST;
                    c += 1;
                    changed = true;
                } else {
                    new_grid[r][c] = cur;
                }
                c += 1;
            }
        }

        self.grid = new_grid;
        new_grid = self.grid.to_vec();

        // move south
        for c in 0..self.width {
            let mut r = 0;
            while r < self.height {
                let cur = self.grid[r][c];
                let next = self.grid[(r + 1) % self.height][c];
                if cur == SOUTH && next == NONE {
                    new_grid[r][c] = NONE;
                    new_grid[(r + 1) % self.height][c] = SOUTH;
                    changed = true;
                    r += 1;
                } else {
                    new_grid[r][c] = cur;
                }
                r += 1;
            }
        }

        self.grid = new_grid;
        changed
    }

    fn _print(&self) {
        for row in self.grid.iter() {
            for c in row {
                print!("{}", match c {
                    SOUTH => 'v',
                    EAST => '>',
                    NONE => '.'
                });
            }
            println!("");
        }
        println!("");
    }

}

fn part_one(file_name: &str) {
    let mut trench = Trench::from_file(file_name);

    let mut steps = 1;
    while trench.execute_step() {
        steps += 1;
    }
    
    println!("Part 1: {}", steps);
}

fn main() {
    part_one("input.txt");

    println!("Done!");
}
