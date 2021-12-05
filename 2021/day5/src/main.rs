use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Line {
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32
}

impl Line {

    /**
     * Parses input like: 
     * x1,y1 -> x2,y2
     * into Line structs
     */
    fn parse(line: &String) -> Line {
        let points = line.split(" -> ")
            .collect::<Vec<&str>>();
        let start_components = points[0].split(",")
            .flat_map(|component| component.parse())
            .collect::<Vec<i32>>();
        let end_components = points[1].split(",")
            .flat_map(|component| component.parse())
            .collect::<Vec<i32>>();
        
        let start_x = start_components[0];
        let start_y = start_components[1];
        let end_x = end_components[0];
        let end_y = end_components[1];

        Line { start_x, start_y, end_x, end_y }
    }

    fn is_straight(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    fn is_horizontal(&self) -> bool {
        self.start_y == self.end_y
    }

    fn is_vertical(&self) -> bool {
        self.start_x == self.end_x
    }

    fn sort_points<'a>(a_start: &'a mut  i32, b_start: &'a mut i32, a_end: &'a mut i32, b_end: &'a mut i32) {
        if a_start > a_end {
            let swap = *a_end;
            *a_end = *a_start;
            *a_start = swap;
        }
        if b_start > b_end {
            let swap = *b_end;
            *b_end = *b_start;
            *b_start = swap;
        }
    }

    // Possibilities after sorting:
    // 1. a---A b---B
    // 2. a--bA--B
    // 3. b--aB--A
    // 4. a-b---B-A
    // 5. b-a---A-B
    // 6. b---B a---A
    fn intersects_parallel(mut a_start: i32, mut b_start: i32, mut a_end: i32, mut b_end: i32) -> bool {
        Line::sort_points(&mut a_start, &mut b_start, &mut a_end, &mut b_end);
        a_start <= b_end && a_end >= b_start
    }

    fn get_parallel_diff(mut a_start: i32, mut b_start: i32, mut a_end: i32, mut b_end: i32) -> Range<i32> {
        if Line::intersects_parallel(a_start, b_start, a_end, b_end) {
            Line::sort_points(&mut a_start, &mut b_start, &mut a_end, &mut b_end);
            let mut points = vec![a_start, b_start, a_end, b_end];
            points.sort();
            points[1]..(points[2] + 1)
        } else {
            0..0
        }
    }

    fn get_perpendicular_intersection(horizontal: &Line, vertical: &Line) -> Vec<(i32, i32)> {
        let mut start_x = horizontal.start_x;
        let mut end_x = horizontal.end_x;
        let mut start_y = vertical.start_y;
        let mut end_y = vertical.end_y;
        Line::sort_points(&mut start_x, &mut start_y, &mut end_x, &mut end_y);
        
        if start_x <= vertical.start_x && 
            end_x >= vertical.start_x && 
            start_y <= horizontal.start_y &&
            end_y >= horizontal.start_y 
        {
            let mut intersection = Vec::with_capacity(1);
            intersection.push((vertical.start_x, horizontal.start_y));
            intersection
        }
        else
        {
            Vec::with_capacity(0)
        }
    }

    fn get_intersections(&self, other: &Line) -> Vec<(i32, i32)> {
        match (
            self.is_horizontal(), 
            other.is_horizontal(), 
            self.start_y == other.start_y, 
            self.start_x == other.start_x
        ) {
            (true, true, true, _) => {
                Line::get_parallel_diff(self.start_x, other.start_x, self.end_x, other.end_x)
                    .map(|number| (number, self.start_y))
                    .collect::<Vec<(i32, i32)>>()
            },
            (false, false, _, true) => {
                Line::get_parallel_diff(self.start_y, other.start_y, self.end_y, other.end_y)
                    .map(|number| (self.start_x, number))
                    .collect::<Vec<(i32, i32)>>()
            },
            (true, false, _, _) => Line::get_perpendicular_intersection(self, other),
            (false, true, _, _) => Line::get_perpendicular_intersection(other, self),
            _ => Vec::with_capacity(0)
        }
    }

}

fn read_lines(file_name: &str) -> Vec<Line> {
    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| { Line::parse(&line) })
        .collect::<Vec<Line>>()
}

fn part_one(file_name: &str) {
    let lines = read_lines(file_name)
        .into_iter()
        .filter(|line| line.is_straight())
        .collect::<Vec<Line>>();

    let mut points = HashSet::new();

    for i in 0..lines.len() {
        let one = &lines[i];
        for j in (i + 1)..lines.len() {
            let two = &lines[j];
            one.get_intersections(two)
                .into_iter()
                .for_each(|point| { points.insert(point); });
        }
    }

    println!("Part 1: {}", points.len());
}

fn add_points(points: Vec<(i32, i32)>, point_counts: &mut HashMap<(i32, i32), i32>) {
    points.into_iter().for_each(|point| {
        if let Some(count) = point_counts.get_mut(&point) {
            *count += 1;
        } else {
            point_counts.insert(point, 1);
        }
    });
}

fn part_two(file_name: &str) {
    let lines = read_lines(file_name);

    let mut point_counts = HashMap::<(i32,i32), i32>::new();

    // #EZmode
    for line in lines {
        let mut start_x = line.start_x;
        let mut end_x = line.end_x;
        let mut start_y = line.start_y;
        let mut end_y = line.end_y;

        if line.is_horizontal() {
            Line::sort_points(&mut start_x, &mut start_y, &mut end_x, &mut end_y);
            let points = (start_x..(end_x + 1))
                .map(|x| (x, start_y))
                .collect::<Vec<_>>();
            add_points(points, &mut point_counts);
        } else if line.is_vertical() {
            Line::sort_points(&mut start_x, &mut start_y, &mut end_x, &mut end_y);
            let points = (start_y..(end_y + 1))
                .map(|y| (start_x, y))
                .collect::<Vec<_>>();
            add_points(points, &mut point_counts);
        } else {
            let mut x_increment = 1;
            let mut y_increment = 1;
            let mut count = end_x - start_x;

            if start_x > end_x {
                x_increment = -1;
                count = start_x - end_x;
            }
            if start_y > end_y {
                y_increment = -1;
            }

            let count = count + 1;

            let points = (0..count)
                .map(|i| (start_x + (i * x_increment), start_y + (i * y_increment)))
                .collect::<Vec<_>>();
            add_points(points, &mut point_counts);
        }
    }

    let count = point_counts.iter()
        .filter(|(_, count)| **count > 1)
        .count();
    
    println!("Part 2: {}", count);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");
    // part_two("sample.txt");

    println!("Done!");
}
