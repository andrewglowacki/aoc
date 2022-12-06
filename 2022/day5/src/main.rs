use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Yard {
    columns: Vec<Vec<char>>
}

impl Yard {
    fn from_lines(mut lines: Vec<String>) -> Yard {
        let column_count = lines.pop()
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10))
            .count();
            
        let mut columns: Vec<Vec<char>> = vec![Vec::new(); column_count];

        lines.iter().rev().for_each(|row| {
            let mut column = 0;
            let mut blank_count = 0;
            
            row.split(" ")
                .map(|piece| match piece.len() == 3 {
                    true => piece.chars().skip(1).next().unwrap(),
                    false => ' '
                })
                .for_each(|item| {
                    if item != ' ' {
                        columns[column].push(item);
                        blank_count = 0;
                        column += 1;
                    } else {
                        if blank_count == 0 {
                            column += 1;
                        }
                        blank_count = (blank_count + 1) % 4;
                    }
                });
        });

        Yard {
            columns
        }
    }

    fn perform_move(&mut self, move_string: String, advanced: bool) {
        let pieces = move_string.split(' ').collect::<Vec<_>>();
        let count = pieces[1].parse::<usize>().unwrap();
        let from = pieces[3].parse::<usize>().unwrap() - 1;
        let to = pieces[5].parse::<usize>().unwrap() - 1;

        let items = (0..count)
            .flat_map(|_| self.columns[from].pop())
            .collect::<Vec<_>>();

        let to = &mut self.columns[to];

        match advanced {
            true => items.into_iter().rev().for_each(|item| to.push(item)),
            false => items.into_iter().for_each(|item| to.push(item))
        };
    }

    fn _print(&self) {
        self.columns.iter()
            .for_each(|column| println!("{:?}", column))
    }

    fn get_message(&self) -> String {
        self.columns.iter()
            .map(|column| column.last().unwrap())
            .collect::<String>()
    }

}

fn solve(file_name: &str, part: usize) {
    let advanced = part != 1;

    let mut lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());

    let mut yard = Yard::from_lines((&mut lines)
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>());

    lines.for_each(|line| yard.perform_move(line, advanced));
    
    let message = yard.get_message();

    println!("Part {}: {}", part, message);
}

fn main() {
    solve("input.txt", 1);
    solve("input.txt", 2);

    println!("Done!");
}
