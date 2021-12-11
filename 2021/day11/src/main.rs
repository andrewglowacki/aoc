use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Grid {
    rows: Vec<Vec<u32>>,
    cascaded: Vec<Vec<bool>>,
    width: usize,
    height: usize
}

impl Grid {
    fn from_file(file_name: &str) -> Grid {
        let rows = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        
        let cascaded = rows.iter()
            .map(|row| vec![false; row.len()])
            .collect::<Vec<_>>();
        
        let height = rows.len();
        let width = rows[0].len();
        
        Grid {
            rows,
            cascaded,
            width,
            height
        }
    }

    fn cascade_energy(&mut self, r: usize, c: usize) {
        // don't cascade this octo if it has been already
        if self.cascaded[r][c] || self.rows[r][c] <= 9 {
            return;
        }

        self.cascaded[r][c] = true;

        let r = r as i32;
        let c = c as i32;

        let points = vec![
            (r - 1, c - 1),
            (r - 1, c),
            (r - 1, c + 1),
            (r, c - 1),
            (r, c + 1),
            (r + 1, c - 1),
            (r + 1, c),
            (r + 1, c + 1)
        ];

        // remove points that are out of bounds
        let points = points.iter().filter(|(r, c)| {
            *r >= 0 && *r < self.height as i32 && *c >= 0 && *c < self.width as i32
        })
        .collect::<Vec<_>>();
        
        points.iter().for_each(|(r, c)| {
            let r = *r as usize;
            let c = *c as usize;
            self.rows[r][c] += 1;
            self.cascade_energy(r, c);
        });
    }

    fn execute_step(&mut self) -> u32 {
        // mark all of the octos as not having cascaded
        // their energy to their neighbors yet
        self.cascaded.iter_mut()
            .flat_map(|row| row)
            .for_each(|cascaded| *cascaded = false);
        
        // add one energy to each octo for this step
        self.rows.iter_mut()
            .flat_map(|row| row)
            .for_each(|energy| *energy += 1);

        // cascade the energy of each octo 
        // over 9 to it's neighbords
        for r in 0..self.height {
            for c in 0..self.width {
                self.cascade_energy(r, c);
            }
        }

        // reset entery to 0 if an octos  
        // has energy greater than 9 and
        // return the number of flashes we had
        self.rows.iter_mut()
            .flat_map(|row| row)
            .filter(|energy| **energy > 9)
            .map(|energy| *energy = 0)
            .count() as u32
    }

    fn _print(&self) {
        self.rows.iter().for_each(|row| {
            row.iter().for_each(|energy| {
                print!("{}", energy);
            });
            println!("");
        });
    }
}

fn part_one(file_name: &str) {
    let mut grid = Grid::from_file(file_name);

    let flashes = (0..100)
        .map(|_| grid.execute_step())
        .sum::<u32>();
    
    println!("Part 1: {}", flashes);
}

fn part_two(file_name: &str) {
    let mut grid = Grid::from_file(file_name);

    let octos = (grid.width * grid.height) as u32;
    let mut steps = 1;
    while grid.execute_step() < octos {
        steps += 1;
    }
    
    println!("Part 2: {}", steps);
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");
    part_two("input.txt");

    println!("Done!");
}
