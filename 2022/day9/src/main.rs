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
    
    fn set(&mut self, x: i32, y: i32) {
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
    head: Point,
    tail: Point,
    visited: HashSet<(i32, i32)>
}

impl RopeBridge {
    fn new() -> RopeBridge {
        let mut visited = HashSet::new();

        visited.insert((0, 0));

        RopeBridge {
            head: Point::new(),
            tail: Point::new(),
            visited
        }
    }

    fn move_head<M, F, P>(&mut self, moving: M, fixed: F, point_maker: P, amount: i32)
        where M: Fn(&Point) -> i32,
            F: Fn(&Point) -> i32,
            P: Fn(i32, i32) -> (i32, i32)
    {
        let points = if amount > 0 {
            (moving(&self.head)..moving(&self.head) + amount).into_iter()
                .filter(|i| *i > moving(&self.tail))
                .map(|i| point_maker(i, fixed(&self.head)))
                .collect::<Vec<_>>()
        } else {
            let amount = amount + 1;
            (moving(&self.head) + amount..moving(&self.head)).into_iter()
                .filter(|i| *i < moving(&self.tail))
                .map(|i| point_maker(i, fixed(&self.head)))
                .rev()
                .collect::<Vec<_>>()
        };

        if let Some((x, y)) = points.last() {
            self.tail.set(*x, *y);
        }
        points.iter().for_each(|point| { 
            self.visited.insert(*point); 
        });
        
        let (x, y) = point_maker(moving(&self.head) + amount, fixed(&self.head));
        self.head.set(x, y);
    }
    
    fn move_x(&mut self, amount: i32) {
        self.move_head(Point::x, Point::y, |x, y| (x, y), amount);
    }

    fn move_y(&mut self, amount: i32) {
        self.move_head(Point::y, Point::x, |y, x| (x, y), amount);
    }

    fn perform_move(&mut self, instruction: &str) {
        let direction = instruction.chars().next().unwrap();
        let amount = instruction[2..].parse::<i32>().unwrap();
        
        match direction {
            'U' => self.move_y(-amount),
            'D' => self.move_y(amount),
            'L' => self.move_x(-amount),
            'R' => self.move_x(amount),
            _ => panic!("Invalid move direction in: {}", instruction)
        };
        
    }
}

fn part_one(file_name: &str) {
    let mut bridge = RopeBridge::new();

    get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .for_each(|line| bridge.perform_move(line.as_str()));

    println!("Part 1: {}", bridge.visited.len());
}

fn part_two(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok());
    
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one("input.txt");
    // part_two("input.txt");

    println!("Done!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn visited(bridge: &RopeBridge) -> Vec<(i32, i32)> {
        let mut points = bridge.visited.iter()
            .map(|item| *item)
            .collect::<Vec<_>>();
        points.sort();
        points
    }

    #[test]
    fn point_test() {
        let mut point = Point::new();
        assert_eq!(0, point.x());
        assert_eq!(0, point.y());
        
        point.move_x(3);
        assert_eq!(3, point.x());
        assert_eq!(0, point.y());

        point.move_x(-1);
        assert_eq!(2, point.x());
        assert_eq!(0, point.y());

        point.move_y(5);
        assert_eq!(2, point.x());
        assert_eq!(5, point.y());
        
        point.move_y(-2);
        assert_eq!(2, point.x());
        assert_eq!(3, point.y());

        point.set(-6, 9);
        assert_eq!(-6, point.x());
        assert_eq!(9, point.y());
    }

    #[test]
    fn bridge_default() {
        let bridge = RopeBridge::new();
        assert_eq!(vec![(0, 0)], visited(&bridge));
        assert_eq!(0, bridge.head.x());
        assert_eq!(0, bridge.head.y());
        assert_eq!(0, bridge.tail.x());
        assert_eq!(0, bridge.tail.y());
    }

    #[test]
    fn move_right_no_movement() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("R 1");

        assert_eq!(vec![(0, 0)], visited(&bridge));
        assert_eq!(1, bridge.head.x());
        assert_eq!(0, bridge.head.y());
        assert_eq!(0, bridge.tail.x());
        assert_eq!(0, bridge.tail.y());
    }

    #[test]
    fn move_left_no_movement() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("L 1");

        assert_eq!(vec![(0, 0)], visited(&bridge));
        assert_eq!(-1, bridge.head.x());
        assert_eq!(0, bridge.head.y());
        assert_eq!(0, bridge.tail.x());
        assert_eq!(0, bridge.tail.y());
    }

    #[test]
    fn move_up_no_movement() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("U 1");

        assert_eq!(vec![(0, 0)], visited(&bridge));
        assert_eq!(0, bridge.head.x());
        assert_eq!(-1, bridge.head.y());
        assert_eq!(0, bridge.tail.x());
        assert_eq!(0, bridge.tail.y());
    }

    #[test]
    fn move_down_no_movement() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("D 1");

        assert_eq!(vec![(0, 0)], visited(&bridge));
        assert_eq!(0, bridge.head.x());
        assert_eq!(1, bridge.head.y());
        assert_eq!(0, bridge.tail.x());
        assert_eq!(0, bridge.tail.y());
    }

    #[test]
    fn move_right_same_line() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("R 3");

        assert_eq!(vec![(0, 0), (1, 0), (2, 0)], visited(&bridge));
        assert_eq!(3, bridge.head.x());
        assert_eq!(0, bridge.head.y());
        assert_eq!(2, bridge.tail.x());
        assert_eq!(0, bridge.tail.y());
    }

    #[test]
    fn move_left_same_line() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("L 3");

        assert_eq!(vec![(-2, 0), (-1, 0), (0, 0)], visited(&bridge));
        assert_eq!(-3, bridge.head.x());
        assert_eq!(0, bridge.head.y());
        assert_eq!(-2, bridge.tail.x());
        assert_eq!(0, bridge.tail.y());
    }

    #[test]
    fn move_up_same_line() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("U 3");

        assert_eq!(vec![(0, -2), (0, -1), (0, 0)], visited(&bridge));
        assert_eq!(0, bridge.head.x());
        assert_eq!(-3, bridge.head.y());
        assert_eq!(0, bridge.tail.x());
        assert_eq!(-2, bridge.tail.y());
    }

    #[test]
    fn move_down_same_line() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("D 3");

        assert_eq!(vec![(0, 0), (0, 1), (0, 2)], visited(&bridge));
        assert_eq!(0, bridge.head.x());
        assert_eq!(3, bridge.head.y());
        assert_eq!(0, bridge.tail.x());
        assert_eq!(2, bridge.tail.y());
    }

    #[test]
    fn move_up_right_one() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("U 1");
        bridge.perform_move("R 1");

        assert_eq!(vec![(0, 0)], visited(&bridge));
        assert_eq!(1, bridge.head.x());
        assert_eq!(-1, bridge.head.y());
        assert_eq!(0, bridge.tail.x());
        assert_eq!(0, bridge.tail.y());
    }
    
    #[test]
    fn move_up_left_one() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("U 1");
        bridge.perform_move("L 1");

        assert_eq!(vec![(0, 0)], visited(&bridge));
        assert_eq!(-1, bridge.head.x());
        assert_eq!(-1, bridge.head.y());
        assert_eq!(0, bridge.tail.x());
        assert_eq!(0, bridge.tail.y());
    }
    #[test]
    fn move_down_right_one() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("D 1");
        bridge.perform_move("R 1");

        assert_eq!(vec![(0, 0)], visited(&bridge));
        assert_eq!(1, bridge.head.x());
        assert_eq!(1, bridge.head.y());
        assert_eq!(0, bridge.tail.x());
        assert_eq!(0, bridge.tail.y());
    }
    #[test]
    fn move_down_left_one() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("D 1");
        bridge.perform_move("L 1");

        assert_eq!(vec![(0, 0)], visited(&bridge));
        assert_eq!(-1, bridge.head.x());
        assert_eq!(1, bridge.head.y());
        assert_eq!(0, bridge.tail.x());
        assert_eq!(0, bridge.tail.y());
    }
    
    #[test]
    fn move_up_right_three() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("U 1");
        bridge.perform_move("R 3");

        assert_eq!(vec![(0, 0), (1, -1), (2, -1)], visited(&bridge));
        assert_eq!(3, bridge.head.x());
        assert_eq!(-1, bridge.head.y());
        assert_eq!(2, bridge.tail.x());
        assert_eq!(-1, bridge.tail.y());
    }
    
    #[test]
    fn move_up_left_three() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("U 1");
        bridge.perform_move("L 3");

        assert_eq!(vec![(-2, -1), (-1, -1), (0, 0)], visited(&bridge));
        assert_eq!(-3, bridge.head.x());
        assert_eq!(-1, bridge.head.y());
        assert_eq!(-2, bridge.tail.x());
        assert_eq!(-1, bridge.tail.y());
    }
    
    #[test]
    fn move_down_right_three() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("D 1");
        bridge.perform_move("R 3");

        assert_eq!(vec![(0, 0), (1, 1), (2, 1)], visited(&bridge));
        assert_eq!(3, bridge.head.x());
        assert_eq!(1, bridge.head.y());
        assert_eq!(2, bridge.tail.x());
        assert_eq!(1, bridge.tail.y());
    }
    
    #[test]
    fn move_down_left_three() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("D 1");
        bridge.perform_move("L 3");

        assert_eq!(vec![(-2, 1), (-1, 1), (0, 0)], visited(&bridge));
        assert_eq!(-3, bridge.head.x());
        assert_eq!(1, bridge.head.y());
        assert_eq!(-2, bridge.tail.x());
        assert_eq!(1, bridge.tail.y());
    }
    
    #[test]
    fn move_up_right_down() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("U 3");
        bridge.perform_move("R 1");
        bridge.perform_move("D 6");

        assert_eq!(vec![(0, -2), (0, -1), (0, 0), (1, -1), (1, 0), (1, 1), (1, 2)], visited(&bridge));
        assert_eq!(1, bridge.head.x());
        assert_eq!(3, bridge.head.y());
        assert_eq!(1, bridge.tail.x());
        assert_eq!(2, bridge.tail.y());
    }
    
    #[test]
    fn move_down_right_up() {
        let mut bridge = RopeBridge::new();

        bridge.perform_move("D 3");
        bridge.perform_move("R 1");
        bridge.perform_move("U 6");

        assert_eq!(vec![(0, 0), (0, 1), (0, 2), (1, -2), (1, -1), (1, 0), (1, 1)], visited(&bridge));
        assert_eq!(1, bridge.head.x());
        assert_eq!(-3, bridge.head.y());
        assert_eq!(1, bridge.tail.x());
        assert_eq!(-2, bridge.tail.y());
    }
    
}