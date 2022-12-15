use std::collections::{BTreeSet, BTreeMap};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn coords(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct ObjectMap {
    objects_by_x: BTreeMap<i32, BTreeSet<i32>>,
    objects_by_y: BTreeMap<i32, BTreeSet<i32>>,
    measurements: Vec<Measurement>,
    limit: Option<i32>
}

struct Measurement {
    sensor: Point,
    distance: i32
}

impl Measurement {
    fn new(sensor: Point, beacon: Point) -> Measurement {
        let distance = sensor.distance(&beacon);
        Measurement { sensor, distance: distance }
    }
    fn get_empty_space_at(&self, line_y: i32, limit: &Option<i32>, shadow_lines: &mut BTreeMap<i32, i32>) {
        
        if (self.sensor.y - line_y).abs() <= self.distance {
            let line_dist_from_end = match self.sensor.y <= line_y {
                true => self.sensor.y + self.distance - line_y,
                false => line_y - (self.sensor.y - self.distance)
            };

            let mut shadow_start = self.sensor.x - line_dist_from_end;
            let mut shadow_end = self.sensor.x + line_dist_from_end;

            if let Some(limit) = limit {
                shadow_start = shadow_start.max(0);
                shadow_end = shadow_end.min(*limit);
                if shadow_start > shadow_end {
                    return;
                }
            }

            let remove_starts = shadow_lines
                .range(shadow_start..shadow_end + 2)
                .map(|(start, _)| *start)
                .collect::<Vec<_>>();
            
            let mut new_start = shadow_start;
            let mut new_end = shadow_end;

            if let Some(last_start) = remove_starts.last() {
                let last_end = shadow_lines.get(last_start).unwrap();
                new_end = new_end.max(*last_end);
            }

            remove_starts.iter().for_each(|start| {
                shadow_lines.remove(start);
            });
            
            if let Some((first, _)) = shadow_lines.iter().next() {
                if *first <= shadow_start {
                    if let Some((below_start, below_end)) = shadow_lines.range(*first..shadow_start)
                        .rev()
                        .next()
                    {
                        if *below_end >= shadow_start {
                            new_start = *below_start;
                            new_end = new_end.max(*below_end);
                        }
                    }
                }
            }

            shadow_lines.insert(new_start, new_end);
        }
    }
}

fn parse_coord(str: &str) -> i32 {
    str[2..str.len() - 1].parse().unwrap()
}

impl ObjectMap {
    fn new() -> ObjectMap {
        ObjectMap {
            objects_by_x: BTreeMap::new(),
            objects_by_y: BTreeMap::new(),
            measurements: Vec::new(),
            limit: None
        }
    }

    fn add_object(first: i32, second: i32, objects: &mut BTreeMap<i32, BTreeSet<i32>>) {
        let set = if let Some(set) = objects.get_mut(&first) {
            set
        } else {
            let set = BTreeSet::new();
            objects.insert(first, set);
            objects.get_mut(&first).unwrap()
        };

        set.insert(second);
    }

    fn parse_and_add(&mut self, line: String) {
        let pieces = line.split(" ").collect::<Vec<_>>();

        let sensor_x = parse_coord(pieces[2]);
        let sensor_y = parse_coord(pieces[3]);
        let sensor = Point::coords(sensor_x, sensor_y);

        let beacon_x = parse_coord(pieces[8]);
        let beacon_y: i32 = pieces[9][2..].parse().unwrap();
        let beacon = Point::coords(beacon_x, beacon_y);

        Self::add_object(sensor_x, sensor_y, &mut self.objects_by_x);
        Self::add_object(sensor_y, sensor_x, &mut self.objects_by_y);
        Self::add_object(beacon_x, beacon_y, &mut self.objects_by_x);
        Self::add_object(beacon_y, beacon_x, &mut self.objects_by_y);

        self.measurements.push(Measurement::new(sensor, beacon));
    }

    fn determine_shadow_lines(&self, line_y: i32) -> BTreeMap<i32, i32> {
        let mut shadow_lines = BTreeMap::new();

        self.measurements.iter()
            .for_each(|measurement| measurement.get_empty_space_at(line_y,  &self.limit, &mut shadow_lines));
        
        shadow_lines
    }

    fn sum_empty_space_at(&self, line_y: i32) -> usize {
        let shadow_lines = self.determine_shadow_lines(line_y);

        let mut spaces = shadow_lines.iter()
            .map(|(start, end)| (end - start) + 1)
            .sum::<i32>();
        
        let (min, _) = shadow_lines.iter().next().unwrap();

        if let Some(set) = self.objects_by_y.get(&line_y) {
            set.iter().for_each(|x| {
                if x >= min {
                    if let Some((_, end)) = shadow_lines.range(*min..x + 1) 
                        .rev()
                        .next()
                    {
                        if end >= x {
                            spaces -= 1;
                        }
                    }
                }
            });
        }

        spaces as usize
    }

    fn find_empty_space_frequency(&self) -> u64 {
        let limit = self.limit.unwrap();
        let mut empty = (0, 0);
        for y in 0..limit {
            let shadow_lines = self.determine_shadow_lines(y);
            
            let mut new_shadow_lines = BTreeMap::new();
            shadow_lines.iter().reduce(|(prev_start, prev_end), (cur_start, cur_end)| {
                if *prev_end == *cur_start - 1 {
                    new_shadow_lines.insert(*prev_start, *cur_end);
                } else {
                    new_shadow_lines.insert(*prev_start, *prev_end);
                    new_shadow_lines.insert(*cur_start, *cur_end);
                }
                (cur_start, cur_end)
            });
            let shadow_lines = match new_shadow_lines.is_empty() {
                true => shadow_lines,
                false => new_shadow_lines
            };

            if shadow_lines.len() == 1 {
                let (start, end) = shadow_lines.iter().next().unwrap();
                if *start == 1 {
                    if *end == limit {
                        println!("Found possible empty at (0, {})", y);
                        empty = (0, y);
                    } else {
                        panic!("Found invalid shadow line: start={}, end={} at y={}", start, end, y);
                    }
                } else if *start == 0 {
                    if *end == limit - 1 {
                        println!("Found possible empty at ({}, {})", limit, y);
                        empty = (limit, y);
                    } else if *end == limit {
                        continue;
                    } else {
                        panic!("Found invalid shadow line: start={}, end={} at y={}", start, end, y);
                    }
                } else {
                    panic!("Found invalid shadow line: start={}, end={} at y={}", start, end, y);
                }
            } else if shadow_lines.len() == 2 {
                let (first_start, first_end) = shadow_lines.iter().next().unwrap();
                let (last_start, last_end) = shadow_lines.iter().last().unwrap();

                if *first_start != 0 {
                    panic!("Found invalid shadow lines at y={} - {:?}", y, shadow_lines);
                } else if *last_end != limit {
                    panic!("Found invalid shadow lines at y={} - {:?}", y, shadow_lines);
                } else if last_start - first_end != 2 {
                    panic!("Found invalid shadow lines at y={} - {:?}", y, shadow_lines);
                } else {
                    println!("Found possible empty at ({}, {})", first_end + 1, y);
                    empty = (first_end + 1, y);
                }
            } else {
                panic!("Found invalid shadow lines (len > 2) at y={} - {:?}", y, shadow_lines);
            }
        }
        let (x, y) = empty;
        let x = x as u64;
        let y = y as u64;
        (x * 4000000 as u64) + y
    }
}

fn part_one(file_name: &str, line_y: i32) {
    let mut map = ObjectMap::new();

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .for_each(|line| map.parse_and_add(line));

    let empty = map.sum_empty_space_at(line_y);
    
    println!("Part 1: {}", empty);
}

fn part_two(file_name: &str, limit: i32) {
    let mut map = ObjectMap::new();

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .for_each(|line| map.parse_and_add(line));

    map.limit = Some(limit);

    let freq = map.find_empty_space_frequency();
    
    println!("Part 2: {}", freq);
}

fn main() {
    part_one("input.txt", 2000000);
    part_two("input.txt", 4000000);

    println!("Done!");
}
