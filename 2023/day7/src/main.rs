use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

const HIGH: usize = 0;
const PAIR: usize = 1;
const TWO_PAIR: usize = 2;
const THREE_OF_A_KIND: usize = 3;
const FULL_HOUSE: usize = 4;
const FOUR_OF_A_KIND: usize = 5;
const FIVE_OF_A_KIND: usize = 6;

#[derive(PartialEq, Eq)]
struct Hand {
    cards: Vec<usize>,
    hand_type: usize,
    bid: u64
}

impl Hand {

    fn get_hand_type_one(cards: &Vec<usize>) -> usize {
        let mut counts = vec![0; 15];
        let mut common_counts: Vec<usize> = vec![0; 6];

        for card in cards {
            let cur_count = counts[*card];
            let new_count = cur_count + 1;
            counts[*card] = new_count;

            if cur_count > 0 {
                common_counts[cur_count] -= 1;
            }
            common_counts[new_count] += 1;
        }

        let key = 
            common_counts[1] + 
            (common_counts[2] << 8) + 
            (common_counts[3] << 10) +
            (common_counts[4] << 15) +
            (common_counts[5] << 20);
        
        match key {
            1048576 => FIVE_OF_A_KIND,
            32769 => FOUR_OF_A_KIND,
            1280 => FULL_HOUSE,
            1026 => THREE_OF_A_KIND,
            513 => TWO_PAIR,
            259 => PAIR,
            5 => HIGH,
            x => panic!("Unexpected key: {} from {:?} and {:?}", x, cards, common_counts)
        }
    }

    fn get_hand_type_two(cards: &Vec<usize>) -> usize {
        let mut counts = vec![0; 15];
        let mut common_counts: Vec<usize> = vec![0; 6];

        for card in cards {
            // don't include jacks
            if *card == 1 {
                continue;
            }

            let cur_count = counts[*card];
            let new_count = cur_count + 1;
            counts[*card] = new_count;

            if cur_count > 0 {
                common_counts[cur_count] -= 1;
            }
            common_counts[new_count] += 1;
        }

        let key = 
            common_counts[1] + 
            (common_counts[2] << 8) + 
            (common_counts[3] << 10) +
            (common_counts[4] << 15) +
            (common_counts[5] << 20);
        
        match key {
            1048576 => FIVE_OF_A_KIND,
            32769 => FOUR_OF_A_KIND,
            32768 => FIVE_OF_A_KIND, // J + FOUR_OF_A_KIND
            1280 => FULL_HOUSE,
            1026 => THREE_OF_A_KIND,
            1025 => FOUR_OF_A_KIND, // J + THREE_OF_A_KIND
            1024 => FIVE_OF_A_KIND, // J + J + FOUR_OF_A_KIND
            513 => TWO_PAIR,
            512 => FULL_HOUSE, // J + TWO_PAIR 
            259 => PAIR,
            258 => THREE_OF_A_KIND,  // J + PAIR
            257 => FOUR_OF_A_KIND, // J + J + PAIR
            256 => FIVE_OF_A_KIND, // J + J + J + PAIR
            5 => HIGH,
            4 => PAIR, // J + x
            3 => THREE_OF_A_KIND, // J + J + x
            2 => FOUR_OF_A_KIND, // J + J + J + x
            1 | 0 => FIVE_OF_A_KIND, // J + J + J + J + x
            x => panic!("Unexpected key: {} from {:?} and {:?}", x, cards, common_counts)
        }
    }

    fn parse<F>(
        line: String, 
        jack_value: u32,
        get_hand_type: F) -> Hand 
        where F: Fn(&Vec<usize>) -> usize
    {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let cards = parts[0].chars().map(|c| {
            let number = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => jack_value,
                'T' => 10,
                x => x.to_digit(10).unwrap()
            };
            number as usize
        })
        .collect::<Vec<_>>();
        let hand_type = get_hand_type(&cards);
        let bid = parts[1].parse::<u64>().unwrap();

        Hand { bid, cards, hand_type }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_compare = self.hand_type.partial_cmp(&other.hand_type).unwrap();
        if  hand_compare == Ordering::Equal {
            self.cards.partial_cmp(&other.cards)
        } else {
            Some(hand_compare)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_compare = self.hand_type.cmp(&other.hand_type);
        if  hand_compare == Ordering::Equal {
            self.cards.cmp(&other.cards)
        } else {
            hand_compare
        }
    }
}

fn compute_winnings(hands: &Vec<Hand>) -> u64 {
    (0..hands.len()).into_iter()
        .map(|i| (i + 1) as u64 * hands[i].bid)
        .sum::<u64>()
}

fn part_one(file_name: &str) {
    let mut hands = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Hand::parse(line, 11, Hand::get_hand_type_one))
        .collect::<Vec<_>>();

    hands.sort();

    let winnings = compute_winnings(&hands);
    
    println!("Part 1: {}", winnings);
}

fn part_two(file_name: &str) {
    let mut hands = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Hand::parse(line, 1, Hand::get_hand_type_two))
        .collect::<Vec<_>>();
    
    hands.sort();

    let winnings = compute_winnings(&hands);
    
    println!("Part 2: {}", winnings);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
