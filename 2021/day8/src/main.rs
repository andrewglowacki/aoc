use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Debug)]
struct Pattern {
    chars: HashSet<char>,
    pattern: String
}

impl Pattern {
    fn parse(line: &str) -> Pattern {
        let chars = line.chars().collect::<HashSet<char>>();
        let mut ordered = chars.iter()
            .map(|c| *c as u8)
            .collect::<Vec<u8>>();
        ordered.sort();
        let pattern = String::from_utf8(ordered).unwrap();
        Pattern {
            chars,
            pattern
        }
    }

    /**
     * Finds the letters that are unique to 
     * the provided patterns with respect to 
     * the other patterns in the vector
     */
    fn find_unique_letters<'a>(patterns: &'a Vec<&Pattern>) -> Vec<(&'a Pattern, char)> {
        let mut uniques = Vec::new();
        for i in 0..patterns.len() {
            let pattern = patterns[i];
            let mut chars = pattern.chars.clone();
            for j in 0..patterns.len() {
                if i == j {
                    continue;
                }
                patterns[j].chars.iter()
                    .for_each(|c| { chars.remove(c); });
            }
            chars.into_iter().for_each(|c| {
                uniques.push((pattern, c));
            });
        }
        uniques
    }
    
    /**
     * Finds the letters that are unique to 
     * the provided patterns with respect to 
     * the other patterns in the vector
     */
    fn find_missing_letters<'a>(patterns: &'a Vec<&Pattern>) -> Vec<(&'a Pattern, char)> {
        let mut uniques = Vec::new();
        for i in 0..patterns.len() {
            let pattern = patterns[i];
            let mut chars = pattern.chars.clone();
            for j in 0..patterns.len() {
                if i == j {
                    continue;
                }
                patterns[j].chars.iter()
                    .for_each(|c| { chars.remove(c); });
            }
            chars.into_iter().for_each(|c| {
                uniques.push((pattern, c));
            });
        }
        uniques
    }

    /**
     * Asserts find_missing_letters() returns 1 
     * result for the provided patterns and returns it.
     */
    fn assert_one_and_get_unique<'a>(patterns: &'a Vec<&Pattern>) -> (&'a Pattern, char) {
        let uniques = Pattern::find_unique_letters(patterns);
        assert_eq!(1, uniques.len(), "{:?}", uniques);
        uniques[0]
    }

}

struct Test {
    input: Vec<Pattern>,
    output: Vec<String>
}

impl Test {
    fn parse(line: String) -> Test {
        let mut parts = line.split(" | ");
        let input = Test::parse_patterns(parts.next().unwrap());
        let output = Test::parse_patterns(parts.next().unwrap())
            .into_iter()
            .map(|pattern| pattern.pattern)
            .collect::<Vec<_>>();
        Test {
            input,
            output
        }
    }

    fn parse_patterns(line: &str) -> Vec<Pattern> {
        line.split_whitespace()
            .map(|pattern| Pattern::parse(pattern))
            .collect::<Vec<_>>()
    }

    fn determine_output(&self) -> i32 {
        let mut by_length = HashMap::<usize, Vec<&Pattern>>::new();
    
        self.input.iter().for_each(|pattern| {
            let len = pattern.chars.len();
            if let Some(patterns) = by_length.get_mut(&len) {
                patterns.push(pattern);
            } else {
                by_length.insert(len, vec![pattern]);
            }
        });

        let one = by_length.get(&2).unwrap()[0];
        let seven = by_length.get(&3).unwrap()[0];
        let four = by_length.get(&4).unwrap()[0];
        let eight = by_length.get(&7).unwrap()[0];

        let len_five = by_length.get(&5).unwrap();
        
        let five_uniques = Pattern::find_missing_letters(len_five);
        assert_eq!(2, five_uniques.len());
        
        let u1 = five_uniques[0].1;
        let u2 = five_uniques[1].1;

        let three = len_five.iter()
            .filter(|pattern| {
                !pattern.chars.contains(&u1) && 
                !pattern.chars.contains(&u2)
            })
            .next()
            .unwrap();

        let mut len_five = len_five.clone();
        len_five.push(four);
        let (two, e) = Pattern::assert_one_and_get_unique(&len_five);
        
        let (five, b) = match u1 == e {
            true => five_uniques[1],
            false => five_uniques[0]
        };

        let d = four.chars.iter()
            .filter(|c| !one.chars.contains(c) && **c != b)
            .map(|c| *c)
            .next().unwrap();
        
        let len_six = by_length.get(&6).unwrap();
        let mut six_nine = vec![];

        let mut zero: Option<&Pattern> = None;
        for pattern in len_six {
            if pattern.chars.contains(&d) {
                six_nine.push(*pattern);
            } else {
                zero = Some(pattern);
            }
        }
        let zero = zero.unwrap();

        let six_nine_uniques = Pattern::find_missing_letters(&six_nine);
        let (six, nine) = match six_nine_uniques[0].1 == e {
            true => (six_nine_uniques[0].0, six_nine_uniques[1].0),
            false => (six_nine_uniques[1].0, six_nine_uniques[0].0),
        };

        let mut pattern_to_number = HashMap::<&String, i32>::new();
        pattern_to_number.insert(&zero.pattern, 0);
        pattern_to_number.insert(&one.pattern, 1);
        pattern_to_number.insert(&two.pattern, 2);
        pattern_to_number.insert(&three.pattern, 3);
        pattern_to_number.insert(&four.pattern, 4);
        pattern_to_number.insert(&five.pattern, 5);
        pattern_to_number.insert(&six.pattern, 6);
        pattern_to_number.insert(&seven.pattern, 7);
        pattern_to_number.insert(&eight.pattern, 8);
        pattern_to_number.insert(&nine.pattern, 9);

        let output = self.output.iter()
            .map(|pattern| pattern_to_number.get(pattern).unwrap())
            .fold(0, |result, num| (result * 10) + num);
        output
    }
}

fn part_one(file_name: &str) {
    let mut counts = vec![0; 9];

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .flat_map(|line| Test::parse(line).output)
        .map(|pattern| pattern.len())
        .for_each(|len| {
            counts[len] += 1;
        });
    
    let total = counts[2] + 
        counts[3] + 
        counts[4] + 
        counts[7];
    
    println!("Part 1: {} + {} + {} + {} = {}", counts[2], counts[3], counts[4], counts[7], total);
}

fn part_two(file_name: &str) {
    let total = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Test::parse(line))
        .map(|test| test.determine_output())
        .sum::<i32>();
    
    println!("Part 2: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");
    part_two("sample.txt");

    println!("Done!");
}
