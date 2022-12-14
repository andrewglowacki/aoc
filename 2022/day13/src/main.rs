use std::cmp::Ordering;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

enum Packet {
    NUMBER(u32),
    LIST(Vec<Packet>)
}

#[derive(PartialEq)]
enum Status {
    Ordered,
    Unknown,
    Unordered
}

impl Packet {
    fn parse_line(line: &String) -> Packet {
        let line = line.chars().collect::<Vec<_>>();
        let mut line = line.iter();

        line.next().unwrap(); // skip start

        let mut list_stack = Vec::<Vec<Packet>>::new();
        let mut current_list = Vec::new();
        let mut current_number = String::new();

        while let Some(c) = line.next() {
            match c {
                ']' => {
                    if current_number.len() > 0 {
                        let number = current_number.parse::<u32>().unwrap();
                        current_list.push(Packet::NUMBER(number));
                        current_number.clear();
                    }
                    if !list_stack.is_empty() {
                        let mut new_current = list_stack.pop().unwrap();
                        new_current.push(Packet::LIST(current_list));
                        current_list = new_current;
                    }
                },
                '[' => {
                    list_stack.push(current_list);
                    current_list = Vec::new();
                },
                ',' => match current_number.len() == 0 {
                    true => continue,
                    false => {
                        let number = current_number.parse::<u32>().unwrap();
                        current_list.push(Packet::NUMBER(number));
                        current_number.clear();
                    }
                },
                _ => {
                    current_number.push(*c);
                }
            }
        }
        Packet::LIST(current_list)
    }

    fn compare_numbers(left: u32, right: u32) -> Status {
        match left as i32 - right as i32 {
            x if x < 0 => Status::Ordered,
            x if x > 0 => Status::Unordered,
            _ => Status::Unknown
        }
    }

    fn get_status(&self, other: &Self) -> Status {
        match (self, other) {
            (Packet::NUMBER(left), Packet::NUMBER(right)) => { 
                Packet::compare_numbers(*left, *right)
            },
            (Packet::LIST(left), Packet::LIST(right)) => {
                let compare_len = left.len().min(right.len());
                for i in 0..compare_len {
                    let left = &left[i];
                    let right = &right[i];
                    let status = left.get_status(right);
                    if status != Status::Unknown {
                        return status;
                    }
                }
                Packet::compare_numbers(left.len() as u32, right.len() as u32)
            },
            (Packet::NUMBER(left), Packet::LIST(_)) => {
                let new_left = vec![Packet::NUMBER(*left)];
                let new_left = Packet::LIST(new_left);
                new_left.get_status(other)
            },
            (Packet::LIST(_), Packet::NUMBER(right)) => {
                let new_right = vec![Packet::NUMBER(*right)];
                let new_right = Packet::LIST(new_right);
                self.get_status(&new_right)
            }
        }
    }

    fn to_string(&self, str: &mut String) {
        match self {
            Packet::NUMBER(i) => {
                *str += i.to_string().as_str();
            },
            Packet::LIST(list) => {
                *str += "[";
                let mut first = true;
                for item in list {
                    if first {
                        first = false;
                    } else {
                        *str += ",";
                    }
                    item.to_string(str);
                }
                *str += "]";
            }
        }
    }

}

struct Pair {
    left: Packet,
    right: Packet,
    index: usize
}

impl Pair {
    fn from_lines(index: usize, left: &String, right: &String) -> Pair {
        Pair {
            index,
            left: Packet::parse_line(left),
            right: Packet::parse_line(right)
        }
    }
    fn are_packets_in_order(&self) -> bool {
        match self.left.get_status(&self.right) {
            Status::Ordered => return true,
            Status::Unordered => return false,
            Status::Unknown => panic!("Left and right are the same!")
        }
    }
}

fn read_pairs(file_name: &str) -> Vec<Pair> {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let mut lines = lines.iter();
    
    let mut pairs = Vec::new();
    while let (Some(left), Some(right)) = (lines.next(), lines.next()) {
        let index = pairs.len() + 1;
        pairs.push(Pair::from_lines(index, left, right));
    }
    pairs
}

fn part_one(file_name: &str) {
    let pairs = read_pairs(file_name);

    let total: usize = pairs.iter()
        .filter(|pair| pair.are_packets_in_order())
        .map(|pair| pair.index)
        .sum();

    println!("Part 1: {}", total);
}

fn part_two(file_name: &str) {

    let mut all_packets = Vec::new();
    all_packets.push(Packet::LIST(vec![Packet::LIST(vec![Packet::NUMBER(2)])]));
    all_packets.push(Packet::LIST(vec![Packet::LIST(vec![Packet::NUMBER(6)])]));

    let pairs = read_pairs(file_name);
    pairs.into_iter().for_each(|pair| {
        all_packets.push(pair.left);
        all_packets.push(pair.right);
    });

    all_packets.sort_by(|a, b| match a.get_status(b) {
        Status::Ordered => Ordering::Less,
        Status::Unknown => Ordering::Equal,
        Status::Unordered => Ordering::Greater
    });

    let mut key = 1;
    let mut check = String::new();
    for i in 0..all_packets.len() {
        all_packets[i].to_string(&mut check);
        match check.as_str() {
            "[[2]]" | "[[6]]" => key *= i + 1,
            _ => ()
        };
        check.clear();
    }

    println!("Part 2: {}", key);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
