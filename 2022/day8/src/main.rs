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

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Trees {
    trees: Vec<Vec<u32>>,
    width: usize,
    height: usize
}

impl Trees {
    fn new(file_name: &str) -> Trees {
        let trees = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        let width = trees[0].len();
        let height = trees.len();

        Trees {
            trees,
            width,
            height
        }
    }

    fn move_next(&self, direction: Direction, point: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = point;
        match direction {
            Direction::Up    if y > 0               => Some((x, y - 1)),
            Direction::Down  if y + 1 < self.height => Some((x, y + 1)),
            Direction::Left  if x > 0               => Some((x - 1, y)),
            Direction::Right if x + 1 < self.width  => Some((x + 1, y)),
            _ => None
        }
    }

    fn height(&self, point: (usize, usize)) -> u32 {
        let (x, y) = point;
        self.trees[y][x]
    }

    fn count_visible_trees(&self, direction: Direction, visible: &mut HashSet<(usize, usize)>) {
        let (move_after_row, mut point) = match direction {
            Direction::Down => (Direction::Right, (0, 0)),
            Direction::Up => (Direction::Right, (0, self.height - 1)),
            Direction::Right => (Direction::Down, (0, 0)),
            Direction::Left => (Direction::Down, (self.width - 1, 0))
        };

        let mut tallest = self.height(point);
        visible.insert(point);

        loop {
            let start = point;
            while let Some(new_point) = self.move_next(direction, point) {
                point = new_point;
                let height = self.height(point);
                if height > tallest {
                    tallest = height;
                    visible.insert(point);
                }
            }
            if let Some(new_point) = self.move_next(move_after_row, start) {
                point = new_point;
                tallest = self.height(point);
                visible.insert(point);
            } else {
                break;
            }
        }
    }

    fn count_visible_trees_from(&self, direction: Direction, mut point: (usize, usize)) -> u32 {
        let mut count = 0;
        let my_height = self.height(point);
        while let Some(new_point) = self.move_next(direction, point) {
            point = new_point;
            let height = self.height(point);
            count += 1;
            if height >= my_height {
                break;
            }
        }
        count
    }

    fn determine_scenic_score(&self, point: (usize, usize)) -> u32 {
        self.count_visible_trees_from(Direction::Up, point) *
        self.count_visible_trees_from(Direction::Down, point) *
        self.count_visible_trees_from(Direction::Left, point) *
        self.count_visible_trees_from(Direction::Right, point)
    }

    fn determine_best_scenic_score(&self) -> u32 {
        let mut best = 0;
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let score = self.determine_scenic_score((x, y));
                best = best.max(score);
            }
        }
        best
    }

}

fn part_one(file_name: &str) {
    let trees = Trees::new(file_name);

    let mut visible = HashSet::new();

    trees.count_visible_trees(Direction::Down, &mut visible);
    trees.count_visible_trees(Direction::Up, &mut visible);
    trees.count_visible_trees(Direction::Left, &mut visible);
    trees.count_visible_trees(Direction::Right, &mut visible);
    
    println!("Part 1: {}", visible.len());
}

fn part_two(file_name: &str) {
    let trees = Trees::new(file_name);

    let best = trees.determine_best_scenic_score();
    
    println!("Part 2: {}", best);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
