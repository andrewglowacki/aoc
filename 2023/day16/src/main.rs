use std::collections::{BTreeSet, BTreeMap, HashSet, HashMap};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Debug, Clone)]
enum Mirror {
    Horizontal,
    Vertical,
    BackSlash,
    ForwardSlash
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Arrangement {
    x_to_y: Vec<BTreeMap<usize, Mirror>>,
    y_to_x: Vec<BTreeMap<usize, Mirror>>,
    width: usize,
    height: usize
}

impl Arrangement {
    fn parse(file_name: &str) -> Arrangement {
        let mut y: usize = 0;
        let mut width = 0;
        let points = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .flat_map(|line| {
                let row = line.char_indices()
                    .filter_map(|(x, c)| {
                        match c {
                            '-' => Some((x, y, Mirror::Horizontal)),
                            '|' => Some((x, y, Mirror::Vertical)),
                            '/' => Some((x, y, Mirror::ForwardSlash)),
                            '\\' => Some((x, y, Mirror::BackSlash)),
                            '.' => None,
                            _ => panic!("Unexpected character: {}", c)
                        }
                    })
                    .collect::<Vec<_>>();
                if width == 0 {
                    width = line.len();
                }
                y += 1;
                row
            })
            .collect::<Vec<_>>();

        let mut y_to_x = vec![BTreeMap::new(); y];
        let mut x_to_y = vec![BTreeMap::new(); width];

        points.into_iter().for_each(|(x, y, mirror)| {
            x_to_y[x].insert(y, mirror.clone());
            y_to_x[y].insert(x, mirror);
        });

        Arrangement { x_to_y, y_to_x, width, height: y }
    }

    fn reflect_mirror(x: usize, y: usize, direction: Direction, mirror: &Mirror, beams: &mut Vec<(usize, usize, Direction)>) {
        match (&direction, mirror) {
            (Direction::Left | Direction::Right, Mirror::Horizontal) => beams.push((x, y, direction)),
            (Direction::Up | Direction::Down, Mirror::Horizontal) => {
                beams.push((x, y, Direction::Left));
                beams.push((x, y, Direction::Right));
            },
            (Direction::Up | Direction::Down, Mirror::Vertical) => beams.push((x, y, direction)), 
            (Direction::Left | Direction::Right, Mirror::Vertical) => {
                beams.push((x, y, Direction::Up));
                beams.push((x, y, Direction::Down));
            },
            (Direction::Up, Mirror::BackSlash) => beams.push((x, y, Direction::Left)),
            (Direction::Down, Mirror::BackSlash) => beams.push((x, y, Direction::Right)),
            (Direction::Left, Mirror::BackSlash) => beams.push((x, y, Direction::Up)),
            (Direction::Right, Mirror::BackSlash) => beams.push((x, y, Direction::Down)),
            (Direction::Up, Mirror::ForwardSlash) => beams.push((x, y, Direction::Right)),
            (Direction::Down, Mirror::ForwardSlash) => beams.push((x, y, Direction::Left)),
            (Direction::Left, Mirror::ForwardSlash) => beams.push((x, y, Direction::Down)),
            (Direction::Right, Mirror::ForwardSlash) => beams.push((x, y, Direction::Up)),
        }
    }

    fn count_energized(&self, start: (usize, usize, Direction)) -> usize {
        let mut energized = HashMap::<(usize, usize), HashSet<Direction>>::new();
        let mut traversed = HashSet::<(usize, usize)>::new();
        let mut beams = Vec::<(usize, usize, Direction)>::new();

        let (start_x, start_y, start_direction) = start;
        
        if let Some(mirror) = self.x_to_y[start_x].get(&start_y) {
            Arrangement::reflect_mirror(start_x, start_y, start_direction.clone(), mirror, &mut beams);
        } else {
            beams.push((start_x, start_y, start_direction.clone()));
        }

        beams.iter().for_each(|(x, y, _)| {
            traversed.insert((*x, *y));
        });

        while let Some((x, y, direction)) = beams.pop() {

            let next = match direction {
                Direction::Right => {
                    self.y_to_x[y].range(x + 1..).next()
                        .map(|(x, mirror)| (*x, y, mirror))
                },
                Direction::Left => {
                    self.y_to_x[y].range(..x).last()
                        .map(|(x, mirror)| (*x, y, mirror))
                },
                Direction::Down => {
                    self.x_to_y[x].range(y + 1..).next()
                        .map(|(y, mirror)| (x, *y, mirror))
                },
                Direction::Up => {
                    self.x_to_y[x].range(..y).last()
                        .map(|(y, mirror)| (x, *y, mirror))
                }
            };

            if let Some((new_x, new_y, mirror)) = next {

                match direction {
                    Direction::Right => (x..new_x).into_iter()
                        .for_each(|x| { traversed.insert((x, y)); }),
                    Direction::Left => (new_x..x).into_iter()
                        .for_each(|x| { traversed.insert((x, y)); }),
                    Direction::Up => (new_y..y).into_iter()
                        .for_each(|y| { traversed.insert((x, y)); }),
                    Direction::Down => (y..new_y).into_iter()
                        .for_each(|y| { traversed.insert((x, y)); })
                }

                let x = new_x;
                let y = new_y;

                if let Some(directions) = energized.get_mut(&(x, y)) {
                    if !directions.insert(direction.clone()) {
                        // we've already encountered this mirror in this,
                        // direction, so anything after this would be 
                        // redundant, so stop following this beam
                        continue;
                    }
                } else {
                    let mut directions = HashSet::new();
                    directions.insert(direction.clone());
                    energized.insert((x, y), directions);
                }

                Arrangement::reflect_mirror(x, y, direction, mirror, &mut beams);
            } else {
                match direction {
                    Direction::Right => (x..self.width).into_iter()
                        .for_each(|x| { traversed.insert((x, y)); }),
                    Direction::Left => (0..x).into_iter()
                        .for_each(|x| { traversed.insert((x, y)); }),
                    Direction::Up => (0..y).into_iter()
                        .for_each(|y| { traversed.insert((x, y)); }),
                    Direction::Down => (y..self.height).into_iter()
                        .for_each(|y| { traversed.insert((x, y)); })
                }
            }
        }

        let traversed_count = traversed.iter()
            .filter(|point| !energized.contains_key(point))
            .count();

        let total = energized.len() + traversed_count;
        total
    }
}

fn part_one(file_name: &str) {
    let arrangement = Arrangement::parse(file_name);
    let energized = arrangement.count_energized((0, 0, Direction::Right));
    println!("Part 1: {}", energized);
}

fn part_two(file_name: &str) {
    let arrangement = Arrangement::parse(file_name);

    let mut max = 0;

    for x in 0..arrangement.width {
        max = max.max(arrangement.count_energized((x, 0, Direction::Down)));
        max = max.max(arrangement.count_energized((x, arrangement.height - 1, Direction::Up)));
    }
    for y in 0..arrangement.height {
        max = max.max(arrangement.count_energized((0, y, Direction::Right)));
        max = max.max(arrangement.count_energized((arrangement.width - 1, y, Direction::Left)));
    }
    
    println!("Part 2: {}", max);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
