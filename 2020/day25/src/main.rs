use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

const SUBJECT: u64 = 7;

fn find_loop_count(pub_key: u64) -> usize {
    let mut value = 1;
    let mut loops = 0;
    while value != pub_key {
        value = value * SUBJECT;
        value = value % 20201227;
        loops += 1;
    }
    loops
}
fn run_loop(pub_key: u64, times: usize) -> u64 {
    let mut value = 1;
    for _ in 0..times {
        value = value * pub_key;
        value = value % 20201227;
    }
    value
}

fn test_input(file_name: &str) {
    let mut lines = get_file_lines(file_name);
    let pub_key_one = lines.next().unwrap().unwrap().parse::<u64>().unwrap();
    let pub_key_two = lines.next().unwrap().unwrap().parse::<u64>().unwrap();

    let key_one_loop_size = find_loop_count(pub_key_one);
    let encryption_key = run_loop(pub_key_two, key_one_loop_size);
    println!("For {}, encryption key is {}", file_name, encryption_key);
}

fn main() {
    test_input("sample.txt");
    test_input("input.txt");
}
