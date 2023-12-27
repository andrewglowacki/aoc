use core::num;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::iter::FromIterator;
use std::mem::swap;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Clone)]
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

#[derive(Clone)]
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
        for z in 2..z_last + 1 {
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
                    let mut new_z_bottom = below + 1;
                    if new_z_bottom >= z {
                         new_z_bottom = z;
                    }
                    if let Some(blocks) = self.z_to_block.get_mut(&new_z_bottom) {
                        blocks.push(block_id);
                    } else {
                        self.z_to_block.insert(new_z_bottom, vec![block_id]);
                    }

                    self.blocks[block_id].z.new_from(new_z_bottom);
                }
            }
        }
    }

    fn count_independent(&self) -> usize {
        let max = self.blocks.iter()
            .map(|block| block.z.to)
            .max()
            .unwrap();

        let mut z_top_to_blocks = Vec::new();
        let mut z_bottom_to_blocks = Vec::new();
        for _ in 0..max + 1 {
            z_top_to_blocks.push(HashSet::<usize>::new());
            z_bottom_to_blocks.push(HashSet::<usize>::new());
        }

        for block_id in 0..self.blocks.len() {
            let top = self.blocks[block_id].z.to;
            let bottom = self.blocks[block_id].z.from;
            z_top_to_blocks[top as usize].insert(block_id);
            z_bottom_to_blocks[bottom as usize].insert(block_id);
        }

        let mut key_is_supported_by = HashMap::<usize, Vec<usize>>::new();
        
        for (z, blocks) in self.z_to_block.range(2..) {
            let below = &z_top_to_blocks[(*z as usize) - 1];
            for block in blocks {
                let supporting = self.block_to_points[*block].iter()
                    .flat_map(|point| self.xy_to_block.get(point).unwrap())
                    .filter(|block_id| below.contains(block_id))
                    .copied()
                    .collect::<HashSet<_>>();
                
                (&mut key_is_supported_by).insert(*block, Vec::from_iter(supporting.into_iter()));
            }
        }

        let mut key_is_supporting = HashMap::<usize, Vec<usize>>::new();

        for (above, below_blocks) in key_is_supported_by.iter() {
            for below in below_blocks {
                if let Some(list) = key_is_supporting.get_mut(&below) {
                    list.push(*above);
                } else {
                    key_is_supporting.insert(*below, vec![*above]);
                }
            }
        }

        let mut count = 0;

        for i in 0..self.blocks.len() {
            if let Some(supporting) = key_is_supporting.get(&i) {
                let min_above_supports = supporting.iter()
                    .map(|block| key_is_supported_by.get(block).unwrap().len())
                    .min()
                    .unwrap();
                if min_above_supports > 1 {
                    println!("Block {} is dissolvable with supporting: {:?} and min above: {}", i, supporting, min_above_supports);
                    count += 1;
                }
            } else {
                println!("Block {} is dissolvable not supporting anything", i);
                count += 1;
            }
        }

        count
    }
    
    fn sum_fallen_if_dissolved(&self) -> usize {
        let max = self.blocks.iter()
            .map(|block| block.z.to)
            .max()
            .unwrap();

        let mut z_top_to_blocks = Vec::new();
        let mut z_bottom_to_blocks = Vec::new();
        for _ in 0..max + 1 {
            z_top_to_blocks.push(HashSet::<usize>::new());
            z_bottom_to_blocks.push(HashSet::<usize>::new());
        }

        for block_id in 0..self.blocks.len() {
            let top = self.blocks[block_id].z.to;
            let bottom = self.blocks[block_id].z.from;
            z_top_to_blocks[top as usize].insert(block_id);
            z_bottom_to_blocks[bottom as usize].insert(block_id);
        }

        let mut key_is_supported_by = HashMap::<usize, Vec<usize>>::new();
        
        for (z, blocks) in self.z_to_block.range(2..) {
            let below = &z_top_to_blocks[(*z as usize) - 1];
            for block in blocks {
                let supporting = self.block_to_points[*block].iter()
                    .flat_map(|point| self.xy_to_block.get(point).unwrap())
                    .filter(|block_id| below.contains(block_id))
                    .copied()
                    .collect::<HashSet<_>>();
                
                (&mut key_is_supported_by).insert(*block, Vec::from_iter(supporting.into_iter()));
            }
        }

        let mut key_is_supporting = HashMap::<usize, Vec<usize>>::new();

        for (above, below_blocks) in key_is_supported_by.iter() {
            for below in below_blocks {
                if let Some(list) = key_is_supporting.get_mut(&below) {
                    list.push(*above);
                } else {
                    key_is_supporting.insert(*below, vec![*above]);
                }
            }
        }

        let mut count = 0;

        for i in 0..self.blocks.len() {
            let mut dropped = HashSet::<usize>::new();
            let mut to_check = Vec::new();
            to_check.push(i);
            while let Some(check_block) = to_check.pop() {
                if let Some(supporting) = key_is_supporting.get(&check_block) {
                    let to_fall = supporting.iter()
                        .filter(|block| {
                            key_is_supported_by.get(block)
                                .unwrap()
                                .iter()
                                .filter(|by_block| !dropped.contains(by_block))
                                .count() <= 1
                        })
                        .collect::<Vec<_>>();

                    count += to_fall.len();

                    to_fall.into_iter().for_each(|block| {
                        to_check.push(*block);
                        dropped.insert(*block);
                    });
                }
            }
        }

        count
    }
}

fn part_one(file_name: &str) {
    let mut pile = BlockPile::parse(file_name);
    pile.drop_blocks();
    let count = pile.count_independent();
    println!("Part 1: {}", count);
}

fn part_two(file_name: &str) {
    let mut pile = BlockPile::parse(file_name);
    pile.drop_blocks();
    let total = pile.sum_fallen_if_dissolved();
    println!("Part 2: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");

    println!("Done!");
}
