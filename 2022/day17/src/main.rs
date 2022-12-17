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
    fn drop_rocks(&mut self, count: usize, shapes: &Shapes) {
        let mut jet_index = 0;
        for i in 0..count {
            let shape = shapes.get(i);
            
            // println!("Initial state:");
            // shape.states[2].iter().rev().for_each(|row| {
            //     print!("|");
            //     for i in 0..7 {
            //         let shift = 6 - i;
            //         print!("{}", match (row >> shift) & 1 > 0 {
            //             true => '#',
            //             false => '.'
            //         });
            //     }
            //     println!("|");
            // });

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
                
                // println!("After jet move: {:?}", jet);
                // shape.states[state].iter().rev().for_each(|row| {
                //     print!("|");
                //     for i in 0..7 {
                //         let shift = 6 - i;
                //         print!("{}", match (row >> shift) & 1 > 0 {
                //             true => '#',
                //             false => '.'
                //         });
                //     }
                //     println!("|");
                // });

                if y == 0 {
                    break;
                } else if self.overlaps(shape, y - 1, state) {
                    break;
                } else {
                    y -= 1;
                }
            }

            self.add_shape(shape, y, state);

            // println!("After rock {}:", i + 1);
            // self.rows.iter().rev().for_each(|row| {
            //     print!("|");
            //     for i in 0..7 {
            //         let shift = 6 - i;
            //         print!("{}", match (row >> shift) & 1 > 0 {
            //             true => '#',
            //             false => '.'
            //         });
            //     }
            //     println!("|");
            // });
            // println!("+-------+");
        }
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

fn run_simulation(part: usize, file_name: &str, count: usize) {
    let pattern = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .next()
        .unwrap();
    
    let jets = parse_jets(pattern);
    let shapes = Shapes::new();
    let mut rocks = Rocks::new(jets);

    rocks.drop_rocks(count, &shapes);

    let height = rocks.rows.len();
    
    println!("Part {}: {}", part, height);
}

fn main() {
    run_simulation(1, "sample.txt", 2022);
    // run_simulation(2, "input.txt", 1000000000000);

    println!("Done!");
}
