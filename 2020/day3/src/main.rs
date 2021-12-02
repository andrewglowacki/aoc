use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;

const TREE: char = '#';

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).unwrap();

    let grid = BufReader::new(file).lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    
    let slopes: Vec<(usize, usize)> = vec![
        (1, 1), 
        (3, 1), 
        (5, 1), 
        (7, 1), 
        (1, 2)
    ];

    let product = slopes.iter().map(|slope| {
        let (dx, dy) = slope;
        let trees = count_trees(&grid, *dx, *dy);
        println!("For Slope: {}, {}: {}", dx, dy, trees);
        trees
    }).product::<usize>();

    println!("Product of trees is: {}", product);
}

fn is_tree(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let row = grid.get(y).unwrap();
    let item = row.get(x % row.len()).unwrap();
    *item == TREE
}

fn count_trees(grid: &Vec<Vec<char>>, dx: usize, dy: usize) -> usize {
    let steps = grid.len() / dy;
    (0..steps).filter(|index| {
        let x = index * dx;
        let y = index * dy;
        is_tree(grid, x, y)
    })
    .count()
}
