use std::ops::Range;

// input
const X_TARGET: Range<i32> = 192..252;
const Y_TARGET: Range<i32> = -89..-59;

fn part_one() {
    // (88 * 89) / 2 = 3916
    println!("Part 1: 3916");
}

fn find_within_range<S,E>(
    init_vel: i32, 
    decrease_to: i32, 
    all_max_step: usize, 
    start_check: S, 
    end_check: E) -> Range<usize> 
    where
    S: Fn(i32) -> bool,
    E: Fn(i32) -> bool
{
    let mut min_step = 1;
    let mut max_step = 1;
    let mut vel = init_vel;
    let mut pos = init_vel;
    while start_check(pos) && vel > decrease_to && max_step <= all_max_step {
        vel -= 1;
        pos += vel;
        min_step += 1;
        max_step += 1;
    }
    while end_check(pos) && max_step <= all_max_step {
        if vel > decrease_to {
            vel -= 1;
            pos += vel;
        } else {
            max_step = all_max_step;
            break;
        }
        max_step += 1;
    }
    min_step..max_step
}

fn part_two() {
    let x_min = 20; // any slower and we won't make it into the x range ever or will reach a speed of zero
    let x_max = 252; // any faster and the first step will over-shoot
    let y_max = 90; // from part 1
    let y_min = -90; // any lower and we over-shoot on first step

    let mut count: usize = 0;

    let y_start_check = |pos: i32| {
        pos > Y_TARGET.end
    };
    let y_end_check = |pos: i32| {
        pos >= Y_TARGET.start
    };
    
    let x_start_check = |pos: i32| {
        pos < X_TARGET.start
    };
    let x_end_check = |pos: i32| {
        pos < X_TARGET.end
    };

    for y in y_min..y_max {
        let y_range = find_within_range(y, i32::MIN, usize::MAX, y_start_check, y_end_check);
        if y_range.start == y_range.end {
            continue;
        }
        for x in x_min..x_max {
            let x_range = find_within_range(x, 0, y_range.end, x_start_check, x_end_check);
            if x_range.end > y_range.start && x_range.start < y_range.end {
                count += 1;
            }
        }
    }

    println!("Part 2: {}", count);
}

fn main() {
    part_one();
    part_two();

    println!("Done!");
}
