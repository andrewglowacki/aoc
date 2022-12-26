use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

const ORE_BOT: usize = 0;
const CLAY_BOT: usize = 1;
const OBSIDIAN_BOT: usize = 2;
const GEODE_BOT: usize = 3;

struct Blueprint {
    id: i32,
    ore: i32,
    clay: i32,
    obsidian_ore: i32,
    obsidian_clay: i32,
    geode_ore: i32,
    geode_obsidian: i32,
    ore_max: i32
}

enum Decision {
    Wait(i32),
    Build(Option<usize>, usize)
}

struct Resources {
    ore: i32,
    ore_rate: i32,
    clay: i32,
    clay_rate: i32,
    obsidian: i32,
    obsidian_rate: i32,
    geode: i32,
    geode_rate: i32
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
    
    fn mine(&mut self, amount: i32) {
        self.ore += self.ore_rate * amount;
        self.clay += self.clay_rate * amount;
        self.obsidian += self.obsidian_rate * amount;
        self.geode += self.geode_rate * amount;
    }

}

impl Blueprint {
    fn parse(line: String, id: i32) -> Blueprint {
        let pieces = line.split(" ").collect::<Vec<_>>();
        
        let ore = pieces[6].parse::<i32>().unwrap();
        let clay = pieces[12].parse::<i32>().unwrap();
        let obsidian_ore = pieces[18].parse::<i32>().unwrap();
        let obsidian_clay = pieces[21].parse::<i32>().unwrap();
        let geode_ore = pieces[27].parse::<i32>().unwrap();
        let geode_obsidian = pieces[30].parse::<i32>().unwrap();

        let ore_max = ore.max(clay)
            .max(obsidian_ore)
            .max(geode_ore);
        
        Blueprint {
            id,
            ore,
            clay,
            obsidian_ore,
            obsidian_clay,
            geode_ore,
            geode_obsidian,
            ore_max
        }
    }

    fn sell(&self, resources: &mut Resources, bot: usize) {
        match bot {
            ORE_BOT => {
                resources.ore_rate -= 1;
                resources.ore += self.ore;
            },
            CLAY_BOT => {
                resources.clay_rate -= 1;
                resources.ore += self.clay;
            },
            OBSIDIAN_BOT => {
                resources.obsidian_rate -= 1;
                resources.ore += self.obsidian_ore;
                resources.clay += self.obsidian_clay;
            },
            GEODE_BOT => {
                resources.geode_rate -= 1;
                resources.ore += self.geode_ore;
                resources.obsidian += self.geode_obsidian;
            },
            _ => panic!("Invalid bot type: {}", bot)
        }
    }

    fn buy(&self, resources: &mut Resources, bot: usize) -> bool {
        match bot {
            ORE_BOT => {
                if resources.ore > self.ore && resources.ore_rate < self.ore_max {
                    resources.ore_rate += 1;
                    resources.ore -= self.ore;
                    true
                } else {
                    false
                }
            }
            CLAY_BOT => {
                if resources.ore > self.clay && resources.clay_rate < self.obsidian_clay {
                    resources.clay_rate += 1;
                    resources.ore -= self.clay;
                    true
                } else {
                    false
                }
            }
            OBSIDIAN_BOT => {
                if resources.clay >= self.obsidian_clay && 
                    resources.ore >= self.obsidian_ore && 
                    resources.obsidian_rate < self.geode_obsidian 
                {
                    resources.obsidian_rate += 1;
                    resources.ore -= self.obsidian_ore;
                    resources.clay -= self.obsidian_clay;
                    true
                } else {
                    false
                }
            },
            GEODE_BOT => {
                if resources.obsidian >= self.geode_obsidian && resources.ore > self.geode_ore {
                    resources.geode_rate += 1;
                    resources.ore -= self.geode_ore;
                    resources.obsidian -= self.geode_obsidian;
                    true
                } else {
                    false
                }
            },
            _ => panic!("Invalid bot: {}", bot)
        }
    }

    fn compute_quality(&self) -> i32 {
        let mut resources = Resources::new();
        
        let mut decisions = vec![
            Decision::Wait(self.clay)
        ];

        let mut minutes = self.clay + 1;
        let mut max_geodes = 0;

        while decisions.len() > 0 {
            let mut decision = decisions.pop().unwrap();
            if let Decision::Wait(time) = &mut decision {
                
                // reset time from the previous operation
                minutes -= *time + 1;
                resources.mine(*time * -1);

                if *time > 0 {
                    minutes += *time;
                    resources.mine(*time);
                    if resources.geode > max_geodes {
                        max_geodes = resources.geode;
                    }
                    *time -= 1;
                } else {
                    decision = Decision::Build(None, 0);
                    if minutes >= 23 {
                        // we don't get any benefit out of any more bots
                        continue;
                    }
                }
            }

            if let Decision::Build(previous, mut next) = decision {
                if let Some(previous) = previous {
                    minutes -= 1;
                    self.sell(&mut resources, previous);
                }

                let mut found = false;
                while next < 4 && !found {
                    if self.buy(&mut resources, next) {
                        minutes -= 1;
                        found = true;
                    }
                    next += 1;
                }
                if !found {
                    continue;
                }
                decision = Decision::Build(Some(next - 1), next);
            }
            
            decisions.push(decision);

            // determine how much time to wait
            let mut max = self.ore_max - resources.ore_rate;
            if resources.clay_rate > 0 {
                max = max.max(self.obsidian_clay - resources.clay_rate);
                if resources.obsidian_rate > 0 {
                    max = max.max(self.geode_obsidian - resources.obsidian_rate);
                    if resources.geode_rate > 0 {
                        max = max.max(minutes);
                    }
                }
            }
            max = max.min(minutes);

            minutes += max + 1;
            decisions.push(Decision::Wait(max));
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
        .sum::<i32>();
    
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
