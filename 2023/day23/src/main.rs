use std::collections::{HashMap, HashSet};
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


struct Forest {
    trails: HashMap<(i32, i32), (i32, i32)>,
    end: (i32, i32)
}

struct Visited {
    set: HashSet<(i32, i32)>,
    list: Vec<(i32, i32)>,
    longest: usize
}

impl Visited {
    fn new() -> Visited {
        Visited {
            set: HashSet::new(),
            list: Vec::new(),
            longest: 0
        }
    }
    fn len(&self) -> usize {
        self.list.len()
    }
    fn contains(&self, point: &(i32, i32)) -> bool {
        self.set.contains(point)
    }
    fn reset_to(&mut self, length: usize) {
        let diff = self.list.len() - length;
        for _ in 0..diff {
            let point = self.list.pop().unwrap();
            self.set.remove(&point);
        }
    }
    fn add(&mut self, point: (i32, i32)) {
        if self.set.insert(point) {
            self.list.push(point);
        }
    }
    fn update_longest(&mut self) {
        let current = self.list.len();
        if current > self.longest {
            self.longest = current;
            println!("Longest is now {}", current);
        }
    }
}

impl Forest {
    fn parse(file_name: &str) -> Forest {
        let mut y = 0;
        let trails = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .flat_map(|line| {
                let mut x = 0;
                let points = line.chars()
                    .flat_map(|c| {
                        let slope = match c {
                            '#' => None,
                            '.' => Some((0, 0)),
                            '>' => Some((1, 0)),
                            '<' => Some((-1, 0)),
                            '^' => Some((0, -1)),
                            'v' => Some((0, 1)),
                            _ => panic!("Unexpected character: {}", c)
                        };
                        let point = slope.map(|slope| ((x, y), slope));
                        x += 1;
                        point
                    })
                    .collect::<Vec<_>>();
                y += 1;
                points
            })
            .collect::<HashMap<_,_>>();

        let height = y;
        let end = trails.iter().find(|((_, y), slope)| match slope {
            (0, 0) => *y == height - 1,
            _ => false
        })
        .map(|(point, _)| *point)
        .unwrap();

        Forest {
            end,
            trails
        }
    }
    

    fn find_longest_trail_slopes(
        &self, 
        mut next: (i32, i32), 
        mut visited: HashSet<(i32, i32)>) -> usize 
    {
        visited.insert(next);

        if next == self.end {
            // println!("Found the end with visited: {:?}", visited);
            return visited.len();
        }

        let mut longest = 0;
        let mut new_next_points = Vec::new();
        let mut flat_points = Vec::<((i32, i32), Vec<(i32, i32)>)>::new();
        let mut branches = Vec::<((i32, i32), Vec<(i32, i32)>, HashSet<(i32, i32)>)>::new();

        // loop until thre are new
        loop {
            let (x, y) = next;
            let neighbors = vec![
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1)
            ];

            // out of the possible neighbors,
            // find ones that are on the trail
            // and haven't been visited yet
            let mut next_points = neighbors.into_iter()
                .filter(|point| !visited.contains(point))
                .filter_map(|point| match self.trails.get(&point) {
                    Some(slope) => Some((point, slope, Vec::new())),
                    None => None
                })
                .collect::<Vec<_>>();

            // println!("From ({}, {}), next points are: {:?}", x, y, next_points);

            // for each of the next points, follow any 
            // sloped trails. If one of these points
            // isn't on a slope, just add it to the flat_points.
            while next_points.len() > 0 {

                while let Some(((x, y), (slope_x, slope_y), mut followed)) = next_points.pop() {
                    if *slope_x == 0 && *slope_y == 0 {
                        flat_points.push(((x, y), followed));
                    } else {
                        followed.push((x, y));
                        let next_point = (x + slope_x, y + slope_y);
                        if !visited.contains(&next_point) {
                            if let Some(new_slope) = self.trails.get(&next_point) {
                                new_next_points.push((next_point, new_slope, followed));
                            }
                        }
                    }
                }
                
                swap(&mut new_next_points, &mut next_points);
            }

            if flat_points.is_empty() {
                // println!("No flat points left, longest is: {}", longest);
                for (next, followed, mut visited) in branches {
                    // println!("Branching with next: {:?}", next);
                    followed.into_iter().for_each(|point| { 
                        visited.insert(point); 
                    });
                    let this_longest = self.find_longest_trail_slopes(next, visited);
                    longest = longest.max(this_longest);
                }
                
                
                return longest;
            } else {
                // continue on the trail with the next flat point
                let next_tuple = flat_points.pop().unwrap();
                next = next_tuple.0;
                let followed = next_tuple.1;
                
                // for each of the other possible directions
                // the trail could go, find the longest trail
                while let Some((next, followed)) = flat_points.pop() {
                    branches.push((next, followed, visited.clone()));
                }
                
                followed.into_iter().for_each(|point| { 
                    visited.insert(point); 
                });
                visited.insert(next);
                
                if next == self.end {
                    longest = longest.max(visited.len());

                    for (next, followed, mut visited) in branches {
                        // println!("Branching with next: {:?}", next);
                        followed.into_iter().for_each(|point| { 
                            visited.insert(point); 
                        });
                        let this_longest = self.find_longest_trail_slopes(next, visited);
                        longest = longest.max(this_longest);
                    }

                    return longest;
                }
                // println!("Next iteration will be: {:?}", next);
            }
        }
    }

    fn find_longest_trail_no_slopes(
        &self, 
        mut next: (i32, i32), 
        visited: &mut Visited) -> usize 
    {
        visited.add(next);

        if next == self.end {
            // println!("Found the end with size: {}", visited.len());
            visited.update_longest();
            return visited.len();
        }

        let mut longest = 0;
        let mut flat_points = Vec::<(i32, i32)>::new();

        // loop until thre are new
        loop {
            let (x, y) = next;
            let neighbors = vec![
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1)
            ];

            neighbors.into_iter()
                .filter(|point| !visited.contains(point) && self.trails.contains_key(point))
                .for_each(|point| flat_points.push(point));

            if flat_points.is_empty() {
                // println!("No flat points left, longest is: {}", longest);
                return longest;
            } else {
                // continue on the trail with the next flat point
                next = flat_points.pop().unwrap();
                
                // for each of the other possible directions
                // the trail could go, find the longest trail
                while let Some(next) = flat_points.pop() {
                    let visit_len = visited.len();
                    let this_longest = self.find_longest_trail_no_slopes(next, visited);
                    visited.reset_to(visit_len);
                    longest = longest.max(this_longest);
                }
                
                visited.add(next);
                
                if next == self.end {
                    // println!("Found the end with visited (count {}): {:?}", visited.len(), visited);
                    // for y in 0..23 {
                    //     for x in 0..23 {
                    //         let point = (x, y);
                    //         if visited.contains(&point) {
                    //             print!("O");
                    //         } else if let Some(slope) = self.trails.get(&point) {
                    //             match slope {
                    //                 (0, 0) => print!("."),
                    //                 (1, 0) => print!(">"),
                    //                 (-1, 0) => print!("<"),
                    //                 (0, -1) => print!("^"),
                    //                 (0, 1) => print!("^"),
                    //                 _ => panic!("Unexpected slop: {:?}", slope)
                    //             }
                    //         } else {
                    //             print!("#");
                    //         }
                    //     }
                    //     println!("");
                    // }
                    longest = longest.max(visited.len());
                    visited.update_longest();

                    // println!("Found end with longest {}", longest);
                    return longest;
                }
                // println!("Next iteration will be: {:?}", next);
            }
        }
    }
}

fn part_one(file_name: &str) {
    let forest = Forest::parse(file_name);
    let longest_trail = forest.find_longest_trail_slopes((1, 0), HashSet::new()) - 1;
    println!("Part 1: {}", longest_trail);
}

fn part_two(file_name: &str) {
    let forest = Forest::parse(file_name);
    let longest_trail = forest.find_longest_trail_no_slopes((1, 0), &mut Visited::new()) - 1;
    println!("Part 2: {}", longest_trail);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
