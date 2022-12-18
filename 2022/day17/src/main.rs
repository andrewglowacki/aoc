use std::collections::BTreeSet;
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
enum Jet {
    Left,
    Right
}

struct Shape {
    states: Vec<Vec<u8>>,
    height: usize
}

impl Shape {
    fn new(left_most: Vec<u8>) -> Shape {
        let height = left_most.len();
        let mut states = Vec::new();
        states.push(left_most.to_owned());

        let mut current = left_most;
        while current.iter()
            .find(|row| *row & 1 != 0)
            .is_none()
        {
            current.iter_mut().for_each(|row| *row = *row >> 1);
            states.push(current.to_owned());
        }

        let mut shape = Shape {
            states,
            height
        };
        
        shape
    }

    fn next_state(&self, jet: &Jet, current_state: usize) -> usize {
        match jet {
            Jet::Left => match current_state == 0 {
                true => 0,
                false => current_state - 1
            },
            Jet::Right => match current_state + 1 < self.states.len() {
                true => current_state + 1,
                false => current_state
            }
        }
    }

    fn overlaps(&self, current_state: usize, rocks: &[u8]) -> bool {
        let state = &self.states[current_state];
        for r in 0..rocks.len() {
            let shape_row = state[r];
            let rock_row = rocks[r];
            if shape_row & rock_row > 0 {
                return true;
            }
        }
        false
    }
}

struct Shapes {
    shapes: Vec<Shape>
}

impl Shapes {
    fn new() -> Shapes {
        // all shapes are upside down
        let shapes = vec![
            // horizontal line
            Shape::new(vec![
                0b1111000
            ]),
            // plus
            Shape::new(vec![
                0b0100000,
                0b1110000,
                0b0100000
            ]),
            // backward L
            Shape::new(vec![
                0b1110000,
                0b0010000,
                0b0010000
            ]),
            // vertical line
            Shape::new(vec![
                0b1000000,
                0b1000000,
                0b1000000,
                0b1000000
            ]),
            // square
            Shape::new(vec![
                0b1100000,
                0b1100000
            ])
        ];

        Shapes {
            shapes
        }
    }

    fn get(&self, index: usize) -> &Shape {
        &self.shapes[index % self.shapes.len()]
    }
}

struct Rocks {
    jets: Vec<Jet>,
    rows: Vec<u8>,
}

impl Rocks {
    fn new(jets: Vec<Jet>) -> Rocks {
        Rocks {
            jets,
            rows: Vec::new()
        }
    }

    fn drop_rocks_until<F>(&mut self, shapes: &Shapes, condition: F) -> usize
        where F: Fn(&Rocks, usize) -> bool
    {
        let mut jet_index = 0;
        let mut i = 0;
        while condition(self, i) {
            let shape = shapes.get(i);
            
            // move with jet first
            let mut y = self.rows.len() + 3;
            let mut state = 2;
            loop {
                let jet = &self.jets[jet_index % self.jets.len()];
                let next_state = shape.next_state(jet, state);
                if next_state != state {
                    state = match self.overlaps(shape, y, next_state) {
                        true => state,
                        false => next_state
                    };
                }
                jet_index += 1;
                
                if y == 0 {
                    break;
                } else if self.overlaps(shape, y - 1, state) {
                    break;
                } else {
                    y -= 1;
                }
            }

            self.add_shape(shape, y, state);
            i += 1;
        }
        i
    }

    fn add_shape(&mut self, shape: &Shape, y: usize, current_state: usize) {
        let top = y + shape.height;
        let y_end = top.min(self.rows.len());
        let copy_count = y_end - y;

        let state = &shape.states[current_state];

        for i in 0..copy_count {
            self.rows[y + i] = self.rows[y + i] | state[i];
        }
        for i in copy_count..state.len() {
            self.rows.push(state[i]);
        }
    }

    fn overlaps(&self, shape: &Shape, y: usize, state: usize) -> bool {
        if y >= self.rows.len() {
            return false;
        }
        let top = y + shape.height;
        let y_end = top.min(self.rows.len());
        let rocks = &self.rows[y..y_end];
        shape.overlaps(state, rocks)
    }
}

fn parse_jets(line: String) -> Vec<Jet> {
    line.chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("Invalid direction: {}", c)
        })
        .collect::<Vec<_>>()
}

fn run_simulation(part: usize, file_name: &str, count: usize) -> Rocks {
    let pattern = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .next()
        .unwrap();
    
    let jets = parse_jets(pattern);
    let shapes = Shapes::new();
    let mut rocks = Rocks::new(jets);

    rocks.drop_rocks_until(&shapes, |_, dropped| dropped < count);

    let height = rocks.rows.len();
    
    if part == 1 {
        println!("Part 1: {}", height);
    }

    rocks
}


fn run_until_height(height: usize, file_name: &str, last_row: u8) {
    let pattern = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .next()
        .unwrap();
    
    let jets = parse_jets(pattern);
    let shapes = Shapes::new();
    let mut rocks = Rocks::new(jets);

    let dropped = rocks.drop_rocks_until(&shapes, |rocks, _| {
        rocks.rows.len() != height || rocks.rows[rocks.rows.len() - 1] != last_row
    });

    println!("dropped {}", dropped);
}

fn main() {
    run_simulation(1, "input.txt", 2022);

    let part_2_file = "input.txt";
    let rocks = run_simulation(2, part_2_file, 100000);

    // find a pattern
    let mut pattern: Option<(usize, usize)> = None;
    for i in 0..rocks.rows.len() {
        let mut pattern_end: Option<usize> = None;
        let start = rocks.rows[i];
        for j in i + 13..rocks.rows.len() {
            if rocks.rows[j] == start {
                let len = j - i;
                if j + len >= rocks.rows.len() {
                    break;
                }
                if rocks.rows[i..j] == rocks.rows[j..j + len] {
                    pattern_end = Some(j);
                    break;
                }
            }
        }
        if let Some(end) = pattern_end {
            pattern = Some((i, end - i));
            break;
        }
    }

    let (start, interval) = pattern.unwrap();

    let height = start + interval;
    let last_row = rocks.rows[height - 1];

    println!("Detected repeat every {} starting at {}", interval, start);

    run_until_height(height, part_2_file, last_row);

    println!("Done!");
}
