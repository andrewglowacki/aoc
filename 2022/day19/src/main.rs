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

    fn time_until(&self, ore: i32, clay: i32, obsidian: i32) -> i32 {
        let ore_left = ore - self.ore;
        let clay_left = clay - self.clay;
        let obsidian_left = obsidian - self.obsidian;

        let mut time = 0;
        if ore_left > 0 {
            time = ore_left / self.ore_rate;
            if ore_left % self.ore_rate > 0 {
                time += 1;
            }
        }

        if clay_left > 0 {
            let mut clay_time = clay_left / self.clay_rate;
            if clay_left % self.clay_rate > 0 {
                clay_time += 1;
            }
            time = time.max(clay_time);
        }

        if obsidian_left > 0 {
            let mut obsidian_time = obsidian_left / self.obsidian_rate;
            if obsidian_left % self.obsidian_rate > 0 {
                obsidian_time += 1;
            }
            time = time.max(obsidian_time);
        }

        time
    }
}

#[derive(Debug)]
enum State {
    Start,
    Middle,
    End
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

        let ore_max = clay
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
                if resources.ore >= self.ore && resources.ore_rate < self.ore_max {
                    resources.ore_rate += 1;
                    resources.ore -= self.ore;
                    true
                } else {
                    false
                }
            }
            CLAY_BOT => {
                if resources.ore >= self.clay && resources.clay_rate < self.obsidian_clay {
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
                if resources.obsidian >= self.geode_obsidian && resources.ore >= self.geode_ore {
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

    fn compute_quality(&self, max_minutes: i32) -> i32 {
        let mut resources = Resources::new();
        
        let mut decisions = vec![
            (CLAY_BOT, State::Start, 0)
        ];

        let mut minutes = 0;
        let mut max_geodes = 0;

        while decisions.len() > 0 {
            let (mut current, state, added_minutes) = decisions.pop().unwrap();

            match state {
                State::Start => (),
                State::Middle => {
                    minutes -= added_minutes;
                    self.sell(&mut resources, current + 1);
                    resources.mine(-1 * added_minutes);
                },
                State::End => {
                    minutes -= added_minutes;
                    self.sell(&mut resources, 0);
                    resources.mine(-1 * added_minutes);
                    continue;
                }
            }

            let mut time_until = -1;
            loop {
                let this_time_until = match current {
                    ORE_BOT => {
                        match resources.ore_rate < self.ore_max {
                            true => resources.time_until(self.ore, 0, 0),
                            false => -1
                        }
                    },
                    CLAY_BOT => {
                        match resources.clay_rate < self.obsidian_clay {
                            true => resources.time_until(self.clay, 0, 0),
                            false => -1
                        }
                    },
                    OBSIDIAN_BOT => {
                        match resources.obsidian_rate < self.geode_obsidian {
                            true => resources.time_until(self.obsidian_ore, self.obsidian_clay, 0),
                            false => -1
                        }
                    },
                    GEODE_BOT => resources.time_until(self.geode_ore, 0, self.geode_obsidian),
                    _ => panic!("Unexpected bot: {}", current)
                };
                if this_time_until < 0 || this_time_until + minutes + 1 >= max_minutes {
                    if current == 0 {
                        break;
                    }
                    current -= 1;
                } else {
                    time_until = this_time_until;
                    break;
                }
            }

            if time_until == -1 {
                if resources.geode_rate > 0 {
                    let total = resources.geode_rate * (max_minutes - minutes);
                    let total = total + resources.geode;
                    max_geodes = max_geodes.max(total);
                }
                continue;
            }

            let minutes_added = time_until + 1;
            resources.mine(minutes_added);
            self.buy(&mut resources, current);
            minutes += minutes_added;

            let (current, state) = match current == 0 {
                true => (0, State::End),
                false => (current - 1, State::Middle)
            };
            decisions.push((current, state, minutes_added));

            let next_option_start = if resources.obsidian_rate > 0 {
                GEODE_BOT
            } else if resources.clay_rate > 0 {
                OBSIDIAN_BOT
            } else {
                CLAY_BOT
            };

            decisions.push((next_option_start, State::Start, 0));
        }

        max_geodes * self.id
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
        .map(|blueprint| blueprint.compute_quality(24))
        .sum::<i32>();
    
    println!("Part 1: {}", quality_total);
}

fn part_two(file_name: &str) {
    let mut id = 1;
    let product = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| {
            let this_id = id;
            id += 1;
            Blueprint::parse(line, this_id)
        })
        .take(3)
        .map(|blueprint| blueprint.compute_quality(32) / blueprint.id)
        .fold(1, |product, current| product * current);
    
    println!("Part 2: {}", product);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
