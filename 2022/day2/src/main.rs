use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn determine_score_pt1(left: char, right: char) -> i32 {
    // normalize the two symbols
    // A, X => Rock     => 1
    // B, Y => Paper    => 2
    // C, Z => Scissors => 3

    let left = ('A' as i32 - left as i32).abs() + 1;
    let right = ('X' as i32 - right as i32).abs() + 1;
    
    // to determine the outcome:
    // 
    // Rock - Rock         => 1 - 1 =  0 => 3
    // Rock - Paper        => 1 - 2 = -1 => 6
    // Rock - Scissors     => 1 - 3 = -2 => 0
    // 
    // Paper - Rock        => 2 - 1 =  1 => 0
    // Paper - Paper       => 2 - 2 =  0 => 3
    // Paper - Scissors    => 2 - 3 = -1 => 6
    // 
    // Scissors - Rock     => 3 - 1 =  2 => 6
    // Scissors - Paper    => 3 - 2 =  1 => 0
    // Scissors - Scissors => 3 - 3 =  0 => 3
    
    let outcome_score = match left - right {
        0 => 3,
        -1 | 2 => 6,
        _ => 0
    };
    
    right + outcome_score
}


fn determine_score_pt2(left: char, right: char) -> i32 {
    // map left to index
    // A => Rock     => 0
    // B => Paper    => 1
    // C => Scissors => 2

    let index = ('A' as i32 - left as i32).abs();

    // map right to outcome
    // X => Lose => 0
    // Y => Draw => 1
    // Z => Win  => 2
    let outcome = ('X' as i32 - right as i32).abs();
    let outcome_score = outcome * 3;

    //                    C
    // to lose = (index + 2) % 3
    // to draw = (index + 0) % 3
    // to win  = (index + 1) % 3

    let c = (outcome + 2) % 3;
    let play_score = ((index + c) % 3) + 1;
    
    outcome_score + play_score
}

fn part_one(file_name: &str) {
    let total = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|chars| determine_score_pt1(chars[0], chars[2]))
        .sum::<i32>();
    
    println!("Part 1: {}", total);
}

fn part_two(file_name: &str) {
    let total = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|chars| determine_score_pt2(chars[0], chars[2]))
        .sum::<i32>();
    
    println!("Part 2: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
