use std::collections::{HashSet, BTreeSet};
use std::fs::File;
use std::mem::swap;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Garden {
    rocks: HashSet<(i32, i32)>,
    start: (i32, i32),
    height: i32,
    width: i32
}

impl Garden {
    fn parse(file_name: &str) -> Garden {
        let mut y = 0;
        let mut start = (0, 0);
        let mut width = 0;
        let rocks = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .flat_map(|line| {
                let mut x = 0;
                let cur_y = y;
                y += 1;
                width = line.len() as i32;
                line.chars()
                    .into_iter()
                    .flat_map(|c| {
                        let cur_x = x;
                        x += 1;
                        match c {
                            '.' => None,
                            '#' => Some((cur_x, cur_y)),
                            'S' => {
                                start = (cur_x, cur_y);
                                None
                            },
                            _ => panic!("Invalid character: {}", c)
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>();

        Garden { rocks, start, width, height: y as i32 }
    }

    fn count_reachable_plots(&self, steps: usize) -> usize {
        let mut current = HashSet::new();
        let mut next = HashSet::new();

        current.insert(self.start);

        for _ in 0..steps {
            for (x, y) in &current {
                let consider = vec![
                    (x - 1, *y),
                    (x + 1, *y),
                    (*x, y - 1),
                    (*x, y + 1)
                ];

                consider.into_iter()
                    .filter(|(x, y)| {
                        *x >= 0 && *x < self.width && *y >= 0 && *y < self.height
                    })
                    .filter(|point| !self.rocks.contains(point))
                    .for_each(|point| { next.insert(point); });
            }
            swap(&mut current, &mut next);
            next.clear();
        }

        current.len()
    }
}

fn part_one(file_name: &str) {
    let garden = Garden::parse(file_name);
    let reachable = garden.count_reachable_plots(64);
    println!("Part 1: {}", reachable);
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");

    println!("Done!");
}
