use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Enhancer {
    algorithm: Vec<bool>,
    index: usize,
    r: i32,
    c: i32
}

impl Enhancer {
    fn new(algorithm: Vec<bool>) -> Enhancer {
        Enhancer {
            algorithm,
            index: 0,
            r: 0,
            c: 0
        }
    }

    fn get(&self) -> bool {
        self.algorithm[self.index]
    }

    fn set(&mut self, image: &Image, center_r: i32, center_c: i32) {
        let mut index = 0;
        for r in center_r - 1..center_r + 2 {
            for c in center_c - 1..center_c + 2 {
                index = index << 1;
                index = index | image.get_bit(r, c);
            }
        }
        self.r = center_r;
        self.c = center_c;
        self.index = index;
    }
    fn shift_right(&mut self, image: &Image) {
        // 8 7 6 5 4 3 2 1 0
        // << 1
        // 7 6 5 4 3 2 1 0 X
        // 8 7 6 5 4 3 2 1 0
        // ^ ^ R ^ ^ R ^ ^ R
        //
        self.c += 1;
        self.index = (self.index << 1) & 0x1B6; // shift and clear reset bits
        let top = image.get_bit(self.r - 1, self.c + 1);
        let mid = image.get_bit(self.r, self.c + 1);
        let bot = image.get_bit(self.r + 1, self.c + 1);
        self.index = self.index | (top << 6) | (mid << 3) | bot;
    }
}

struct Image {
    rows: Vec<Vec<bool>>,
    width: i32,
    height: i32,
    infinite_bit: usize
}

impl Image {
    fn get_bit(&self, r: i32, c: i32) -> usize {
        if r >= 0 && r < self.height {
            if c >= 0 && c < self.width {
                let row = &self.rows[r as usize];
                return row[c as usize] as usize;
            }
        }
        self.infinite_bit
    }
    fn new(rows: Vec<Vec<bool>>, infinite_bit: usize) -> Image {
        let height = rows.len() as i32;
        let width = rows[0].len() as i32;
        Image { 
            rows,
            width, 
            height,
            infinite_bit
        }
    }
    fn print(&self) {
        for r in 0..self.height as usize {
            let row = &self.rows[r];
            for c in 0..self.width as usize {
                if row[c] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn read_input(file_name: &str) -> (Enhancer, Image) {
    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    let algorithm = lines.next().unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();

    lines.next(); // skip blank
    
    let image = lines.map(|line| {
        line.chars().map(|c| c == '#')
            .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    (Enhancer::new(algorithm), Image::new(image, 0))
}

fn enhance(mut image: Image, mut enhancer: Enhancer, times: usize) -> Image {
    for _ in 0..times {
        let mut new_image = Vec::new();
        for r in -2..image.height + 2 {
            let mut new_row = Vec::new();
            enhancer.set(&image, r, -2);
            for _ in -2..image.width + 2 {
                let new_bit = enhancer.get(); 
                new_row.push(new_bit);
                enhancer.shift_right(&image);
            }
            new_image.push(new_row);
        }
        let infinite_bit = match enhancer.algorithm[0] {
            true => (image.infinite_bit + 1) % 2,
            false => 0
        };
        image = Image::new(new_image, infinite_bit);
    }

    image
}

fn part_one(file_name: &str) {
    let (enhancer, image) = read_input(file_name);

    let image = enhance(image, enhancer, 2);

    let lit = image.rows.iter()
        .flat_map(|row| row)
        .filter(|lit| **lit)
        .count();

    println!("Part 1: {}", lit);
}

fn part_two(file_name: &str) {
    let (enhancer, image) = read_input(file_name);

    let image = enhance(image, enhancer, 50);
    
    let lit = image.rows.iter()
        .flat_map(|row| row)
        .filter(|lit| **lit)
        .count();

    println!("Part 2: {}", lit);
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");
    part_two("input.txt");

    println!("Done!");
}
