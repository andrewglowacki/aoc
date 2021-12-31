use std::fmt::Formatter;
use std::fmt::Display;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

const HEIGHT: usize = 137;
const WIDTH: usize = 139;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Copy, Clone)]
struct Line {
    low: u128,
    high: u16
}

impl Line {
    fn new() -> Line {
        Line {
            low: 0,
            high: 0
        }
    }
    fn set(&mut self, r: usize) {
        match r >= 128 {
            true => self.high = self.high | (1 << (15 - ((r - 128) as u16))),
            false => self.low = self.low | (1 << (127 - r))
        };
    }

    fn is_set(&self, r: usize) -> bool {
        match r >= 128 {
            true => (self.high >> (15 - ((r - 128) as u16))) & 1 == 1,
            false => (self.low >> (127 - r)) & 1 == 1
        }
    }

    fn move_east(&self, cur_self: &mut Line, cur_to: &mut Line, old_to: &Line, intersecting: &Line, changed: &mut Line) {
        let moves_low = (self.low ^ (old_to.low | intersecting.low)) & self.low;
        let non_moves_low = !moves_low;
        cur_to.low = cur_to.low | moves_low;
        cur_self.low = cur_self.low & non_moves_low;

        let moves_high = (self.high ^ (old_to.high | intersecting.high)) & self.high;
        let non_moves_high = !moves_high;
        cur_to.high = cur_to.high | moves_high;
        cur_self.high = cur_self.high & non_moves_high;

        changed.low = changed.low | moves_low;
        changed.high = changed.high | moves_high;
    }

    fn move_south(&mut self, intersecting: &Line, changed: &mut Line) {
        // include the first bit at the end to make the calculation easier
        let orig_first_bit = self.low >> 127;
        let first_bit = (orig_first_bit as u16) << (15 - (HEIGHT - 128));
        let first_bit = first_bit | ((intersecting.low >> 127) as u16) << (15 - (HEIGHT - 128));
        self.high = self.high | first_bit; 
        
        // do high first since it has extra bit space
        let new_positions_high = self.high >> 1;
        let moves_high = (new_positions_high ^ (self.high | intersecting.high)) & new_positions_high;
        let non_moves_high = !(moves_high << 1);
        
        let first_high_bit = self.high >> 15;
        let intersecting_high_bit = intersecting.high >> 15;
        let last_low = (self.low & 1) as u16;
        let new_first_high = (last_low ^ (first_high_bit | intersecting_high_bit)) & last_low;
        self.high = (self.high & non_moves_high) | moves_high | (new_first_high << 15);
        let new_last_low = !(new_first_high as u128);

        let first_bit = ((self.high >> (15 - (HEIGHT - 128))) & 1) as u128;
        self.high = self.high & 0xFF80; // remove extra bits

        let new_positions = self.low >> 1;
        let moves_low = (new_positions ^ (self.low | intersecting.low)) & new_positions;
        let non_moves_low = !(moves_low << 1);
        self.low = ((self.low & non_moves_low) | (first_bit << 127) | moves_low) & new_last_low;

        changed.low = changed.low | moves_low | ((self.low >> 127) ^ orig_first_bit);
        changed.high = changed.high | (moves_high & 0xFF80);
    }

    fn assign(&mut self, from: &Line) {
        self.high = from.high;
        self.low = from.low;
    }

}

impl Display for Line {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:0128b} {:09b}|{:02b}|{:05b}", self.low, (self.high & 0xFF80) >> 7, (self.high & 0x60) >> 5, self.high & 0x1F)
        // write!(fmt, "{:0128b}{:016b}", self.low, self.high)
    }
}

struct Trench {
    east: Vec<Line>,
    south: Vec<Line>
}

impl Trench {
    fn from_file(file_name: &str) -> Trench {
        let grid = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        let height = grid.len();
        let width = grid[0].len();

        assert_eq!(WIDTH, width);
        assert_eq!(HEIGHT, height);

        let mut east = (0..width)
            .map(|_| Line::new())
            .collect::<Vec<_>>();
        
        let mut south = east.to_vec();

        for r in 0..height {
            let row = &grid[r];

            (0..width).into_iter().for_each(|c| {
                match row[c] {
                    '>' => east[c].set(r),
                    'v' => south[c].set(r),
                    '.' => (),
                    _ => panic!("Invalid character at {}, {}: {}", r, c, row[c])
                }
            });
        }

        Trench {
            east: east,
            south: south
        }
    }

    fn execute_step(&mut self) -> bool {
        let first_east = self.east[0];

        let mut changed_east_count = 0;
        let mut changed_south_count = 0;
        let mut changed_east = Line::new();
        let mut changed_south = Line::new();

        // move east cucumbers first
        let mut prev = self.east[0];
        let mut temp = Line::new();
        for i in 0..WIDTH {
            temp.assign(&self.east[i]);
            let move_to = &mut self.east[(i + 1) % WIDTH];
            let intersecting = &self.south[(i + 1) % WIDTH];
            let new_prev = *move_to;
            let prev_changed = changed_east;
            prev.move_east(&mut temp, move_to, &new_prev, intersecting, &mut changed_east);
            if prev_changed.low != changed_east.low || prev_changed.high != changed_east.high {
                changed_east_count += 1;
            }
            self.east[i].assign(&temp);
            prev = new_prev;
        }
        temp.assign(&self.east[WIDTH - 1]);
        prev.move_east(&mut temp, &mut self.east[0], &first_east, &self.south[0], &mut changed_east);
        self.east[WIDTH - 1].assign(&temp);

        // now move south cucumbers
        for i in 0..WIDTH {
            let move_line = &mut self.south[i];
            let prev_changed = changed_south;
            move_line.move_south(&self.east[i], &mut changed_south);
            if prev_changed.low != changed_south.low || prev_changed.high != changed_south.high {
                changed_south_count += 1;
            }
        }

        println!("Changed east: {}", changed_east_count);
        println!("{}", changed_east);
        println!("Changed south: {}", changed_south_count);
        println!("{}", changed_south);
        changed_east_count > 0 || changed_south_count > 0
    }

    fn print(&self) {
        print!("    ");
        for i in 0..WIDTH {
            if i % 10 == 0 {
                print!("|");
            } else {
                print!("{}", i % 10);
            }
        }
        println!("");
        for r in 0..4 {
            print!("{:03} ", r);
            for c in 0..WIDTH {
                match (self.east[c].is_set(r), self.south[c].is_set(r)) {
                    (false, false) => print!("."),
                    (false, true) => print!("v"),
                    (true, false) => print!(">"),
                    (true, true) => panic!("Both east and south are set at {}, {}", r, c)
                }
            }
            println!(" {:03}", r);
        }
        println!("-    ------------------------");
        for r in HEIGHT - 11..HEIGHT {
            print!("{:03} ", r);
            for c in 0..WIDTH {
                match (self.east[c].is_set(r), self.south[c].is_set(r)) {
                    (false, false) => print!("."),
                    (false, true) => print!("v"),
                    (true, false) => print!(">"),
                    (true, true) => panic!("Both east and south are set at {}, {}", r, c)
                }
            }
            println!(" {:03}", r);
        }
        print!("    ");
        for i in 0..WIDTH {
            if i % 10 == 0 {
                print!("|");
            } else {
                print!("{}", i % 10);
            }
        }
        println!("");
        println!("========================================");
    }

}

fn part_one(file_name: &str) {
    let mut trench = Trench::from_file(file_name);
    // trench.print();
    // println!();

    let mut steps = 1;
    while trench.execute_step() && steps < 1000 {
        // trench.print();
        println!("step {}   ", steps);
        steps += 1;
        if steps < 5 || (steps > 50 && steps < 56) {
            trench.print();
            println!();
        }
    }
    trench.print();
    println!();
    
    println!("Part 1: {}", steps);
}

fn part_two(file_name: &str) {
    let trench = Trench::from_file(file_name);
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("input.txt");
    // part_two("input.txt");

    println!("Done!");
}
