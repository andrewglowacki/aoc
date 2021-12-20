use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct PolymerManual {
    insertions: HashMap<(char, char), char>,
    template: Vec<char>,
}

impl PolymerManual {

    fn from_file(file_name: &str) -> PolymerManual {
        let mut lines = get_file_lines(file_name)
            .flat_map(|line| line.ok());
        
        let template = lines.next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>();
        
        lines.next(); // skip blank

        let mut insertions = HashMap::new();

        lines.for_each(|line| {
            let insertion = line.chars()
                .collect::<Vec<_>>();
            let first = insertion[0];
            let second = insertion[1];
            let insert = insertion[6];
            insertions.insert((first, second), insert);
        });

        PolymerManual {
            template, 
            insertions
        }
    }

    fn apply(&mut self, iterations: usize) -> HashMap<char, u64> {
        let mut pairs = HashMap::<(char, char), u64>::new();
        for i in 0..self.template.len() - 1 {
            let a = self.template[i];
            let b = self.template[i + 1];
            if let Some(count) = pairs.get_mut(&(a, b)) {
                *count += 1; 
            } else {
                pairs.insert((a, b), 1);
            }
        }

        for _ in 0..iterations {
            let mut new_pairs = HashMap::<(char, char), u64>::new();
            for (pair, count) in pairs.iter() {
                let (a, b) = pair;
                let a = *a;
                let b = *b;
                if let Some(insert) = self.insertions.get(&pair) {
                    let insert = *insert;
                    if let Some(new_count) = new_pairs.get_mut(&(a, insert)) {
                        *new_count += *count; 
                    } else {
                        new_pairs.insert((a, insert), *count);
                    }
                    if let Some(new_count) = new_pairs.get_mut(&(insert, b)) {
                        *new_count += *count; 
                    } else {
                        new_pairs.insert((insert, b), *count);
                    }
                } else {
                    if let Some(new_count) = new_pairs.get_mut(pair) {
                        *new_count += *count; 
                    } else {
                        new_pairs.insert((a, b), *count);
                    }
                }
            }
            pairs = new_pairs;
        }

        let mut base_counts = HashMap::<char, u64>::new();

        for (pair, count) in pairs.iter() {
            let (a, _) = pair;
            if let Some(final_count) = base_counts.get_mut(a) {
                *final_count += *count;
            } else {
                base_counts.insert(*a, *count);
            }
        }

        let last = self.template.len() - 1;
        let last_count = base_counts.get_mut(&self.template[last]).unwrap();
        *last_count += 1;

        base_counts
    }

    fn determine_answer(&self, counts: HashMap<char, u64>) -> u64 {
        let mut max_base = 'Z';
        let mut max_count: u64 = 0;
        let mut min_base = 'Z';
        let mut min_count = u64::MAX;
        
        counts.iter()
            .for_each(|(base, count)| {
                let base = *base;
                let count = *count;
                if count > max_count {
                    max_count = count;
                    max_base = base;
                }
                if count < min_count {
                    min_count = count;
                    min_base = base;
                }
            });

        assert_ne!('Z', min_base);
        assert_ne!('Z', max_base);

        max_count - min_count
    }
}


fn part_one(file_name: &str) {
    let mut manual = PolymerManual::from_file(file_name);
    
    let counts = manual.apply(10);

    println!("Part 1: {}", manual.determine_answer(counts));
}

fn part_two(file_name: &str) {
    let mut manual = PolymerManual::from_file(file_name);

    let counts = manual.apply(40);
    
    println!("Part 2: {}", manual.determine_answer(counts));
}

fn main() {
    part_one("input.txt");
    // part_two("sample.txt");
    part_two("input.txt");

    println!("Done!");
}
