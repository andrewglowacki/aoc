use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::{HashSet, LinkedList};

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn read_deck(lines: &mut Lines<BufReader<File>>) -> LinkedList<usize> {
    lines.next(); // skip player label
    lines.flat_map(|line| line.ok())
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<LinkedList<usize>>()
}

#[derive(Debug)]
enum Winner {
    One,
    Two
}

fn play_recursive_combat_round(
    p1_card: usize, 
    p2_card: usize, 
    player1: &LinkedList<usize>, 
    player2: &LinkedList<usize>) 
    -> Winner {

    if player1.len() >= p1_card && player2.len() >= p2_card {
        let new_player1 = player1.iter()
            .take(p1_card).cloned()
            .collect::<LinkedList<usize>>();
        let new_player2 = player2.iter()
            .take(p2_card).cloned()
            .collect::<LinkedList<usize>>();
        recursive_combat(new_player1, new_player2)
    } else {
        match p1_card > p2_card {
            true => Winner::One,
            false => Winner::Two
        }
    }
}

fn join(list: &LinkedList<usize>) -> String {
    list.iter().fold(String::new(), |acc, item| {
        acc + format!("{},", item).as_str()
    })
}

fn recursive_combat(player1: LinkedList<usize>, player2: LinkedList<usize>) -> Winner {
    let mut player1 = player1;
    let mut player2 = player2;

    let mut seen_configurations = HashSet::<(String, String)>::new();

    seen_configurations.insert((join(&player1), join(&player2)));

    while let (Some(p1_card), Some(p2_card)) = (player1.pop_front(), player2.pop_front()) {
        let (winner, card1, card2) = match play_recursive_combat_round(p1_card, p2_card, &player1, &player2) {
            Winner::One => (&mut player1, p1_card, p2_card),
            Winner::Two => (&mut player2, p2_card, p1_card)
        };
        winner.push_back(card1);
        winner.push_back(card2);

        if !seen_configurations.insert((join(&player1), join(&player2))) {
            return Winner::One;
        }
    }

    match player1.is_empty() {
        true => Winner::Two,
        false => Winner::One
    }
}

fn test_input(file_name: &str) {
    let mut lines = get_file_lines(file_name);

    let mut player1 = read_deck(&mut lines);
    let mut player2 = read_deck(&mut lines);

    while let Some(p1_card) = player1.pop_front() {
        if let Some(p2_card) = player2.pop_front() {
            let (winner, card1, card2) = match play_recursive_combat_round(p1_card, p2_card, &player1, &player2) {
                Winner::One => (&mut player1, p1_card, p2_card),
                Winner::Two => (&mut player2, p2_card, p1_card)
            };
            winner.push_back(card1);
            winner.push_back(card2);
        } else {
            player1.push_front(p1_card);
            break;
        }
    }

    let winner = match player1.is_empty() {
        true => player2,
        false => player1
    };

    let score = (0..winner.len()).zip(winner.iter().rev())
        .map(|(mult, card)| (mult + 1) * card)
        .sum::<usize>();
    
    println!("For {}, score is: {}", file_name, score);
}

fn main() {
    test_input("sample.txt");
    test_input("input.txt");
}
