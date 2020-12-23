use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::BTreeMap;


fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn move_cups(current: i32, cups: &mut BTreeMap<i32, i32>) -> i32 {
    let one = cups.remove(&current).unwrap();
    let two = cups.remove(&one).unwrap();
    let three = cups.remove(&two).unwrap();
    let cur_next = cups.remove(&three).unwrap();

    cups.insert(current, cur_next);

    let (dest_cup, three_next) = if let Some(smaller) =  cups.range(0..current).last() {
        smaller
    } else {
        cups.iter().last().unwrap()
    };
    let dest_cup = *dest_cup;
    let three_next = *three_next;
    
    cups.insert(dest_cup, one);
    cups.insert(one, two);
    cups.insert(two, three);
    cups.insert(three, three_next);

    cur_next
}

fn _get_final_order(file_name: &str, cups: &BTreeMap<i32, i32>) {
    let mut final_order = String::new();
    let mut next = 1;
    loop {
        let current = *cups.get(&next).unwrap();
        if current == 1 {
            break;
        }
        final_order.push_str(current.to_string().as_str());
        next = current;
    }

    println!("For {}, final order is: {}", file_name, final_order);
}

fn test_input(file_name: &str) {
    let cups_list = get_file_lines(file_name)
        .next().unwrap().unwrap()
        .chars()
        .map(|c| String::from(c).parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut cups = BTreeMap::new();
    for i in 0..(cups_list.len() - 1) {
        cups.insert(cups_list[i], cups_list[i + 1]);
    }
    cups.insert(cups_list[cups_list.len() - 1], (cups_list.len() + 1) as i32);


    let total: i32 = 1000000;
    for i in (cups_list.len() + 1)..(total as usize) {
        let i = i as i32;
        cups.insert(i, i + 1);
    }

    cups.insert(total, cups_list[0]);
    
    let moves = 10000000;
    let mut current = cups_list[0];
    for _ in 0..moves {
        current = move_cups(current, &mut cups);
    }

    let one_next = cups.get(&1).unwrap();
    let two_next = cups.get(&one_next).unwrap();
    let result = *one_next as i64 * *two_next as i64;

    println!("For {}, the {} * {} = {}", file_name, one_next, two_next, result);
}

fn main() {
    test_input("sample.txt");
    test_input("input.txt");
}
