use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::max;

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).unwrap();

    let mut seat_ids: Vec<u32> = BufReader::new(file).lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|code| get_seat_id(&code))
        .collect();

    seat_ids.sort_unstable();
    
    let max = seat_ids.last().unwrap();
    println!("Max Seat ID: {}", max);

    let mut last: u32 = *seat_ids.first().unwrap();
    let my_seat = 1 + seat_ids.into_iter()
        .take_while(|seat| {
            let found = seat - last != 2;
            last = *seat;
            found
        })
        .last()
        .unwrap();

    println!("My Seat ID: {}", my_seat);
}

fn compute_position(max_pos: u32, upper_char: char, code: &Vec<char>, code_start: usize, code_end: usize) -> u32 {
    (code_start..code_end)
        .map(|index| (index - code_start, code.get(index).unwrap()))
        .fold(0, |offset, item| {
            let (index, direction) = item;
            let group_size = max(1, max_pos >> (index + 1));
            if *direction == upper_char {
                offset + group_size
            } else {
                offset
            }
        })
}

fn get_seat_id(code: &Vec<char>) -> u32 {
    let row = compute_position(128, 'B', code, 0, 7);
    let col = compute_position(8, 'R', code, 7, 10);
    let seat_id = (row * 8) + col;
    seat_id
}