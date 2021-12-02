use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::BTreeSet;
use std::ops::Bound::Excluded;

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).unwrap();

    let mut seen: BTreeSet<i32> = BTreeSet::new();

    for line in io::BufReader::new(file).lines() {
        let number = line.unwrap().parse::<i32>().unwrap();
        let diff = 2020 - number;
        let range = {
            if diff > number {
                (Excluded(number), Excluded(diff))
            } else {
                (Excluded(diff), Excluded(number))
            }
        };

        for check in seen.range(range) {
            let left = diff - check;
            if seen.contains(&left) {
                println!("{} * {} * {} = {}", number, check, left, (number * check * left));
                return;
            }
        }
        seen.insert(number);
    }

    println!("Done!");
}
