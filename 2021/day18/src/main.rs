use std::iter::Sum;
use std::ops::Add;
use std::iter::Peekable;
use std::str::Chars;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

enum Element {
    Number(u32),
    Pair(Box<Pair>)
}

enum Exploded {
    No(Element),
    Yes(u32, u32),
    Pass(u32, u32, Element)
}

impl Element {
    fn parse(line: &mut Peekable<Chars<'_>>) -> Element {
        match *line.peek().unwrap() {
            '[' => Element::pair(Pair::parse(line)),
            _ => Element::Number(line.next().unwrap().to_digit(10).unwrap() as u32)
        }
    }
    fn pair(pair: Pair) -> Element {
        Element::Pair(Box::new(pair))
    }
    fn try_explode(self, depth: usize) -> Exploded {
        if let Element::Pair(pair) = self {
            if depth > 4 {
                // must be a pair of numbers
                if let (Element::Number(left), Element::Number(right)) = (pair.left, pair.right) {
                    Exploded::Yes(left, right)
                } else {
                    panic!("Invalid state!");
                }
            } else {
                let (exploded, pair) = pair.try_explode(depth);
                if let Some((left_num, right_num)) = exploded {
                    Exploded::Pass(left_num, right_num, Element::pair(pair))
                } else {
                    Exploded::No(Element::pair(pair))
                }
            }
        } else {
            Exploded::No(self)
        }
    }
    fn distribute_left(&mut self, amount: &mut u32) {
        if *amount > 0 {
            match self {
                Element::Number(number) => {
                    *number += *amount;
                    *amount = 0;
                },
                Element::Pair(pair) => {
                    pair.right.distribute_left(amount);
                    pair.left.distribute_left(amount);
                }
            }
        }
    }
    fn distribute_right(&mut self, amount: &mut u32) {
        if *amount > 0 {
            match self {
                Element::Number(number) => {
                    *number += *amount;
                    *amount = 0;
                },
                Element::Pair(pair) => {
                    pair.left.distribute_right(amount);
                    pair.right.distribute_right(amount);
                }
            }
        }
    }
    fn get_magnitude(&self) -> u64 {
        match self {
            Element::Number(number) => *number as u64,
            Element::Pair(pair) => pair.get_magnitude()
        }
    }
    fn print(&self) {
        match self {
            Element::Number(number) => print!("{}", number),
            Element::Pair(pair) => pair.print_rec()
        }
    }
    fn copy(&self) -> Element {
        match self {
            Element::Number(number) => Element::Number(*number),
            Element::Pair(pair) => Element::pair(pair.copy())
        }
    }
}

struct Pair {
    left: Element,
    right: Element
}

impl Pair {
    fn parse(line: &mut Peekable<Chars<'_>>) -> Pair {
        assert_eq!('[', line.next().unwrap());
        let left = Element::parse(line);
        assert_eq!(',', line.next().unwrap());
        let right = Element::parse(line);
        assert_eq!(']', line.next().unwrap());

        Pair {
            left,
            right
        }
    }

    fn new(left: Element, right: Element) -> Pair {
        Pair { left, right }
    }

    fn reduce(self) -> Self {
        let mut next = self;
        loop {
            let (result, pair) = next.try_explode(1);
            next = pair;
            if result.is_none() && !next.try_split() {
                return next;
            }
        }
    }
    fn try_split(&mut self) -> bool {
        let mut new_pair = None;
        if let Element::Number(number) = self.left {
            if number >= 10 {
                let half = number / 2;
                let extra = number % 2;
                new_pair = Some(Element::pair(Pair::new(
                    Element::Number(half),
                    Element::Number(half + extra)
                )));
            }
        } else if let Element::Pair(pair) = &mut self.left {
            if pair.try_split() {
                return true;
            }
        }
        if let Some(new_left) = new_pair {
            self.left = new_left;
            return true;
        }
        
        if let Element::Number(number) = self.right {
            if number >= 10 {
                let half = number / 2;
                let extra = number % 2;
                new_pair = Some(Element::pair(Pair::new(
                    Element::Number(half),
                    Element::Number(half + extra)
                )))
            }
        } else if let Element::Pair(pair) = &mut self.right {
            if pair.try_split() {
                return true;
            }
        }
        if let Some(new_right) = new_pair {
            self.right = new_right;
            true
        } else {
            false
        }
    }
    fn try_explode(mut self, depth: usize) -> (Option<(u32, u32)>, Pair) {
        match self.left.try_explode(depth + 1) {
            Exploded::No(mut left) => match self.right.try_explode(depth + 1) {
                Exploded::No(right) => (None, Pair::new(left, right)),
                Exploded::Yes(mut left_num, right_num) => {
                    left.distribute_left(&mut left_num);
                    (Some((left_num, right_num)), Pair::new(left, Element::Number(0)))
                },
                Exploded::Pass(mut left_num, right_num, right) => {
                    left.distribute_left(&mut left_num);
                    (Some((left_num, right_num)), Pair::new(left, right))
                }
            },
            Exploded::Yes(left_num, mut right_num) => {
                self.right.distribute_right(&mut right_num);
                (Some((left_num, right_num)), Pair::new(Element::Number(0), self.right))
            },
            Exploded::Pass(left_num, mut right_num, left) => {
                self.right.distribute_right(&mut right_num);
                (Some((left_num, right_num)), Pair::new(left, self.right))
            }
        }
    }
    fn get_magnitude(&self) -> u64 {
        (self.left.get_magnitude() * 3) + (self.right.get_magnitude() * 2)
    }
    fn print(&self) {
        self.print_rec();
        println!("");
    }
    fn print_rec(&self) {
        print!("[");
        self.left.print();
        print!(",");
        self.right.print();
        print!("]");
    }
    fn copy(&self) -> Self {
        Pair {
            left: self.left.copy(),
            right: self.right.copy()
        }
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let added = Pair {
            left: Element::pair(self),
            right: Element::pair(other)
        };
        added.reduce()
    }
}

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn get_homework(file_name: &str) -> Vec<Pair> {
    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| Pair::parse(&mut line.chars().peekable()))
        .collect::<Vec<_>>()
}

fn part_one(file_name: &str) {
    let sum = get_homework(file_name)
        .into_iter()
        .reduce(|result, pair| {
            let result = result + pair;
            result
        })
        .unwrap();
    
    println!("Part 1: {}", sum.get_magnitude());
}

fn part_two(file_name: &str) {
    let homework_one = get_homework(file_name);
    let homework_two = get_homework(file_name);

    let mut max_magnitude = 0;

    for i in 0..homework_one.len() {
        for j in 0..homework_two.len() {
            if i == j {
                continue;
            }

            let sum = homework_one[i].copy() + homework_two[j].copy();
            let magnitude = sum.get_magnitude();
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }
    
    println!("Part 2: {}", max_magnitude);
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");
    part_two("input.txt");

    println!("Done!");
}
