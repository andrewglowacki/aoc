use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::HashMap;

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn test_input(file_name: &str) {
    let mut lines = get_file_lines(file_name);

    while let Some(Ok(line)) = lines.next() {
        let initial_numbers = line.split(",")
            .flat_map(|number_str| number_str.parse::<u32>())
            .collect::<Vec<u32>>();
        
        let mut last_by_number = HashMap::<u32, usize>::new();
        for i in 0..(initial_numbers.len() - 1) {
            last_by_number.insert(initial_numbers[i], i + 1);
        }

        let mut last = initial_numbers[initial_numbers.len() - 1];
        for turn in (initial_numbers.len() + 1)..30000001 {
            let speak_now = last_by_number.insert(last, turn - 1)
                .map(|last_turn| (turn - 1) - last_turn)
                .unwrap_or(0) as u32;
            last = speak_now;
        }
        
        println!("For {}, and line: {}, last spoken is: {}", file_name, line, last);
    }

}

fn main() {
    test_input("sample.txt");
    test_input("other_sample.txt");
    test_input("input.txt");
}
