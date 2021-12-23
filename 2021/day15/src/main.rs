use std::collections::HashMap;
use std::cmp::min;
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

struct Cave {
    rows: Vec<Vec<u32>>,
    base_height: usize,
    base_width: usize,
    width: usize,
    height: usize
}

struct CaveLog {
    visiting: HashSet<(usize, usize)>,
    calculated: HashMap<(usize, usize, u8), u32>
}

impl CaveLog {
    fn new() -> CaveLog {
        CaveLog {
            visiting: HashSet::new(),
            calculated: HashMap::new()
        }
    }
    fn visited(&mut self, r: usize, c: usize, risk: u32, config: u8) {
        self.visiting.remove(&(r, c));
        self.calculated.insert((r, c, config), risk);
    }
    fn get_risk(&self, r: usize, c: usize, config: u8) -> Option<&u32> {
        self.calculated.get(&(r, c, config))
    }
}

struct Stack {
    frames: Vec<Frame>
}

impl Stack {
    fn new() -> Stack {
        Stack {
            frames: Vec::new()
        }
    }
    fn push(&mut self, frame: Frame) {
        self.frames.push(frame);
    }
    fn pop(&mut self) -> Frame {
        self.frames.pop().unwrap()
    }
    fn last(&mut self) -> &mut Frame {
        let len = self.frames.len();
        self.frames.get_mut(len - 1).unwrap()
    }
    fn has_frames(&self) -> bool {
        self.frames.len() > 0
    }
}

struct Frame {
    r: usize,
    c: usize,
    config: u8,
    points: Vec<(usize, usize)>,
    lowest_risk: u32,
}

impl Frame {
    fn new(r: usize, c: usize, config: u8, points: Vec<(usize, usize)>) -> Frame {
        Frame {
            r,
            c,
            config,
            points,
            lowest_risk: u32::MAX
        }
    }
    fn check_risk(&mut self, risk: u32) {
        self.lowest_risk = min(self.lowest_risk, risk);
    }
}

impl Cave {
    fn from_file(file_name: &str, size_mult: usize) -> Cave {
        let rows = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let height = rows.len();
        let width = rows[0].len();
        Cave {
            rows,
            base_height: height,
            base_width: width,
            width: width * size_mult,
            height: height * size_mult
        }
    }

    fn get_risk(&self, r: usize, c: usize) -> u32 {
        if r == 0 && c == 0 {
            0
        } else {
            let add = ((r / self.base_height) + (c / self.base_width)) as u32;
            let risk = self.rows[r % self.base_height][c % self.base_width];
            let risk = risk + add;
            risk % 10 + (risk / 10)
        }
    }

    fn add_if_valid(&self, r: i32, c: i32, log: &CaveLog, points: &mut Vec<(usize, usize)>) -> bool {
        if r >= 0 && r < self.height as i32 && c >= 0 && c < self.width as i32 {
            let point = (r as usize, c as usize);
            if !log.visiting.contains(&point) {
                points.push(point);
                return true;
            }
        }
        false
    }
    
    fn get_adjacent_points(&self, r: usize, c: usize, log: &CaveLog) -> (u8, Vec<(usize, usize)>) {
        let mut points = vec![];
        let mut config = 0;
        let c = c as i32;
        let r = r as i32;
        if self.add_if_valid(r - 1, c, log, &mut points) {
            config = 0x01;
        }
        if self.add_if_valid(r + 1, c, log, &mut points) {
            config |= 0x02;
        }
        if self.add_if_valid(r, c - 1, log, &mut points) {
            config |= 0x04;
        }
        if self.add_if_valid(r, c + 1, log, &mut points) {
            config |= 0x08;
        }
        (config, points)
    }

    fn find_least_risky_path(&self) -> (u32, HashSet<(usize, usize)>) {
        let mut log = CaveLog::new();
        let mut stack = Stack::new();
        let (config, points) = self.get_adjacent_points(0, 0, &log);
        stack.push(Frame::new(0, 0, config, points));
        loop {
            let mut frame = stack.pop();

            let r = frame.r;
            let c = frame.c;
            let config = frame.config;
            
            let base_risk = self.get_risk(r, c);
            
            if r == self.height - 1 && c == self.width - 1 {
                // end game
                let prev = stack.last();
                prev.check_risk(base_risk);
                continue;
            } else if let Some(risk_to_end) = log.get_risk(r, c, config) {
                if stack.has_frames() {
                    let prev = stack.last();
                    prev.check_risk(*risk_to_end);
                    continue;
                } else {
                    return (*risk_to_end, self.get_path_to_end(r, c, &mut log));
                }
            }

            log.visiting.insert((r, c));

            if frame.points.len() > 0 {
                let (r, c) = frame.points.pop().unwrap();
                let (new_config, new_points) = self.get_adjacent_points(r, c, &log);
                stack.push(frame);
                if new_points.len() > 0 {
                    stack.push(Frame::new(r, c, new_config, new_points));
                }
                continue;
            }

            if frame.lowest_risk == u32::MAX {
                log.visiting.remove(&(r, c));
            } else {
                let result = frame.lowest_risk + base_risk;
                log.visited(r, c, result, config);
                
                if stack.has_frames() {
                    let prev = stack.last();
                    prev.check_risk(result);
                } else {
                    return (result, self.get_path_to_end(r, c, &mut log));
                }
            }
        }
    }

    fn get_path_to_end(&self, r: usize, c: usize, log: &mut CaveLog) -> HashSet<(usize, usize)> {
        if r == self.height - 1 && c == self.width - 1 {
            log.visiting.clone()
        } else {
            log.visiting.insert((r, c));
            let (_, points) = self.get_adjacent_points(r, c, &log);
            if let Some((r, c, _)) = points.into_iter()
                .flat_map(|(r, c)| {
                    let (config, _) = self.get_adjacent_points(r, c, &log);
                    if let Some(risk) = log.get_risk(r, c, config) {
                        Some((r, c, *risk))
                    } else if r == self.height - 1 && c == self.width - 1 {
                        Some((r, c, self.get_risk(r, c)))
                    } else {
                        None
                    }
                })
                .min_by_key(|(_, _, risk)| *risk) 
            {
                self.get_path_to_end(r, c, log)
            }
            else
            {
                panic!("at ({}, {}) no next point with {} visited thus far", r, c, log.visiting.len());
            }
        }
    }

    fn _print_path(&self, points: HashSet<(usize, usize)>) {
        for r in 0..self.height {
            for c in 0..self.width {
                let in_path = points.contains(&(r, c));
                if in_path {
                    print!("\x1b[42m{}\x1b[0m", self.get_risk(r, c));
                } else {
                    print!("{}", self.get_risk(r, c));
                }
            }
            println!("");
        }
    }

}

fn part_one(file_name: &str) {
    let cave = Cave::from_file(file_name, 1);
    let (risk, _) = cave.find_least_risky_path();
    println!("Part 1: {}", risk);
}

fn part_two(file_name: &str) {
    let cave = Cave::from_file(file_name, 5);
    let (risk, _) = cave.find_least_risky_path();
    println!("Part 2: {}", risk);
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");
    part_two("input.txt");

    println!("Done!");
}
