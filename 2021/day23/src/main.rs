use std::ops::Range;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

const HALL_SIZE: usize = 7;
const ROOMS: usize = 4;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert
}

impl Amphipod {
    fn from_char(c: char) -> Amphipod {
        match c {
            'A' => Amber,
            'B' => Bronze,
            'C' => Copper,
            'D' => Desert,
            _ => panic!("Invalid amphipod character: {}", c)
        }
    }
    fn from_index(index: usize) -> Amphipod {
        match index {
            0 => Amber,
            1 => Bronze,
            2 => Copper,
            3 => Desert,
            _ => panic!("Invalid amphipod index: {}", index)
        }
    }
    fn index(&self) -> usize {
        match self {
            Amber => 0,
            Bronze => 1,
            Copper => 2,
            Desert => 3
        }
    }
    fn cost(&self) -> usize {
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000
        }
    }
    fn to_char(&self) -> char {
        match self {
            Amber => 'A',
            Bronze => 'B',
            Copper => 'C',
            Desert => 'D'
        }
    }
}

use Amphipod::*;

struct Room {
    owner: Amphipod,
    wrong: Vec<Amphipod>,
    correct: usize
}

impl Room {
    fn new(owner: Amphipod) -> Room {
        Room {
            owner,
            wrong: Vec::new(),
            correct: 0,
        }
    }
    fn add(&mut self) {
        if self.wrong.len() > 0 {
            panic!("There are still wrong amphipods in this room: {:?}", self.wrong);
        } else {
            self.correct += 1;
        }
    }
    fn remove(&mut self) {
        self.correct -= 1;
    }
    fn is_empty(&self) -> bool {
        self.wrong.is_empty()
    }
    fn move_bottom_to_correct(&mut self) {
        while self.wrong.len() > 0 && self.wrong[0] == self.owner {
            self.wrong.remove(0);
            self.correct += 1;
        }
    }
    fn remove_next(&mut self) {
        self.wrong.pop().unwrap();
    }
    fn get_next(&self) -> Amphipod {
        self.wrong.last().unwrap().clone()
    }
}

struct Burrow {
    rooms: Vec<Room>,
    hallway: Vec<Option<Amphipod>>,
    in_hallway: Vec<usize>,
    lowest_cost: usize,
    cost: usize,
    rows: usize
}

impl Burrow {
    fn parse(file_name: &str) -> Burrow {
        let mut lines = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .skip(2)
            .take(2)
            .collect::<Vec<_>>();
        
        lines.iter_mut().for_each(|line| {
            line.retain(|c| c != '#' && c != ' ');
        });

        let lines = lines.iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(2, lines.len());
        assert_eq!(4, lines[0].len());
        assert_eq!(4, lines[1].len());

        let rooms = (0..4).map(|i| {
                let mut room = Room::new(Amphipod::from_index(i));
                room.wrong.push(Amphipod::from_char(lines[1][i]));
                room.wrong.push(Amphipod::from_char(lines[0][i]));
                room
            })
            .collect::<Vec<_>>();

        let hallway = (0..HALL_SIZE).map(|_| None)
            .collect::<Vec<_>>();
        
        let in_hallway = (0..4).map(|_| 0)
            .collect::<Vec<_>>();

        Burrow {
            rooms, 
            hallway,
            in_hallway,
            lowest_cost: usize::MAX,
            cost: 0,
            rows: 2
        }
    }

    fn move_bottom_to_correct(&mut self) {
        self.rooms.iter_mut().for_each(|room| {
            room.move_bottom_to_correct();
        });
    }

    fn find_next_in_hall(&self, from: usize, forward: bool) -> Option<usize> {
        match forward {
            true => (from..HALL_SIZE)
                .find(|i| self.hallway[*i].is_some()),
            false => (0..from + 1)
                .map(|i| from - i)
                .find(|i| self.hallway[*i].is_some())
        }
    }

    fn move_to_room(&mut self) -> Vec<(i32, Amphipod, usize)> {
        let mut moved = Vec::new();
        let mut had_move = true;
        while had_move {
            had_move = false;
            for i in 0..ROOMS {
                if self.rooms[i].is_empty() {
                    let owner = self.rooms[i].owner.clone();
                    let per_move_cost = owner.cost();
                    if let Some(from_index) = (0..4).filter(|j| i != *j)
                        .find(|j| {
                            let room = &self.rooms[*j];
                            !room.is_empty() && room.get_next() == owner
                        })
                    {
                        // move directly from one room to the other
                        
                        let from_moves = self.rows - (self.rooms[from_index].wrong.len() + self.rooms[from_index].correct);
                        let to_moves = (self.rows - self.rooms[i].correct) + 1;
                        let base_cost = (from_moves + to_moves) * per_move_cost;
                        let hall_cost = calc_cost((from_index + 1) * 2, (i + 1) * 2, per_move_cost);
                        let cost = base_cost + hall_cost;

                        self.cost += cost;
                        self.rooms[from_index].remove_next();
                        self.rooms[i].add();
                        moved.push((-(from_index as i32) - 1, owner.clone(), cost));
                        had_move = true;
                        break;
                    }
                    else if self.in_hallway[i] > 0
                    {
                        let to_moves = (self.rows - self.rooms[i].correct) + 1;
                        let base_cost = to_moves * per_move_cost;

                        let left_hall_index = i + 2;
                        if let Some(index) = self.find_next_in_hall(left_hall_index, false) {
                            if *self.hallway[index].as_ref().unwrap() == owner {
                                self.hallway[index] = None;
                                let cost = base_cost + calc_cost(index, (i + 1) * 2, per_move_cost);
                                self.cost += cost;
                                self.rooms[i].add();
                                self.in_hallway[i] -= 1;
                                moved.push((index as i32, owner, cost));
                                had_move = true;
                                break;
                            }
                        }


                        let right_hall_index = left_hall_index + 1;
                        if let Some(index) = self.find_next_in_hall(right_hall_index, true) {
                            if *self.hallway[index].as_ref().unwrap() == owner {
                                self.hallway[index] = None;
                                let cost = base_cost + calc_cost(index, (i + 1) * 2, per_move_cost);
                                self.cost += cost;
                                self.rooms[i].add();
                                self.in_hallway[i] -= 1;
                                moved.push((index as i32, owner, cost));
                                had_move = true;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        moved
    }

    fn move_back(&mut self, to_move: &Vec<(i32, Amphipod, usize)>) {
        for (index, amphipod, cost) in to_move {
            let amp_index = amphipod.index();
            self.rooms[amp_index].remove();
            if *index < 0 {
                let to_room = -1 * (index + 1);
                self.rooms[to_room as usize].wrong.push(*amphipod);
            } else {
                let index = *index as usize;
                self.hallway[index] = Some(*amphipod);
                self.in_hallway[amp_index] += 1;
            }
            self.cost -= cost;
        }
    }

    fn get_open_positions(&self, from: usize) -> Range<usize> {
        let start = match self.find_next_in_hall(from, false) {
            Some(index) => index + 1,
            None => 0
        };
        let end = match self.find_next_in_hall(from + 1, true) {
            Some(index) => index,
            None => HALL_SIZE
        };
        start..end
    }
}

fn calc_cost(from: usize, to: usize, per_move_cost: usize) -> usize {
    match from < to {
        true => (to - from) * per_move_cost,
        false => (from - to) * per_move_cost
    }
}

fn print_hall_char(amphipod: &Option<Amphipod>) {
    let c = match amphipod {
        Some(amphipod) => amphipod.to_char(),
        None => '.'
    };
    print!("{}", c);
}

fn _print(burrow: &Burrow) {
    println!("#############");
    print!("#");
    for i in 0..2 {
        print_hall_char(&burrow.hallway[i]);
    }
    for i in 2..5 {
        print!("x");
        print_hall_char(&burrow.hallway[i]);
    }
    print!("x");
    for i in 5..7 {
        print_hall_char(&burrow.hallway[i]);
    }
    println!("#");
    for r in 0..burrow.rows {
        if r == 0 {
            print!("##");
        } else {
            print!("  ");
        }
        let index = (burrow.rows - r) - 1;
        for i in 0..ROOMS {
            print!("#");
            let room = &burrow.rooms[i];
            let top = room.correct + room.wrong.len();
            if top > index {
                if index >= room.correct {
                    print!("{}", room.wrong[index - room.correct].to_char());
                } else {
                    print!("{}", room.owner.to_char());
                }
            } else {
                print!(".");
            }
        }
        if r == 0 {
            println!("###");
        } else {
            println!("#");
        }
    }
    println!("  #########");
    println!("");
}

fn next_move(burrow: &mut Burrow) {
    let orig_cost = burrow.cost;
    let to_burrow = burrow.move_to_room();

    println!("---- after move to room, cost: {} ----", burrow.cost);
    _print(&burrow);

    // move from closed rooms to the hallway
    let mut closed_count = 0;
    for i in 0..ROOMS {
        if burrow.rooms[i].is_empty() {
            continue;
        } else {
            closed_count += 1;
            let next = &burrow.rooms[i].get_next();
            let positions = burrow.get_open_positions(next.index());
            let room = &mut burrow.rooms[i];
            room.remove_next();
            burrow.in_hallway[next.index()] += 1;
            let per_pos_cost = next.cost();
            let base_cost = (burrow.rows - (room.wrong.len() + room.correct)) * per_pos_cost;
            for to_pos in positions {
                let cur_pos = (i * 2) + 2;
                let cost = base_cost + calc_cost(cur_pos, to_pos, per_pos_cost);
                burrow.cost += cost;
                burrow.hallway[to_pos] = Some(*next);
                println!("---- after move to hallway - cost: {} ----", burrow.cost);
                _print(&burrow);
                next_move(burrow);
                burrow.hallway[to_pos] = None;
                burrow.cost -= cost;
            }
            burrow.in_hallway[next.index()] -= 1;
            burrow.rooms[i].wrong.push(*next);
        }
        break;
    }

    if closed_count == 0 {
        let none_in_hall = burrow.hallway.iter()
            .find(|item| item.is_some())
            .is_none();
        if none_in_hall && burrow.cost < burrow.lowest_cost {
            println!("setting lowest cost to {} from {}", burrow.cost, burrow.lowest_cost);
            burrow.lowest_cost = burrow.cost;
        }
    }

    burrow.move_back(&to_burrow);

    println!("End of tries, cost is now: {} - was {}", burrow.cost, orig_cost);
}

fn part_one(file_name: &str) {
    let mut burrow = Burrow::parse(file_name);
    burrow.move_bottom_to_correct();

    println!("---- initial ----");
    _print(&burrow);

    next_move(&mut burrow);

    println!("Part 1: {}", burrow.lowest_cost);
}

fn part_two(file_name: &str) {
    let mut burrow = Burrow::parse(file_name);
    burrow.move_bottom_to_correct();
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("sample.txt");
    // part_two("input.txt");

    println!("Done!");
}
