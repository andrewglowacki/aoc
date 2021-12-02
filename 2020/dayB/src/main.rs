use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

type Layout = Vec<Vec<char>>;

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn occupied_count(seat: char) -> u32 {
    match seat {
        '#' => 1,
        _ => 0
    }
}

fn get_seat(layout: &Layout, x: i32, y: i32) -> Option<char> {
    if x < 0 || y < 0 || y >= layout.len() as i32 {
        None
    } else {
        let row = &layout[y as usize];
        let x = x as usize;
        if x >= row.len() {
            None
        } else {
            Some(row[x])
        }
    }
}

fn get_adjacent_occupied(layout: &Layout, x: usize, y: usize) -> u32 {
    let x = x as i32;
    let y = y as i32;

    vec![
        get_seat(layout, x - 1, y - 1),
        get_seat(layout, x, y - 1),
        get_seat(layout, x + 1, y - 1),
        get_seat(layout, x - 1, y),
        get_seat(layout, x + 1, y),
        get_seat(layout, x - 1, y + 1),
        get_seat(layout, x, y + 1),
        get_seat(layout, x + 1, y + 1)
    ].iter().flat_map(|seat| seat)
        .map(|seat| occupied_count(*seat))
        .sum()
}

fn get_los_occupied_with_direction(layout: &Layout, x: usize, y: usize, x_dir: i32, y_dir: i32) -> u32 {
    let mut x = (x as i32) + x_dir;
    let mut y = (y as i32) + y_dir;
    while let Some(seat) = get_seat(layout, x, y) {
        match seat {
            '#' => return 1,
            'L' => return 0,
            _ => ()
        };
        x += x_dir;
        y += y_dir;
    }
    return 0;
}

fn get_los_occupied(layout: &Layout, x: usize, y: usize) -> u32 {
    get_los_occupied_with_direction(layout, x, y, -1, -1) +
    get_los_occupied_with_direction(layout, x, y, -1, 0) +
    get_los_occupied_with_direction(layout, x, y, -1, 1) +
    get_los_occupied_with_direction(layout, x, y, 0, -1) +
    get_los_occupied_with_direction(layout, x, y, 0, 1) +
    get_los_occupied_with_direction(layout, x, y, 1, -1) +
    get_los_occupied_with_direction(layout, x, y, 1, 0) +
    get_los_occupied_with_direction(layout, x, y, 1, 1)
}

fn change_seats_part_one(layout: &Layout) -> (u32, Layout) {
    let mut new_layout = Layout::new();

    let mut changes = 0;
    for y in 0..layout.len() {
        let row_len = layout[y].len();
        let mut new_row = vec![];
        for x in 0..row_len {
            new_row.push(match layout[y][x] {
                '#' if get_adjacent_occupied(&layout, x, y) >= 4 => {
                    changes += 1;
                    'L'
                },
                'L' if get_adjacent_occupied(&layout, x, y) == 0 => {
                    changes += 1;
                    '#'
                },
                x => x
            });
        }
        new_layout.push(new_row);
    }

    (changes, new_layout)
}

fn change_seats_part_two(layout: &Layout) -> (u32, Layout) {
    let mut new_layout = Layout::new();

    let mut changes = 0;
    for y in 0..layout.len() {
        let row_len = layout[y].len();
        let mut new_row = vec![];
        for x in 0..row_len {
            new_row.push(match layout[y][x] {
                '#' if get_los_occupied(&layout, x, y) >= 5 => {
                    changes += 1;
                    'L'
                },
                'L' if get_los_occupied(&layout, x, y) == 0 => {
                    changes += 1;
                    '#'
                },
                x => x
            });
        }
        new_layout.push(new_row);
    }

    (changes, new_layout)
}

fn get_occupied_count(layout: &Layout) -> u32 {
    let mut occupied = 0;
    for y in 0..layout.len() {
        let row = &layout[y];
        for x in 0..row.len() {
            occupied += occupied_count(row[x]);
        }
    }
    occupied
}

fn stabilize_part_one(layout: Layout) -> Layout {
    let mut layout = layout;
    loop {
        let (changes, new_layout) = change_seats_part_one(&layout);
        layout = new_layout;
        if changes == 0 {
            return layout;
        }
    }
}

fn stabilize_part_two(layout: Layout) -> Layout {
    let mut layout = layout;
    loop {
        let (changes, new_layout) = change_seats_part_two(&layout);
        layout = new_layout;
        if changes == 0 {
            return layout;
        }
    }
}

fn run_input_file(file_name: &str) {
    let layout_orig = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Layout>();
    
    let stable_layout = stabilize_part_one(layout_orig.clone());
    let occupied_count = get_occupied_count(&stable_layout);
    println!("For {}, Part 1 - Stable occupied count is {}", file_name, occupied_count);

    let stable_layout = stabilize_part_two(layout_orig.clone());
    let occupied_count = get_occupied_count(&stable_layout);
    println!("For {}, Part 2 - Stable occupied count is {}", file_name, occupied_count);
}

fn main() {
    run_input_file("sample.txt");
    run_input_file("input.txt");
}
