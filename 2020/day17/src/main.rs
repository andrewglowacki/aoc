use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::HashSet;

type Grid = HashSet<(i32, i32, i32, i32)>;
type Point = (i32, i32, i32, i32);

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn read_initial_grid(file_name: &str) -> Grid {
    let mut grid = Grid::new();
    let mut lines = get_file_lines(file_name);

    let mut y = 0;
    while let Some(Ok(line)) = lines.next() {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                grid.insert((x, y, 0, 0));
            }
            x += 1;
        }
        y += 1;
    }

    grid
}

fn count_adjacent(grid: &Grid, center: &Point) -> i32 {
    let mut count = -(grid.contains(center) as i32);
    let (center_x, center_y, center_z, center_w) = center;
    for x in (center_x - 1)..(center_x + 2) {
        for y in (center_y - 1)..(center_y + 2) {
            for z in (center_z - 1)..(center_z) + 2 {
                for w in (center_w - 1)..(center_w) + 2 {
                    count += grid.contains(&(x, y, z, w)) as i32;
                }
            }
        }
    }
    count
}

fn check_inactive(orig: &Grid, grid: &mut Grid, inactive_checked: &mut Grid, center: &Point) {
    let (center_x, center_y, center_z, center_w) = center;
    for x in (center_x - 1)..(center_x + 2) {
        for y in (center_y - 1)..(center_y + 2) {
            for z in (center_z - 1)..(center_z) + 2 {
                for w in (center_w - 1)..(center_w) + 2 {
                    let check_point = &(x, y, z, w);
                    if inactive_checked.contains(check_point) || orig.contains(check_point) {
                        continue;
                    }
                    if count_adjacent(orig, check_point) == 3 {
                        grid.insert(*check_point);
                    }
                    inactive_checked.insert(*check_point);
                }
            }
        }
    }
}

fn execute_one_cycle(grid: &mut Grid) {
    let mut inactive_checked: Grid = HashSet::new();

    let orig = grid.clone();
    for center in orig.iter() {
        let count = count_adjacent(&orig, &center);
        if count < 2 || count > 3 {
            grid.remove(&center);
        }
        check_inactive(&orig, grid, &mut inactive_checked, &center);
    }
}

fn test_input(file_name: &str) {
    let mut grid = read_initial_grid(file_name);

    for _ in 0..6 {
        execute_one_cycle(&mut grid);
    }

    println!("For {}, active after 6 cycles is {}", file_name, grid.len());
}

fn main() {
    test_input("sample.txt");
    test_input("input.txt");
}
