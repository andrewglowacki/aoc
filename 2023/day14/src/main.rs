use std::collections::{BTreeSet, BTreeMap, HashMap};
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

struct RockSet {
    x_to_y: Vec<BTreeMap<usize, bool>>,
    y_to_x: Vec<BTreeMap<usize, bool>>,
    width: usize,
    height: usize
}

impl RockSet {
    fn new(width: usize, height: usize) -> RockSet {
        RockSet {
            x_to_y: vec![BTreeMap::new(); width],
            y_to_x: vec![BTreeMap::new(); height],
            width,
            height
        }
    }

    fn add(&mut self, x: usize, y: usize, movable: bool) {
        self.x_to_y[x].insert(y, movable);
        self.y_to_x[y].insert(x, movable);
    }

    fn tilt<F>(
        main_map: &mut Vec<BTreeMap<usize, bool>>,
        other_map: &mut Vec<BTreeMap<usize, bool>>,
        increment: i32,
        new_start: usize,
        limit: usize,
        pop_func: F) where F: Fn(&mut BTreeMap<usize, bool>) -> Option<(usize, bool)> 
    {
        let mut new_map = BTreeMap::new();
        for x in 0..limit {
            let mut new_y = new_start;
            while let Some((y, movable)) = pop_func(&mut main_map[x]) {
                match movable {
                    true => {
                        other_map[y].remove(&x);
                        other_map[new_y].insert(x, true);
                        new_map.insert(new_y, true);
                        new_y = (new_y as i32 + increment) as usize;
                    },
                    false => {
                        new_y = (y as i32 + increment) as usize;
                        new_map.insert(y, false);
                    }
                }
            }
            swap(&mut main_map[x], &mut new_map);
            new_map.clear();
        }
    }
}

struct Platform {
    rocks: RockSet
}

impl Platform {
    fn parse(file_name: &str) -> Platform {

        let lines = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .collect::<Vec<_>>();

        let width = lines[0].len();
        let height = lines.len();

        let mut rocks = RockSet::new(width, height);

        let mut y = 0;

        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .for_each(|line| {
                line.char_indices().for_each(|(x, c)| {
                    match c {
                        '#' => rocks.add(x, y, false),
                        'O' => rocks.add(x, y, true),
                        _ => ()
                    }
                });
                y += 1;
            });

        Platform { rocks }
    }

    fn calc_load(&self) -> usize {
        let height = self.rocks.height;

        self.rocks.x_to_y.iter().map(|rocks| {
            rocks.iter()
                .filter(|(_, movable)| **movable)
                .map(|(y, _)| height - y).sum::<usize>()
        })
        .sum::<usize>()
    }

    fn get_points(&self) -> BTreeSet<(usize, usize)> {
        let mut points = BTreeSet::new();
        for x in 0..self.rocks.width {
            for (y, _) in &self.rocks.x_to_y[x] {
                points.insert((x, *y));
            }
        }
        points
    }

    fn tilt_up(&mut self) {
        RockSet::tilt(&mut self.rocks.x_to_y, &mut self.rocks.y_to_x, 1, 0, self.rocks.width, BTreeMap::pop_first);
    }
    fn tilt_down(&mut self) {
        RockSet::tilt(&mut self.rocks.x_to_y, &mut self.rocks.y_to_x, -1, self.rocks.height - 1, self.rocks.width, BTreeMap::pop_last);
    }
    fn tilt_left(&mut self) {
        RockSet::tilt(&mut self.rocks.y_to_x, &mut self.rocks.x_to_y, 1, 0, self.rocks.height, BTreeMap::pop_first);
    }
    fn tilt_right(&mut self) {
        RockSet::tilt(&mut self.rocks.y_to_x, &mut self.rocks.x_to_y, -1, self.rocks.width - 1, self.rocks.height, BTreeMap::pop_last);
    }

    fn spin(&mut self) {
        self.tilt_up();
        self.tilt_left();
        self.tilt_down();
        self.tilt_right();
    }
}

fn part_one(file_name: &str) {
    let mut platform = Platform::parse(file_name);
    platform.tilt_up();
    let load = platform.calc_load();
    println!("Part 1: {}", load);
}

fn part_two(file_name: &str) {
    let mut orientations = HashMap::<BTreeSet<(usize, usize)>, i32>::new();

    let mut platform = Platform::parse(file_name);
    for i in 0..1000000 {
        platform.spin();
        let points = platform.get_points();
        if let Some(prev_i) = orientations.get(&points) {
            let repeat_count = i - prev_i;
            let remaining = (1000000000 - (i + 1)) % repeat_count;
            for _ in 0..remaining {
                platform.spin();
            }
            let load = platform.calc_load();
            println!("Part 2: {}", load);
            break;
        } else {
            orientations.insert(points, i);
        }
    }
    
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
