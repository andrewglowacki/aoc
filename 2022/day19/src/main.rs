use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Blueprint {
    id: u32,
    ore: u32,
    clay: u32,
    obsidian_ore: u32,
    obsidian_clay: u32,
    geode_ore: u32,
    geode_obsidian: u32
}

struct Resources {
    ore: u32,
    ore_rate: u32,
    clay: u32,
    clay_rate: u32,
    obsidian: u32,
    obsidian_rate: u32,
    geode: u32,
    geode_rate: u32
}

impl Resources {
    fn new() -> Resources {
        Resources {
            ore: 0,
            ore_rate: 1,
            clay: 0,
            clay_rate: 0, 
            obsidian: 0, 
            obsidian_rate: 0, 
            geode: 0, 
            geode_rate: 0
        }
    }
    
    fn mine(&mut self) {
        self.ore += self.ore_rate;
        self.clay += self.clay_rate;
        self.obsidian += self.obsidian_rate;
        self.geode += self.geode_rate;
    }

    fn spend(&mut self, blueprint: &Blueprint) {
        let clay_target = blueprint.obsidian_clay / blueprint.obsidian_ore;
        if self.clay_rate < clay_target {
            if self.ore >= blueprint.clay {
                self.clay_rate += 1;
                self.ore -= blueprint.clay;
            }
        } else {
            let obsidian_target = blueprint.geode_obsidian / blueprint.geode_ore;
            if self.obsidian_rate < obsidian_target {
                if self.obsidian + self.obsidian_rate < blueprint.geode_obsidian {
                    if self.ore >= blueprint.obsidian_ore && self.clay >= blueprint.obsidian_clay {
                        self.obsidian_rate += 1;
                        self.ore -= blueprint.obsidian_ore;
                        self.clay -= blueprint.obsidian_clay;
                    }
                }
            }
            if self.obsidian >= blueprint.geode_obsidian && self.ore >= blueprint.ore {
                self.geode_rate += 1;
                self.obsidian -= blueprint.geode_obsidian;
                self.ore -= blueprint.geode_ore;
            }
        }
    }

    fn print(&self) {
        println!("Ore: {} Clay: {} Obsidian: {} Geode: {} - OreBots: {} ClayBots: {} ObsidianBots: {} GeodeBots: {}", self.ore, self.clay, self.obsidian, self.geode, self.ore_rate, self.clay_rate, self.obsidian_rate, self.geode_rate);
    }
}

impl Blueprint {
    fn parse(line: String, id: u32) -> Blueprint {
        let pieces = line.split(" ").collect::<Vec<_>>();

        Blueprint {
            id,
            ore: pieces[6].parse::<u32>().unwrap(),
            clay: pieces[12].parse::<u32>().unwrap(),
            obsidian_ore: pieces[18].parse::<u32>().unwrap(),
            obsidian_clay: pieces[21].parse::<u32>().unwrap(),
            geode_ore: pieces[27].parse::<u32>().unwrap(),
            geode_obsidian: pieces[30].parse::<u32>().unwrap()
        }
    }

    fn compute_quality(&self) -> u32 {
        let mut resources = Resources::new();
        for _ in 0..24 {
            resources.spend(&self);
            resources.mine();
            resources.print();
        }
        let quality = resources.geode * self.id;
        println!("Quality is {}", quality);
        quality
    }
}

fn part_one(file_name: &str) {
    let mut id = 1;
    let quality_total = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| {
            let this_id = id;
            id += 1;
            Blueprint::parse(line, this_id)
        })
        .map(|blueprint| blueprint.compute_quality())
        .sum::<u32>();
    
    println!("Part 1: {}", quality_total);
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("sample.txt");
    part_two("input.txt");

    println!("Done!");
}
