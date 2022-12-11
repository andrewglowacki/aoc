use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};
use std::slice::Iter;

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Monkey {
    items: Vec<u64>,
    mult_old: bool,
    mult_op: u64,
    add_op: u64,
    div_amount: u64,
    division_check: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize
}

fn last_number(line: &String) -> u64 {
    let pos = line.rfind(" ").unwrap();
    line[pos + 1..].parse::<u64>().unwrap()
}

impl Monkey {
    fn from_lines(lines: &mut Iter<String>, div_amount: u64) -> Monkey {
        let items = lines.next().unwrap();
        let items = items[items.find(":").unwrap() + 1..]
            .split(",")
            .map(|piece| piece.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let op_line = lines.next().unwrap();
        let mult_old = op_line.ends_with("old");

        let (mult_op, add_op) = match mult_old {
            true => (1, 0),
            false => match op_line.contains("+") {
                true => (1, last_number(op_line)),
                false => (last_number(op_line), 0)
            }
        };

        let division_check = last_number(lines.next().unwrap());
        let true_monkey = last_number(lines.next().unwrap()) as usize;
        let false_monkey = last_number(lines.next().unwrap()) as usize;

        Monkey {
            items,
            mult_old,
            mult_op,
            add_op,
            div_amount,
            division_check,
            true_monkey,
            false_monkey,
            inspections: 0
        }
    }

    fn new_worry(&self, item: u64) -> u64 {
        match self.mult_old {
            true => (item * item) / self.div_amount,
            false => ((item * self.mult_op) + self.add_op) / self.div_amount
        }
    }

    fn inspect_and_throw(&mut self) -> Vec<(usize, u64)> {
        let thrown = self.items.iter()
            .map(|item| self.new_worry(*item))
            .map(|item| match item % self.division_check == 0 {
                true => (self.true_monkey, item),
                false => (self.false_monkey, item)
            })
            .collect::<Vec<_>>();
        self.items.clear();
        self.inspections += thrown.len();
        thrown
    }

}

struct Monkeys {
    monkeys: Vec<Monkey>
}

impl Monkeys {
    fn from_lines(file_name: &str, div_amount: u64) -> Monkeys {
        let lines = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();
        let mut lines = lines.iter();

        let mut monkeys = Vec::new();

        while let Some(_) = lines.next() {
            monkeys.push(Monkey::from_lines(&mut lines, div_amount));
        }

        Monkeys { monkeys }
    }

    fn play_round(&mut self) {
        let count = self.monkeys.len();
        for i in 0..count {
            let thrown = self.monkeys[i].inspect_and_throw();
            thrown.into_iter().for_each(|(monkey, worry)| {
                self.monkeys[monkey].items.push(worry);
            });
        }
    }
}

fn part_one(file_name: &str) {
    let mut monkeys = Monkeys::from_lines(file_name, 3);

    for _ in 0..20 {
        monkeys.play_round();
    }

    let mut inspections = monkeys.monkeys.iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();

    let top_product = inspections[0] * inspections[1];
    
    println!("Part 1: {}", top_product);
}

fn part_two(file_name: &str) {
    let mut monkeys = Monkeys::from_lines(file_name, 1);

    for _ in 0..10000 {
        monkeys.play_round();
    }

    let mut inspections = monkeys.monkeys.iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();

    let top_product = inspections[0] * inspections[1];
    
    println!("Part 2: {}", top_product);
}

fn main() {
    part_one("input.txt");
    part_two("sample.txt");

    println!("Done!");
}
