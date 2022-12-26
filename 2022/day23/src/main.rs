use std::collections::{HashSet, HashMap};
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
    elves: HashSet<(i32, i32)>,
    moves: Vec<Box<dyn Fn(&Self, i32, i32) -> Option<(i32, i32)>>>,
    index: usize
}

enum Proposal {
    Single((i32, i32)),
    Multiple
}

struct Proposals {
    proposals: HashMap<(i32, i32), Proposal>
}

impl Proposals {
    fn new() -> Proposals {
        Proposals {
            proposals: HashMap::new()
        }
    }
    fn add(&mut self, dest: (i32, i32), current: (i32, i32)) {
        let proposal = self.proposals.get_mut(&dest);

        if let Some(x) = proposal {
            *x = Proposal::Multiple;
        } else {
            self.proposals.insert(dest, Proposal::Single(current));
        }
    }
}

impl Map {
    fn from_file(file_name: &str) -> Map {
        let mut elves = HashSet::new();

        let mut y = 0;
        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .for_each(|line| {
                let mut x = 0;
                for c in line.chars() {
                    if c == '#' {
                        elves.insert((x, y));
                    }
                    x += 1;
                }
                y += 1;
            });
        
        Map {
            elves,
            moves: vec![
                Box::new(Map::open_up),
                Box::new(Map::open_down),
                Box::new(Map::open_left),
                Box::new(Map::open_right)
            ],
            index: 0
        }
    }

    fn open_up(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        match !self.elves.contains(&(x - 1, y - 1)) && 
              !self.elves.contains(&(x,     y - 1)) && 
              !self.elves.contains(&(x + 1, y - 1))
        {
            true => Some((x, y - 1)),
            false => None
        }
    }
    fn open_down(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        match !self.elves.contains(&(x - 1, y + 1)) && 
              !self.elves.contains(&(x,     y + 1)) && 
              !self.elves.contains(&(x + 1, y + 1))
        {
            true => Some((x, y + 1)),
            false => None
        }
    }
    fn open_left(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        match !self.elves.contains(&(x - 1, y - 1)) && 
              !self.elves.contains(&(x - 1, y    )) && 
              !self.elves.contains(&(x - 1, y + 1))
        {
            true => Some((x - 1, y)),
            false => None
        }
    }
    fn open_right(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        match !self.elves.contains(&(x + 1, y - 1)) && 
              !self.elves.contains(&(x + 1, y    )) && 
              !self.elves.contains(&(x + 1, y + 1))
        {
            true => Some((x + 1, y)),
            false => None
        }
    }

    fn is_alone(&self, x: i32, y: i32) -> bool {
        // up
        !self.elves.contains(&(x - 1, y - 1)) &&
        !self.elves.contains(&(x    , y - 1)) &&
        !self.elves.contains(&(x + 1, y - 1)) &&
        // down
        !self.elves.contains(&(x - 1, y + 1)) &&
        !self.elves.contains(&(x    , y + 1)) &&
        !self.elves.contains(&(x + 1, y + 1)) &&
        // left
        !self.elves.contains(&(x - 1, y - 1)) &&
        !self.elves.contains(&(x - 1, y    )) &&
        !self.elves.contains(&(x - 1, y + 1)) &&
        // right
        !self.elves.contains(&(x + 1, y - 1)) &&
        !self.elves.contains(&(x + 1, y    )) &&
        !self.elves.contains(&(x + 1, y + 1))
    }

    fn next_move(&mut self) -> usize {
        let mut proposals = Proposals::new();

        for (x, y) in self.elves.iter() {
            let x = *x;
            let y = *y;

            if self.is_alone(x, y) {
                continue;
            }
            
            for i in self.index..self.index + 4 {
                if let Some(dest) = self.moves[i % 4](&self, x, y) {
                    proposals.add(dest, (x, y));
                    break;
                }
            }
        }

        let mut moves = 0;
        for (dest, proposal) in proposals.proposals.into_iter() {
            if let Proposal::Single(current) = proposal {
                self.elves.remove(&current);
                self.elves.insert(dest);
                moves += 1;
            }
        }

        self.index = (self.index + 1) % 4;
        moves
    }

    fn calc_empty_tiles(&self) -> i32 {
        let mut x_min = i32::MAX;
        let mut y_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_max = i32::MIN;

        self.elves.iter().for_each(|(x, y)| {
            x_min = x_min.min(*x);
            y_min = y_min.min(*y);
            x_max = x_max.max(*x);
            y_max = y_max.max(*y);
        });

        // println!("x: {} - {} y: {} - {}", x_min, x_max, y_min, y_max);
        // for y in y_min..y_max + 1 {
        //     for x in x_min..x_max + 1 {
        //         if self.elves.contains(&(x, y)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!("");
        // }

        let width = (x_max - x_min) + 1;
        let height = (y_max - y_min) + 1;
        let spaces = width * height;
        spaces - self.elves.len() as i32
    }
}

fn part_one(file_name: &str) {
    let mut map = Map::from_file(file_name);

    for _ in 0..10 {
        map.next_move();
    }

    let empty = map.calc_empty_tiles();

    println!("Part 1: {}", empty);
}

fn part_two(file_name: &str) {
    let mut map = Map::from_file(file_name);

    let mut moves = 1;
    while map.next_move() > 0 {
        moves += 1;
    }
    
    println!("Part 2: {}", moves);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
