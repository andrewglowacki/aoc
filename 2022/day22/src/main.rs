use std::collections::{HashSet, BTreeSet, HashMap};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Map {
    y_to_x: Vec<(i32, i32)>,
    x_to_y: Vec<(i32, i32)>,
    walls: HashSet<(i32, i32)>,
    width: i32
}

struct Vector {
    x: i32,
    y: i32
}

impl Vector {
    fn new() -> Vector {
        Vector {
            x: 1,
            y: 0
        }
    }

    fn do_move(&self, x: i32, y: i32) -> (i32, i32) {
        (self.x + x, self.y + y)
    }
    fn turn_left(&mut self) {
        let swap = -1 * self.x;
        self.x = self.y;
        self.y = swap;
    }
    
    fn turn_right(&mut self) {
        let swap = -1 * self.y;
        self.y = self.x;
        self.x = swap;
    }

    fn is_vertical(&self) -> bool {
        self.x == 0
    }

    fn up(&self) -> Vector {
        Vector {
            x: 0,
            y: -1
        }
    }
    fn down(&self) -> Vector {
        Vector {
            x: 0,
            y: 1
        }
    }
    fn left(&self) -> Vector {
        Vector {
            x: -1,
            y: 0
        }
    }
    fn right(&self) -> Vector {
        Vector {
            x: 1,
            y: 0
        }
    }
    fn facing(&self) -> i32 {
        match self.x == 0 {
            true => match self.y == -1 {
                true => 3,
                false => 1
            },
            false => match self.x == -1 {
                true => 2,
                false => 0
            }
        }
    }
}

impl Map {
    fn new(first: String) -> Map {
        let width = first.len() as i32;

        let mut map = Map {
            width,
            y_to_x: Vec::new(),
            x_to_y: vec![(-1, -1); width as usize],
            walls: HashSet::new()
        };

        map.add_line(first);

        map
    }

    fn add_line(&mut self, line: String) {
        let line = line.chars().collect::<Vec<_>>();
        
        let mut left = -1;
        for i in 0..line.len() {
            if line[i] != ' ' {
                left = i as i32;
                break;
            }
        }

        let mut right = -1;
        for i in 0..line.len() {
            let i = (line.len() - i) - 1;
            if line[i] != ' ' {
                right = i as i32;
                break;
            }
        }

        let y = self.y_to_x.len() as i32;
        
        for x in left..right + 1 {
            if x as usize >= self.x_to_y.len() {
                self.x_to_y.push((-1, -1));
                self.width += 1;
            }
            let (min, max) = &mut self.x_to_y[x as usize];
            if *min == -1 {
                *min = y;
            }
            *max = y;
            match line[x as usize]{
                '#' => {
                    self.walls.insert((x, y));
                },
                '.' => (),
                _ => panic!("Unexpected character at ({}, {})", x, y)
            }
        }
        
        self.y_to_x.push((left, right));
    }

    fn follow_directions(&self, movements: Vec<Movement>) -> (i32, i32, i32) {
        let mut x = self.y_to_x[0].0;
        let mut y = 0;

        let mut vector = Vector::new();

        for movement in movements {
            if let Movement::Walk(amount) = movement {
                if vector.is_vertical() {
                    let min = self.y_min(x);
                    let max = self.y_max(x);
                    for _ in 0..amount {
                        let mut dest = y + vector.y;
                        if dest > max {
                            dest = min;
                        } else if  dest < min {
                            dest = max;
                        }
                        if self.walls.contains(&(x, dest)) {
                            break;
                        }
                        y = dest;
                    }
                } else {
                    let min = self.x_min(y);
                    let max = self.x_max(y);
                    for _ in 0..amount {
                        let mut dest = x + vector.x;
                        if dest > max {
                            dest = min;
                        } else if  dest < min {
                            dest = max;
                        }
                        if self.walls.contains(&(dest, y)) {
                            break;
                        }
                        x = dest;
                    }
                }
            } else {
                match movement {
                    Movement::Left => vector.turn_left(),
                    Movement::Right => vector.turn_right(),
                    _ => panic!("impossible")
                };
            }
        }

        let facing = vector.facing();
        (x, y, facing)
    }

    fn y_min(&self, x: i32) -> i32 {
        self.x_to_y[x as usize].0
    }
    fn y_max(&self, x: i32) -> i32 {
        self.x_to_y[x as usize].1
    }
    fn x_min(&self, y: i32) -> i32 {
        self.y_to_x[y as usize].0
    }
    fn x_max(&self, y: i32) -> i32 {
        self.y_to_x[y as usize].1
    }

    fn follow_directions_cube(&self, movements: Vec<Movement>) -> (i32, i32, i32) {
        let mut x = self.y_to_x[0].0;
        let mut y = 0;

        let mut vector = Vector::new();
        
        // A: x=149, y=0-49 - x=99, y=100-149
        // B: x=50, y=50-99 - y=100, x=0-49
        // C: x=50, y=0-49 - x=0, y=100-149
        // D: x=99, y=50-99 - y=49, x=100-149
        // E: y=149, x=50-99 - x=49, y=150-199
        // F: y=0, x=50-99 - x=0, y=150-199
        // G: y=0, x=100-149 - y=199, x=0..49
        //
        //        __F____G___
        //       C|    |    |A
        //        |____|____|
        //       B|    |D
        //   __B__|____|
        //  C|    |    |A
        //   |____|____|
        //  F|    |E
        //   |____|
        //      G

        let mut visited = HashSet::<(i32, i32)>::new();
        for movement in movements {
            if let Movement::Walk(amount) = movement {
                print!("Moving from {}, {} by {}", x, y, amount);
                for _ in 0..amount {
                    let (mut dest_x, mut dest_y) = vector.do_move(x, y);

                    let mut new_vector: Option<Vector> = None;

                    if vector.is_vertical() {
                        if dest_y > self.y_max(x) {
                            if x < 50 {
                                // G
                                // From: y=199, x=0..49
                                // To: y=0, x=100-149
                                dest_y = 0;
                                dest_x = x + 100;
                                new_vector = Some(vector.down());
                            } else if x < 100 {
                                // E
                                // From: y=149, x=50-99
                                // To: x=49, y=150-199
                                dest_x = 49;
                                dest_y = x + 100;
                                new_vector = Some(vector.up());
                            } else {
                                // D
                                // From: y=49, x=100-149
                                // To: x=49, y=50-99
                                dest_x = 99;
                                dest_y = x - 50;
                                new_vector = Some(vector.left());
                            }
                        } else if  dest_y < self.y_min(x) {
                            if x < 50 {
                                // B
                                // From: y=100, x=0-49
                                // To: x=50, y=50-99
                                dest_x = 50;
                                dest_y = x + 50;
                                new_vector = Some(vector.right());
                            } else if x < 100 {
                                // F
                                // From: y=0, x=50-99
                                // To: x=0, y=150-199
                                dest_x = 0;
                                dest_y = 150 + (x - 50);
                                new_vector = Some(vector.right());
                            } else {
                                // G
                                // From: y=0, x=100-149
                                // To: y=199, x=0..49
                                dest_y = 199;
                                dest_x = x - 100;
                                new_vector = Some(vector.down());
                            }
                        }
                    } else {
                        if dest_x > self.x_max(y) {
                            if y < 50 {
                                // A
                                // From: x=149, y=0-49 
                                // To: x=99, y=100-149
                                dest_x = 99;
                                dest_y = 149 - y;
                                new_vector = Some(vector.left());
                            } else if y < 100 {
                                // D
                                // From: x=99, y=50-99
                                // To: y=49, x=100-149
                                dest_y = 49;
                                dest_x = y + 50;
                                new_vector = Some(vector.up());
                            } else if y < 150 {
                                // A
                                // From: x=99, y=100-149
                                // To: x=149, y=0-49
                                dest_x = 149;
                                dest_y = 149 - y;
                                new_vector = Some(vector.left());
                            } else if y < 200 {
                                // E
                                // From: x=49, y=150-199
                                // To: y=149, x=50-99
                                dest_y = 149;
                                dest_x = 50 + (y - 150);
                                new_vector = Some(vector.up());
                            } else {
                                panic!("y value is out of range: {}", y);
                            }
                        } else if dest_x < self.x_min(y) {
                            if y < 50 {
                                // C
                                // From: x=50, y=0-49
                                // To: x=0, y=100-149
                                dest_x = 0;
                                dest_y = 149 - y;
                                new_vector = Some(vector.right());
                            } else if y < 100 {
                                // B
                                // From: x=50, y=50-99
                                // To: y=100, x=0-49
                                dest_y = 100;
                                dest_x =  y - 50;
                                new_vector = Some(vector.down());
                            } else if y < 150 {
                                // C
                                // From: x=0, y=100-149
                                // To: x=50, y=0-49
                                dest_x = 50;
                                dest_y = 149 - y;
                                new_vector = Some(vector.right());
                            } else if y < 200 {
                                // F
                                // From: x=0, y=150-199
                                // To: y=0, x=50-99
                                dest_y = 0;
                                dest_x = 50 + (y - 150);
                                new_vector = Some(vector.down());
                            } else {
                                panic!("y value is out of range: {}", y);
                            }
                        }
                    }
                    if self.walls.contains(&(dest_x, dest_y)) {
                        break;
                    }
                    x = dest_x;
                    y = dest_y;
                    visited.insert((x, y));
                    if let Some(new_vector) = new_vector {
                        vector = new_vector;
                    }
                }
                println!(" to {}, {} - facing: {}", x, y, vector.facing());
            } else {
                println!("Rotating: {:?}", movement);
                match movement {
                    Movement::Left => vector.turn_left(),
                    Movement::Right => vector.turn_right(),
                    _ => panic!("impossible")
                };
            }
        }
        
        let mut y = 0;
        for (min, max) in &self.y_to_x {
            let min = *min;
            let max = *max;
            for _ in 0..min {
                print!(" ");
            }

            for x in min..max + 1 {
                let point = (x, y);
                if visited.contains(&point) {
                    print!("x");
                } else if (self.walls.contains(&point)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }

            for _ in max + 1..self.width {
                print!(" ");
            }
            println!("");
            y += 1;
        }

        let facing = vector.facing();
        (x, y, facing)
    }

}

#[derive(Debug)]
enum Movement {
    Walk(i32),
    Left,
    Right
}

fn parse_movements(line: String) -> Vec<Movement> {
    let line_str = line;
    let line = line_str.chars().collect::<Vec<_>>();
    let mut movements = Vec::new();
    let mut last_start = 0;
    for i in 0..line.len() {
        match line[i] {
            'L' => {
                let amount = line_str[last_start..i].parse::<i32>().unwrap();
                movements.push(Movement::Walk(amount));
                movements.push(Movement::Left);
                last_start = i + 1;
            },
            'R' => {
                let amount = line_str[last_start..i].parse::<i32>().unwrap();
                movements.push(Movement::Walk(amount));
                movements.push(Movement::Right);
                last_start = i + 1;
            },
            _ => ()
        }
    }
    let last_amount = line_str[last_start..].parse::<i32>().unwrap();
    movements.push(Movement::Walk(last_amount));
    movements
}

fn part_one(file_name: &str) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());

    let first = lines.next().unwrap();
    let mut map = Map::new(first);

    (&mut lines).take_while(|line| !line.is_empty())
        .for_each(|line| map.add_line(line));
    
    let map = map;
    let movements = parse_movements(lines.next().unwrap());

    let (x, y, facing) = map.follow_directions(movements);

    let password = (4 * (x + 1)) + (1000 * (y + 1)) + facing;

    println!("Part 1: {}", password);
}

fn part_two(file_name: &str) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());

    let first = lines.next().unwrap();
    let mut map = Map::new(first);

    (&mut lines).take_while(|line| !line.is_empty())
        .for_each(|line| map.add_line(line));
    
    let map = map;
    let movements = parse_movements(lines.next().unwrap());

    let (x, y, facing) = map.follow_directions_cube(movements);

    let password = (4 * (x + 1)) + (1000 * (y + 1)) + facing;

    println!("Part 1: {}", password);
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
