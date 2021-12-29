use std::collections::BTreeSet;
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
    visitable: BTreeSet<(u32, usize, usize)>
}

impl CaveLog {
    fn new() -> CaveLog {
        CaveLog {
            visiting: HashSet::new(),
            visitable: BTreeSet::new()
        }
    }
    fn visit_least(&mut self) -> (u32, usize, usize) {
        let first = *self.visitable.iter().next().unwrap();
        self.visitable.remove(&first);
        let (_, r, c) = first;
        self.visiting.insert((r, c));
        first
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

    fn add_if_valid(&self, r: i32, c: i32, current_risk: u32, log: &mut CaveLog) {
        if r >= 0 && r < self.height as i32 && c >= 0 && c < self.width as i32 {
            let point = (r as usize, c as usize);
            if !log.visiting.contains(&point) {
                let risk = self.get_risk(point.0, point.1) + current_risk;
                log.visitable.insert((risk, point.0, point.1));
            }
        }
    }
    
    fn add_adjacent_points(&self, r: usize, c: usize, current_risk: u32, log: &mut CaveLog) {
        let c = c as i32;
        let r = r as i32;
        self.add_if_valid(r - 1, c, current_risk, log);
        self.add_if_valid(r + 1, c, current_risk, log);
        self.add_if_valid(r, c - 1, current_risk, log);
        self.add_if_valid(r, c + 1, current_risk, log);
    }

    fn find_least_risky_path(&self) -> u32 {
        let mut log = CaveLog::new();

        log.visiting.insert((0, 0));
        self.add_adjacent_points(0, 0, 0, &mut log);

        loop {
            let (risk, r, c) = log.visit_least();
            if r == self.width - 1 && c == self.height - 1 {
                return risk;
            }
            self.add_adjacent_points(r, c, risk, &mut log);
        }

    }

}

fn part_one(file_name: &str) {
    let cave = Cave::from_file(file_name, 1);
    let risk = cave.find_least_risky_path();
    println!("Part 1: {}", risk);
}

fn part_two(file_name: &str) {
    let cave = Cave::from_file(file_name, 5);
    let risk = cave.find_least_risky_path();
    println!("Part 2: {}", risk);
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");
    part_two("input.txt");

    println!("Done!");
}
