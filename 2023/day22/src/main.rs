use core::num;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
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

struct Range {
    from: i32,
    to: i32
}

impl Range {
    fn new(a: i32, b: i32) -> Range {
        Range {
            from: a.min(b),
            to: a.max(b)
        }
    }
    fn intersects(&self, other: &Range) -> bool {
        self.from <= other.to && self.to >= other.from
    }
    fn new_from(&mut self, new_from: i32) {
        let diff = self.from - new_from;
        self.from = new_from;
        self.to -= diff;
    }
}

struct Block {
    x: Range,
    y: Range,
    z: Range
}

impl Block {
    fn parse(line: String) -> Block {
        let start_end = line.split('~').collect::<Vec<_>>();
        let start = start_end[0].split(',')
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let end = start_end[1].split(',')
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let x = Range::new(start[0], end[0]);
        let y = Range::new(start[1], end[1]);
        let z = Range::new(start[2], end[2]);

        Block { x, y, z }
    }
}

struct BlockPile {
    xy_to_block: HashMap<(i32, i32), Vec<usize>>,
    block_to_points: Vec<Vec<(i32, i32)>>,
    blocks: Vec<Block>,
    z_to_block: BTreeMap<i32, Vec<usize>>
}

impl BlockPile {
    fn parse(file_name: &str) -> BlockPile {
        let mut pile = BlockPile { 
            xy_to_block: HashMap::new(),
            block_to_points: Vec::new(),
            z_to_block: BTreeMap::new(),
            blocks: Vec::new()
        };

        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| Block::parse(line))
            .for_each(|block| pile.add_block(block));

        pile
    }

    fn add_block(&mut self, block: Block) {
        let block_id = self.blocks.len();
        let mut points = Vec::new();

        for x in block.x.from..block.x.to + 1 {
            for y in block.y.from..block.y.to + 1 {
                points.push((x, y));
            }
        }

        points.iter().for_each(|point| {
            if let Some(column) = self.xy_to_block.get_mut(&point) {
                column.push(block_id);
            } else {
                self.xy_to_block.insert(*point, vec![block_id]);
            }
        });

        if let Some(blocks) = self.z_to_block.get_mut(&block.z.from) {
            blocks.push(block_id);
        } else {
            self.z_to_block.insert(block.z.from, vec![block_id]);
        }

        self.block_to_points.push(points);
        self.blocks.push(block);
    }

    fn drop_blocks(&mut self) {
        let z_last = *self.z_to_block.keys().last().unwrap(); 
        let mut new_blocks = Vec::<usize>::new();
        for z in 2..z_last {
            if let Some(mut blocks) = self.z_to_block.remove(&z) {
                while let Some(block_id) = blocks.pop() {
                    let points = &self.block_to_points[block_id];
                    let below = points.iter()
                        .flat_map(|point| self.xy_to_block.get(point).unwrap())
                        .filter(|check_block_id| **check_block_id != block_id)
                        .map(|block_id| self.blocks[*block_id].z.to)
                        .filter(|test_z| *test_z < z)
                        .max()
                        .unwrap_or(0);
                    let new_z_bottom = below + 1;
                    if new_z_bottom < z {
                        if let Some(blocks) = self.z_to_block.get_mut(&new_z_bottom) {
                            blocks.push(block_id);
                        } else {
                            self.z_to_block.insert(new_z_bottom, vec![block_id]);
                        }

                        self.blocks[block_id].z.new_from(new_z_bottom);
                    } else {
                        new_blocks.push(block_id);
                    }
                }
                swap(&mut new_blocks, &mut blocks);
            }
        }
    }

    fn count_independent(&self) -> usize {
        let max = self.blocks.iter()
            .map(|block| block.z.to)
            .max()
            .unwrap();

        let mut z_top_to_blocks = Vec::new();
        for _ in 0..max + 1 {
            z_top_to_blocks.push(Vec::<usize>::new());
        }

        for block_id in 0..self.blocks.len() {
            let top = self.blocks[block_id].z.to;
            z_top_to_blocks[top as usize].push(block_id);
        }

        let mut supported_by = HashMap::<usize, Vec<usize>>::new();
        
        for z in 2..z_top_to_blocks.len() {
            if let Some(blocks) = self.z_to_block.get(&(z as i32)) {
            }
        }

        0
    }
}

fn part_one(file_name: &str) {
    let mut pile = BlockPile::parse(file_name);
    pile.drop_blocks();
    let count = pile.count_independent();
    println!("Part 1: {}", count);
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("sample.txt");
    part_two("sample.txt");

    println!("Done!");
}
