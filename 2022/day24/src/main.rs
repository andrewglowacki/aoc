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

struct Clock {
    horizontal: u128,
    vertical: u32,
    minutes: u32,
    width: u32,
    height: u32,
    state: usize,
    states: usize
}

impl Clock {
    fn new(width: usize, height: usize) -> Clock {
        Clock {
            horizontal: 1,
            vertical: 1,
            minutes: 0,
            width: width as u32,
            height: height as u32,
            state: 0,
            states: width * height
        }
    }
    fn tick(&mut self) {
        self.minutes += 1;
        self.state = (self.state + 1) % self.states;
        if self.minutes % self.width == 0 {
            self.horizontal = 1;
        } else {
            self.horizontal = self.horizontal << 1;
        }
        if self.minutes % self.height == 0 {
            self.vertical = 1;
        } else {
            self.vertical = self.vertical << 1;
        }
    } 
}

struct Horizontal {
    grid: Vec<Vec<u128>>,
    width: usize,
}

impl Horizontal {
    fn new(grid: Vec<Vec<u128>>, width: usize) -> Horizontal {
        Horizontal { grid, width }
    }
    fn fill_left(&mut self, x: usize, y: usize) {
        let row = &mut self.grid[y];
        let mut indicator = 1 << (self.width - 1);

        row[x] = row[x] | 1;
        for x in x + 1..x + self.width {
            let x = x % self.width;
            row[x] = row[x] | indicator;
            indicator = indicator >> 1;
        }
    }
    fn fill_right(&mut self, x: usize, y: usize) {
        let row = &mut self.grid[y];
        let mut indicator = 1;

        for x in x..x + self.width {
            let x = x % self.width;
            row[x] = row[x] | indicator;
            indicator = indicator << 1;
        }
    }
    fn is_oepn(&self, x: usize, y: usize, clock: &Clock) -> bool {
        let point = self.grid[y][x];
        point & clock.horizontal == 0
    }
}

struct Vertical {
    grid: Vec<Vec<u32>>,
    height: usize,
}

impl Vertical {
    fn new(grid: Vec<Vec<u32>>, height: usize) -> Vertical {
        Vertical { grid, height }
    }
    fn fill_up(&mut self, x: usize, y: usize) {
        let mut indicator = 1 << (self.height - 1);

        self.grid[y][x] = self.grid[y][x] | 1;

        for y in y + 1..y + self.height {
            let y = y % self.height;
            self.grid[y][x] = self.grid[y][x] | indicator;
            indicator = indicator >> 1;
        }
    }
    fn fill_down(&mut self, x: usize, y: usize) {
        let mut indicator = 1;

        for y in y..y + self.height {
            let y = y % self.height;
            self.grid[y][x] = self.grid[y][x] | indicator;
            indicator = indicator << 1;
        }
    }
    fn is_oepn(&self, x: usize, y: usize, clock: &Clock) -> bool {
        let point = self.grid[y][x];
        point & clock.vertical == 0
    }
}

struct Blizzards {
    horizontal: Horizontal,
    vertical: Vertical,
    width: usize,
    height: usize,
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32
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

        let horizontal = vec![vec![0; width]; height];
        let mut horizontal = Horizontal::new(horizontal, width);
        let vertical = vec![vec![0; width]; height];
        let mut vertical = Vertical::new(vertical, height);

        let grid = grid;

        for y in 0..height {
            let row = &grid[y];
            for x in 0..width {
                let c = row[x];
                match c {
                    '^' => vertical.fill_up(x, y),
                    'v' => vertical.fill_down(x, y),
                    '<' => horizontal.fill_left(x, y),
                    '>' => horizontal.fill_right(x, y),
                    '.' => (),
                    _ => panic!("Invalid character: {} at {}, {}", c, x, y)
                }
            }

        }

        Blizzards {
            horizontal,
            vertical,
            width,
            height,
            start_x: 0,
            start_y: -1,
            end_x: width as i32 - 1,
            end_y: height as i32
        }
    }
    
    fn is_open(&self, x: i32, y: i32, clock: &Clock) -> bool {
        if x == self.end_x && y == self.end_y {
            true
        } else if x == self.start_x && y == self.start_y {
            true
        } else if x < 0 || y < 0 {
            false
        } else {
            let x = x as usize;
            let y = y as usize;
            if x >= self.width || y >= self.height {
                false
            } else {
                self.vertical.is_oepn(x, y, clock) && 
                self.horizontal.is_oepn(x, y, clock)
            }
        }
    }

    fn visit_if_open(
        &self, 
        clock: &Clock, 
        x: i32, 
        y: i32, 
        visited: &mut HashSet<(i32, i32, usize)>,
        explore_next: &mut HashSet<(i32, i32)>) 
    {
        if self.is_open(x, y, clock) && visited.insert((x, y, clock.state)) {
            explore_next.insert((x, y));
        }
    }

    fn find_shorted_path(&self, clock: &mut Clock) -> u32 {
        let mut visited = HashSet::new();

        let end = (self.end_x as i32, self.end_y as i32);

        let mut explore = HashSet::new();
        explore.insert((self.start_x, self.start_y));

        let mut found = false;

        while explore.len() > 0 {
            clock.tick();

            let mut explore_next = HashSet::new();

            for (x, y) in explore {
                self.visit_if_open(&clock, x, y, &mut visited, &mut explore_next);
                self.visit_if_open(&clock, x - 1, y, &mut visited, &mut explore_next);
                self.visit_if_open(&clock, x + 1, y, &mut visited, &mut explore_next);
                self.visit_if_open(&clock, x, y - 1, &mut visited, &mut explore_next);
                self.visit_if_open(&clock, x, y + 1, &mut visited, &mut explore_next);
            }

            if explore_next.contains(&end) {
                found = true;
                break;
            }

            explore = explore_next;
        }

        if !found {
            panic!("Terminated without finding the end");
        }

        clock.minutes
    }

}

fn part_one(file_name: &str) {
    let blizzards = Blizzards::from_file(file_name);
    let mut clock = Clock::new(blizzards.width, blizzards.height);

    let time = blizzards.find_shorted_path(&mut clock);
    
    println!("Part 1: {}", time);
}

fn part_two(file_name: &str) {
    let mut blizzards = Blizzards::from_file(file_name);
    let mut clock = Clock::new(blizzards.width, blizzards.height);

    blizzards.find_shorted_path(&mut clock);

    blizzards.start_x = blizzards.end_x;
    blizzards.start_y = blizzards.end_y;
    blizzards.end_x = 0;
    blizzards.end_y = -1;

    blizzards.find_shorted_path(&mut clock);

    blizzards.end_x = blizzards.start_x;
    blizzards.end_y = blizzards.start_y;
    blizzards.start_x = 0;
    blizzards.start_y = -1;

    let time_total = blizzards.find_shorted_path(&mut clock);
    
    println!("Part 2: {}", time_total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
