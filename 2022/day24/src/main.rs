use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Vertical {
    list: Vec<u32>,
    max: u32,
    last: u32,
    max_inv: u32
}

impl Vertical {
    fn new(list: Vec<u32>, max: usize) -> Vertical {
        let max = 1 << max;
        let max_inv = !max;
        let last = max >> 1;
        Vertical { list, max, max_inv, last }
    }
    fn shift_up(&mut self) {
        let max = self.max;
        let max_inv = self.max_inv;

        self.list.iter_mut().for_each(|row| {
            *row = *row << 1;
            if *row & max > 0 {
                *row = (*row & max_inv) | 1;
            }
        });
    }
    fn shift_down(&mut self) {
        let last = self.last;

        self.list.iter_mut().for_each(|row| {
            let lead = *row & 1 > 0;
            *row = *row >> 1;
            if lead {
                *row = *row | last;
            }
        });
    }
}

struct Horizontal {
    list: Vec<u128>,
    max: u128,
    last: u128,
    max_inv: u128
}

impl Horizontal {
    fn new(list: Vec<u128>, max: usize) -> Horizontal {
        let max = 1 << max;
        let max_inv = !max;
        let last = max >> 1;
        Horizontal { list, max, max_inv, last }
    }
    fn shift_left(&mut self) {
        let max = self.max;
        let max_inv = self.max_inv;

        self.list.iter_mut().for_each(|row| {
            *row = *row << 1;
            if *row & max > 0 {
                *row = (*row & max_inv) | 1;
            }
        });
    }
    fn shift_right(&mut self) {
        let last = self.last;

        self.list.iter_mut().for_each(|row| {
            let lead = *row & 1 > 0;
            *row = *row >> 1;
            if lead {
                *row = *row | last;
            }
        });
    }
}

struct Blizzards {
    up: Vertical,
    down: Vertical,
    left: Horizontal,
    right: Horizontal,
    location: (usize, usize),
    width: usize,
    height: usize,
    at_start: bool
}

impl Blizzards {
    fn from_file(file_name: &str) -> Blizzards {
        let mut grid = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .skip(1)
            .map(|line| {
                let mut row = line.chars()
                    .skip(1)
                    .collect::<Vec<_>>();
                row.pop().unwrap();
                row
            })
            .collect::<Vec<_>>();
        grid.pop().unwrap();

        let width = grid[0].len();
        let height = grid.len();

        let mut up = vec![0; height];
        let mut down = vec![0; height];
        let mut left = Vec::new();
        let mut right = Vec::new();

        let grid = grid;

        for y in 0..height {
            let row = &grid[y];
            let mut this_left: u128 = 0;
            let mut this_right: u128 = 0;

            for x in 0..width {
                let c = row[x];
                match c {
                    '^' => up[x] |= 1,
                    'v' => down[x] |= 1,
                    '<' => this_left |= 1,
                    '>' => this_right |= 1,
                    '.' => (),
                    _ => panic!("Invalid character: {} at {}, {}", c, x, y)
                }
                this_left = this_left << 1;
                this_right = this_right << 1;

                up[x] = up[x] << 1;
                down[x] = down[x] << 1;
            }

            left.push(this_left);
            right.push(this_right);
        }

        let up = Vertical::new(up, height);
        let down = Vertical::new(down, height);
        let left = Horizontal::new(left, width);
        let right = Horizontal::new(right, width);

        Blizzards {
            up,
            down, 
            left,
            right,
            width,
            height,
            location: (0, 0),
            at_start: true,
        }
    }

    fn move_blizzard(&mut self) {
        self.up.shift_up();
        self.down.shift_down();
        self.left.shift_left();
        self.right.shift_right();
    } 
}

fn part_one(file_name: &str) {
    let mut blizzards = Blizzards::from_file(file_name);
    
    println!("Part 1: {}", "incomplete");
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
