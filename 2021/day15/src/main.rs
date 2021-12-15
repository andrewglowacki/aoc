use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::min;
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
    height: usize,
    width: usize
}

struct CaveLog {
    visiting: HashSet<(usize, usize)>,
    calculated: HashMap<(usize, usize), (u32, Vec<(usize, usize)>)>,
    sum: u32
}

impl CaveLog {
    fn new() -> CaveLog {
        CaveLog {
            visiting: HashSet::new(),
            calculated: HashMap::new(),
            sum: 0
        }
    }
    fn copy(copy: &CaveLog) -> CaveLog {
        CaveLog {
            visiting: copy.visiting.clone(),
            calculated: copy.calculated.clone(),
            sum: copy.sum
        }
    }
    fn can_visit(&mut self, r: usize, c: usize) -> bool {
        self.visiting.insert((r, c))
    }
    fn visited(&mut self, r: usize, c: usize, risk: u32, to_end: Vec<(usize, usize)>, have_all: bool) {
        self.visiting.remove(&(r, c));
        if have_all {
            self.calculated.insert((r, c), (risk, to_end));
        }
    }
    fn get_risk(&self, r: usize, c: usize) -> Option<(u32, Vec<(usize, usize)>)> {
        if let Some((risk, path)) = self.calculated.get(&(r, c)) {
            Some((*risk, path.to_vec()))
        } else {
            None
        }
    }
}

impl Cave {
    fn from_file(file_name: &str) -> Cave {
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
            height,
            width
        }
    }

    fn find_cheapest_path(&self, r: usize, c: usize, log: &mut CaveLog) -> Option<(u32, Vec<(usize, usize)>)> {
        let base_risk = self.rows[r][c];
        if r == self.height - 1 && c == self.width - 1 {
            // end game
            println!("End!");
            return Some((base_risk, vec![(r, c)]));
        } else if let Some((risk_to_end, path)) = log.get_risk(r, c) {
            println!("Returning pre-computed risk-to-end of {} for {:?}", risk_to_end, (r, c));
            return Some((risk_to_end, path));
        } else if !log.can_visit(r, c) {
            return None;
        }

        println!("Finding path for {:?}", (r, c));
        
        let points = self.get_adjacent_points(r, c);

        let mut lowest_risk = u32::MAX;
        let mut lowest_risk_path = vec![];
        let mut info = vec![];
        let mut have_all = true;
        for point in points {
            let (r, c) = point;
            if let Some((final_risk, path)) = self.find_cheapest_path(r, c, log) {
                lowest_risk = min(lowest_risk, final_risk);
                lowest_risk_path = path;
                info.push(format!("  {:?} = {}", point, lowest_risk));
            } else {
                info.push(format!("  {:?} = skipped", point));
                have_all = false;
            }
        }

        if lowest_risk == u32::MAX {
            None
        } else {
            println!("For {:?}: final: {}", (r, c), lowest_risk + base_risk);
            info.into_iter().for_each(|line| println!("{}", line));

            lowest_risk_path.insert(0, (r, c));
            log.visited(r, c, lowest_risk + base_risk, lowest_risk_path.to_vec(), have_all);
            Some((lowest_risk + base_risk, lowest_risk_path))
        }
    }
    
    fn get_adjacent_points(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut points = vec![];
        if r != 0 {
            points.push((r - 1, c));
        }
        if r != self.height - 1 {
            points.push((r + 1, c));
        }
        if c != 0 {
            points.push((r, c - 1));
        }
        if c != self.width - 1 {
            points.push((r, c + 1));
        }
        points
    }

    fn get_adjacent_not_visited(&self, r: usize, c: usize, log: &CaveLog) -> Vec<(usize, usize, u32)> {
        self.get_adjacent_points(r, c).into_iter()
            .filter(|point| !log.visiting.contains(point))
            .map(|(r, c)| (r, c, self.rows[r][c]))
            .collect::<Vec<_>>()
    }

    fn add_or_append(risk: u32, r: usize, c: usize, log: CaveLog, next_point_risk: &mut BTreeMap<u32, Vec<(usize, usize, CaveLog)>>) {
        if let Some(for_risk) = next_point_risk.get_mut(&risk) {
            for_risk.push((r, c, log));
        } else {
            next_point_risk.insert(risk, vec![(r, c, log)]);
        }
    }

    fn find_cheapest_path2(&self) -> u32 {
        let mut next_point_risk = BTreeMap::<u32, Vec<(usize, usize, CaveLog)>>::new();

        for point in self.get_adjacent_points(0, 0) {
            let (r, c) = point;
            let risk = self.rows[r][c];
            Cave::add_or_append(risk, r, c, CaveLog::new(), &mut next_point_risk);
        }

        loop {
            let (min_risk, infos) = next_point_risk.iter_mut().next().unwrap();
            let min_risk = *min_risk;
            let (r, c, mut log) = infos.pop().unwrap();

            if infos.len() == 0 {
                next_point_risk.remove(&min_risk);
            }
            
            log.visiting.insert((r, c));
            
            if r == self.height - 1 && c == self.width - 1 {
                return min_risk;
            }

            for (r, c, risk) in self.get_adjacent_not_visited(r, c, &log) {
                let new_risk = min_risk + risk;
                let new_log = CaveLog::copy(&log);
                Cave::add_or_append(new_risk, r, c, new_log, &mut next_point_risk);
            }
        }
    }

}

fn part_one(file_name: &str) {
    let cave = Cave::from_file(file_name);
    // let mut log = CaveLog::new();
    // let (cheapest, path) = cave.find_cheapest_path(0, 0, &mut log).unwrap();
    // println!("Part 1: {} : {:?}", (cheapest - cave.rows[0][0]), path);
    println!("Part 1: {}", cave.find_cheapest_path2());
    // let mut calculated = log.calculated.iter()
    //     .collect::<Vec<_>>();
    // calculated.sort();
    // for (point, info) in calculated {
    //     println!("Calculated for {:?} is {:?}", point, info);
    // }
}

fn part_two(file_name: &str) {
    let _lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("input.txt");
    part_one("sample.txt");
    part_two("input.txt");

    println!("Done!");
}
