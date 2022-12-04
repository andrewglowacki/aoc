use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

#[derive(Debug)]
struct Range {
    lower: i32,
    upper: i32
}

#[derive(Debug)]
struct Pair {
    left: Range,
    right: Range
}

impl Range {
    fn from_string(line: &str) -> Range {
        let mut pieces = line.split('-');

        let lower = pieces.next()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let upper = pieces.next()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        Range {
            lower,
            upper
        }
    }
    fn completely_overlaps(&self, other: &Self) -> bool {
        match self.lower - other.lower {
            x if x < 0 => self.upper >= other.upper,
            x if x > 0 => self.upper <= other.upper,
            _ => self.upper <= other.upper || other.upper <= self.upper
        }
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.lower <= other.upper && self.upper >= other.lower
    }
}

impl Pair {
    fn from_string(line: String) -> Self {
        let mut pieces = line.split(',');
        
        let left = Range::from_string(pieces.next().unwrap());
        let right = Range::from_string(pieces.next().unwrap());

        Pair {
            left,
            right
        }
    }

    fn has_completely_overlapping(&self) -> bool {
        self.left.completely_overlaps(&self.right)
    }

    fn has_overlapping(&self) -> bool {
        self.left.overlaps(&self.right)
    }
}

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn part_one(file_name: &str) {
    let count = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(Pair::from_string)
        .filter(Pair::has_completely_overlapping)
        .count();
    
    println!("Part 1: {}", count);
}

fn part_two(file_name: &str) {
    let count = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(Pair::from_string)
        .filter(Pair::has_overlapping)
        .count();
    
    println!("Part 2: {}", count);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_outside() {
        let pair = Pair::from_string("5-6,7-8".to_owned());
        assert_eq!(false, pair.has_overlapping());
    }

    #[test]
    fn lower_partial_border() {
        let pair = Pair::from_string("5-6,6-8".to_owned());
        assert_eq!(true, pair.has_overlapping());
    }
    
    #[test]
    fn lower_partial_over() {
        let pair = Pair::from_string("5-7,6-8".to_owned());
        assert_eq!(true, pair.has_overlapping());
    }
    
    #[test]
    fn inside_lower() {
        let pair = Pair::from_string("6-7,6-8".to_owned());
        assert_eq!(true, pair.has_overlapping());
    }

    #[test]
    fn inside_upper() {
        let pair = Pair::from_string("7-8,6-8".to_owned());
        assert_eq!(true, pair.has_overlapping());
    }

    #[test]
    fn inside_fully() {
        let pair = Pair::from_string("7-7,6-8".to_owned());
        assert_eq!(true, pair.has_overlapping());
    }

    #[test]
    fn upper_outside() {
        let pair = Pair::from_string("9-11,7-8".to_owned());
        assert_eq!(false, pair.has_overlapping());
    }

    #[test]
    fn upper_partial_border() {
        let pair = Pair::from_string("8-10,6-8".to_owned());
        assert_eq!(true, pair.has_overlapping());
    }
    
    #[test]
    fn upper_partial_over() {
        let pair = Pair::from_string("7-10,6-8".to_owned());
        assert_eq!(true, pair.has_overlapping());
    }
    
    #[test]
    fn inside_not_touching() {
        let pair = Pair::from_string("7-10,8-9".to_owned());
        assert_eq!(true, pair.has_overlapping());
    }
    
    #[test]
    fn rev_inside_lower() {
        let pair = Pair::from_string("7-10,7-9".to_owned());
        assert_eq!(true, pair.has_overlapping());
    }
    
    #[test]
    fn rev_inside_upper() {
        let pair = Pair::from_string("7-10,8-10".to_owned());
        assert_eq!(true, pair.has_overlapping());
    }
    
    #[test]
    fn rev_inside_full() {
        let pair = Pair::from_string("7-10,8-9".to_owned());
        assert_eq!(true, pair.has_overlapping());
    }
    
    #[test]
    fn rev_inside_one() {
        let pair = Pair::from_string("7-10,8-8".to_owned());
        assert_eq!(true, pair.has_overlapping());
    }

}