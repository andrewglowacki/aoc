use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new() -> Point {
        Point {
            x: 0,
            y: 0
        }
    }
    
    fn set(&mut self, point: (i32, i32)) {
        let (x, y) = point;
        self.x = x;
        self.y = y;
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn move_x(&mut self, amount: i32) {
        self.x += amount;
    }
    fn move_y(&mut self, amount: i32) {
        self.y += amount;
    }

}

struct RopeBridge {
    points: Vec<Point>,
    visited: HashSet<(i32, i32)>
}

struct MoveResult {
    tail: (i32, i32),
    head: (i32, i32),
    visited: Vec<(i32, i32)>
}

impl RopeBridge {
    fn new(size: usize) -> RopeBridge {
        let mut visited = HashSet::new();

        visited.insert((0, 0));

        let mut points = Vec::new();
        
        for _ in 0..size {
            points.push(Point::new());
        }

        RopeBridge {
            points,
            visited
        }
    }

    fn move_head<P>( 
        tail: &Point,
        moving_tail: i32, 
        moving_head: i32, 
        fixed_head: i32, 
        point_maker: P, 
        amount: i32) -> MoveResult
        where P: Fn(i32, i32) -> (i32, i32)
    {
        let points = if amount > 0 {
            (moving_head..moving_head + amount).into_iter()
                .filter(|i| *i > moving_tail)
                .map(|i| point_maker(i, fixed_head))
                .collect::<Vec<_>>()
        } else {
            let amount = amount + 1;
            (moving_head + amount..moving_head + 1).into_iter()
                .filter(|i| *i < moving_tail)
                .map(|i| point_maker(i, fixed_head))
                .rev()
                .collect::<Vec<_>>()
        };

        let new_tail = match points.last() {
            Some((x, y)) => (*x, *y),
            _ => (tail.x, tail.y)
        };
        
        let new_head = point_maker(moving_head + amount, fixed_head);

        MoveResult { 
            tail: new_tail, 
            head: new_head,
            visited: points
        }
    }
    
    fn move_x(tail: &Point, head: &Point, amount: i32) -> MoveResult {
        RopeBridge::move_head(tail, tail.x, head.x, head.y, |x, y| (x, y), amount)
    }

    fn move_y(tail: &Point, head: &Point, amount: i32) -> MoveResult {
        RopeBridge::move_head(tail, tail.y, head.y, head.x, |y, x| (x, y), amount)
    }

    fn perform_move(&mut self, instruction: &str) {
        let direction = instruction.chars().next().unwrap();
        let amount = instruction[2..].parse::<i32>().unwrap();

        let (move_x, new_amount) = match direction {
            'U' => (false, -amount),
            'D' => (false, amount),
            'L' => (true, -amount),
            'R' => (true, amount),
            _ => panic!("Invalid move direction in: {}", instruction)
        };

        let amount = new_amount;

        let mut last_visited = Vec::new();
        let mut prev_tail = Point::new();
        prev_tail.set((self.points[0].x, self.points[0].y));
        
        for i in 1..self.points.len() {
            let result = match move_x {
                false => RopeBridge::move_y(&self.points[i], &prev_tail, amount),
                true => RopeBridge::move_x(&self.points[i], &prev_tail, amount)
            };
    
            prev_tail.set((self.points[i].x, self.points[i].y));
            self.points[i].set(result.tail);
            if i == 1 {
                self.points[0].set(result.head);
            }

            last_visited = result.visited;
        }
        
        last_visited.into_iter().for_each(|point| {
            self.visited.insert(point);
        });
        
    }
}

fn part_one(file_name: &str) {
    let mut bridge = RopeBridge::new(2);

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .for_each(|line| bridge.perform_move(line.as_str()));

    println!("Part 1: {}", bridge.visited.len());
}

fn part_two(file_name: &str) {
    let mut bridge = RopeBridge::new(10);

    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("input.txt");
    // part_two("input.txt");

    println!("Done!");
}