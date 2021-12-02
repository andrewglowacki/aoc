use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::{BTreeSet, HashMap};

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn parse_joltages(file_name: &str) -> BTreeSet<u32> {
    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<BTreeSet<u32>>()
}

struct Adapter {
    higher: Vec<u32>,
    paths_to_end: u64
}

fn run_input(file_name: &str) {
    let mut joltages = parse_joltages(file_name);
    
    let (diff_by_one, diff_by_three, _) = joltages.iter().fold((0, 0, 0), |result, joltage| {
        let joltage = *joltage;
        let (diff_by_one, diff_by_three, last_joltage) = result;
        match joltage - last_joltage {
            0 | 2 => (diff_by_one, diff_by_three, joltage),
            1 => (diff_by_one + 1, diff_by_three, joltage),
            3 => (diff_by_one, diff_by_three + 1, joltage),
            _ => panic!("Invalid next adapter. Last: {} current: {}", last_joltage, joltage)
        }
    });
    let diff_by_three = diff_by_three + 1; // last_adapter -> device

    println!("For {}: Differences by one: {} by three: {}, multiplied: {}", file_name, diff_by_one, diff_by_three, (diff_by_one * diff_by_three));
    
    joltages.insert(0);

    let mut adapters = joltages.iter()
        .map(|joltage| {
            let joltage = *joltage;
            let higher = joltages.range((joltage + 1)..(joltage + 4))
                .map(|joltage| *joltage)
                .collect::<Vec<u32>>();
            
            (joltage, Adapter { paths_to_end: 0, higher })
        })
        .collect::<HashMap<u32, Adapter>>();

    let max_joltage = joltages.iter().last().unwrap();
    let mut last_adapter = adapters.get_mut(&max_joltage).unwrap();
    last_adapter.paths_to_end = 1;

    for joltage in joltages.iter().rev().skip(1) {
        let adapter = adapters.get(&joltage).unwrap();
        let mut sum = 0;
        for next in adapter.higher.iter() {
            sum += adapters.get(&next).unwrap().paths_to_end;
        }
        let mut adapter = adapters.get_mut(&joltage).unwrap();
        adapter.paths_to_end = sum;
    }
    println!("For {}: {}", file_name, adapters.get(&0).unwrap().paths_to_end);
}

fn main() {
    run_input("sample.txt");
    run_input("input.txt");
}
