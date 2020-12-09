use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::LinkedList;
use std::collections::BTreeSet;

fn read_num(line: String) -> u32 {
    line.parse::<u32>().unwrap()
}

fn does_sum_exist(avail_numbers: &BTreeSet<u32>, num: u32) -> bool {
    let mut front = avail_numbers.iter().peekable();
    let mut back = avail_numbers.iter().rev().peekable();

    while let (Some(smaller), Some(bigger)) = (front.peek(), back.peek()) {
        if *smaller >= *bigger {
            return false;
        }
        
        match (*smaller + *bigger) as i32 - num as i32 {
            x if x < 0 => front.next(),
            x if x > 0 => back.next(),
            _ => return true
        };
    }
    false
}

const PREAMBLE_LEN: usize = 25;

fn read_preamble(lines: &mut Lines<BufReader<File>>) -> LinkedList<u32> {
    lines.take(PREAMBLE_LEN)
        .map(|line| read_num(line.unwrap()))
        .collect::<LinkedList<u32>>()
}

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn main() {
    let mut lines = get_file_lines("input.txt");

    let mut number_queue = read_preamble(&mut lines);
    
    let mut avail_numbers = number_queue.iter()
        .map(|num| *num)
        .collect::<BTreeSet<u32>>();

    // part 1
    let invalid_number = lines.map(|line| read_num(line.unwrap()))
        .find(|num| {
            let exists = does_sum_exist(&avail_numbers, *num);
            let first_num = number_queue.pop_front().unwrap();
            avail_numbers.remove(&first_num);
            number_queue.push_back(*num);
            avail_numbers.insert(*num);
            !exists
        })
        .unwrap();
    
    println!("First number with no sum is: {}", invalid_number);

    // part 2
    let mut lines = get_file_lines("input.txt");
    let mut number_queue = LinkedList::<u32>::new();
    let mut avail_numbers = BTreeSet::<u32>::new();
    let mut current_sum = 0;
    
    while let Some(Ok(line)) = lines.next() {
        let num = read_num(line);
        current_sum += num;
        number_queue.push_back(num);
        avail_numbers.insert(num);
        if current_sum > invalid_number {
            while current_sum > invalid_number {
                let removed = number_queue.pop_front().unwrap();
                current_sum -= removed;
                avail_numbers.remove(&removed);
            }
        }
        if current_sum == invalid_number {
            break;
        }
    }

    let smallest = avail_numbers.iter().nth(0).unwrap();
    let biggest = avail_numbers.iter().rev().nth(0).unwrap();
    let weakness = smallest + biggest;
    println!("Found weakness! {} + {} = {}, (current sum: {})", smallest, biggest, weakness, current_sum);
}
