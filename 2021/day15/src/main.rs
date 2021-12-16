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

    fn find_least_risky_path(&self, r: usize, c: usize, log: &mut CaveLog) -> Option<u32> {
        let base_risk = self.get_risk(r, c);

        let (config, points) = self.get_adjacent_points(r, c, &log);
        
        if r == self.height - 1 && c == self.width - 1 {
            // end game
            return Some(base_risk);
        } else if let Some(risk_to_end) = log.get_risk(r, c, config) {
            return Some(*risk_to_end);
        } else if points.len() == 0 {
            return None
        }

        log.visiting.insert((r, c));

        let mut lowest_risk = u32::MAX;
        for point in points {
            let (r, c) = point;
            if let Some(final_risk) = self.find_least_risky_path(r, c, log) {
                lowest_risk = min(lowest_risk, final_risk);
            }
        }

        if lowest_risk == u32::MAX {
            log.visiting.remove(&(r, c));
            None
        } else {
            log.visited(r, c, lowest_risk + base_risk, config);
            Some(lowest_risk + base_risk)
        }
    }

}

fn part_one(file_name: &str) {
    let cave = Cave::from_file(file_name, 1);
    let mut log = CaveLog::new();
    println!("Part 1: {}", cave.find_least_risky_path(0, 0, &mut log).unwrap());
}

fn part_two(file_name: &str) {
    let cave = Cave::from_file(file_name, 5);
    let mut log = CaveLog::new();
    println!("Part 2: {}", cave.find_least_risky_path(0, 0, &mut log).unwrap());
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");
    part_two("input.txt");

    println!("Done!");
}
