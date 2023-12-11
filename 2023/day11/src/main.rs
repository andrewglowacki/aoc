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

struct Universe {
    galaxies: HashSet<(i64, i64)>
}

impl Universe {
    fn parse(file_name: &str, expansion: i64) -> Universe {
        let expansion = expansion - 1;
        let grid = get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut galaxies_by_column = (0..grid[0].len())
            .map(|_| Vec::<i64>::new())
            .collect::<Vec<_>>();

        let mut use_y = 0;
        for y in 0..grid.len() {
            let row = &grid[y];
            let mut had_galaxy = false;
            for x in 0..row.len() {
                if row[x] == '#' {
                    galaxies_by_column[x].push(use_y);
                    had_galaxy = true;
                }
            }
            if !had_galaxy {
                use_y += expansion;
            }
            use_y += 1;
        }

        let mut galaxies = HashSet::new();

        let mut use_x = 0;
        for x in 0..galaxies_by_column.len() {
            let column = &galaxies_by_column[x];
            if column.is_empty() {
                use_x += expansion;
            } else {
                column.into_iter()
                    .map(|y| (use_x, *y))
                    .for_each(|point| { galaxies.insert(point); });
            }
            use_x += 1;
        }

        Universe { galaxies }
    }

    fn sum_distances(&self) -> i64 {
        let mut total = 0;
        for (one_x, one_y) in &self.galaxies {
            for (two_x, two_y) in &self.galaxies {
                if one_x == two_x && one_y == two_y {
                    continue;
                }

                // avoid dupes
                if one_x > two_x {
                    continue;
                } else if one_x == two_x && one_y > two_y {
                    continue;
                }

                let dist = 
                    (one_x - two_x).abs() as i64 + 
                    (one_y - two_y).abs() as i64;

                total += dist;
            }
        }
        total
    }
}

fn part_one(file_name: &str) {
    let universe = Universe::parse(file_name, 2);
    let total = universe.sum_distances();
    println!("Part 1: {}", total);
}

fn part_two(file_name: &str) {
    let universe = Universe::parse(file_name, 1000000);
    let total = universe.sum_distances();
    println!("Part 2: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}
