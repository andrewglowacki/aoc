use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::BTreeSet;

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn extended_glowackian_algorithm(args: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    // 
    // Compares the first tuple in the args with 
    // each other tuple in the args. It determines
    // the frequency with which they are equal to each
    // other (less subtracting the initial difference)
    //
    // Ex:
    // How often do these two intersect?
    // 7x + 3 and 15y + 4
    // 7 -  3 = 4,  15 - 4 =  11 - diff 7  - initial case, we subtract the differences. (x multiple, y multiple) = (1,1)
    // 4  + 7 = 11, 11 + 0 =  11 - diff 0  - (2, 1) - diff = 0, so record this as the first seen multiples
    // 11 + 7 = 18, 11 + 0 =  11 - diff 7  - (3, 1)
    // 18 + 0 = 18, 11 + 15 = 26 - diff 8  - (3, 2)
    // 18 + 7 = 24, 26 + 0  = 26 - diff 2  - (4, 2)
    // 24 + 7 = 32, 26 + 0 =  26 - diff 6  - (5, 2)
    // 32 + 0 = 32, 26 + 15 = 41 - diff 9  - (5, 3)
    // 32 + 7 = 39, 41 + 0  = 41 - diff 2 -  (6, 3)
    // 39 + 7 = 46, 46 + 0 =  46 - diff 7  - (7, 3) second intersection - first intersection = (5, 2)
    //
    let (base, base_diff) = args[0];
    let mut results = vec![];
    let mut results2 = vec![];
    for i in 1..args.len() {
        let (check, check_diff) = args[i];
        let mut x = base - base_diff;
        let mut y = check - check_diff;
        let mut x_mult = 1;
        let mut y_mult = 1;
        let mut seen_one = false;
        let mut seen_at = (0, 0);
        loop {
            while y < x {
                y += check;
                y_mult += 1;
            }
            if y == x {
                if seen_one {
                    break;
                } else {
                    seen_one = true;
                    seen_at = (x_mult, y_mult);
                }
            }
            x += base;
            x_mult += 1;
        }
        results.push((x_mult - seen_at.0, seen_at.0));
    }

    results
}

fn test_input(file_name: &str) {
    let lines = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .collect::<Vec<String>>();
    
    let arrival_time = lines[0].parse::<i32>().unwrap();

    let buses = lines[1].split(",")
        .flat_map(|bus| bus.parse::<i32>())
        .collect::<BTreeSet<i32>>();
    
    let (wait_time, earliest_bus) = buses.iter()
        .map(|bus| (bus - (arrival_time % bus), bus))
        .min_by_key(|entry| entry.0)
        .unwrap();
    
    println!("For {}, Earliest Bus is {} answer: {}", file_name, earliest_bus, earliest_bus * wait_time);

    let buses = lines[1].split(",")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
    
    println!("buses are: {:?}", buses);

    let buses_and_index = (0..buses.len())
        .filter(|i| buses[*i] != "x")
        .map(|i| (buses[i].parse::<i64>().unwrap(), i as i64))
        .collect::<Vec<(i64, i64)>>();

    println!("buses and index: {:?}", buses_and_index);
    
    let (first, first_diff) = buses_and_index[0];

    // each element is:
    // per multiple of the base variable:
    // the following tuples:
    // (how often the first number and comparator are equal, starting offset repeating starts)
    let intersection = extended_glowackian_algorithm(buses_and_index);
    // intersection should now indicate how often first and
    // each other variable are equal (less the difference)

    // Ex:
    // buses and index: 
    // [(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)]
    // Results of iteration: 
    // [(13, 11), (59, 50), (31, 8), (19, 18)]
    //

    let mut max: i64 = 0;
    let mut next_intersection = intersection.iter()
        .cloned()
        .collect::<Vec<(i64, i64)>>();

    let orig_first_multiple = next_intersection[0].0;
    next_intersection.sort_by(|a, b| {
        b.0.partial_cmp(&a.0).unwrap()
    });

    // now we have to find when the result multiples 'intersect',
    // once all variables are the same, we are done
    let mut iter: i64 = 0;
    'start: loop {
        if iter % 10000000 == 0 {
            // get all variables close to the max
            for (base_multiple, base) in next_intersection.iter_mut() {
                let diff = max - *base;
                if diff <= 0 {
                    continue;
                }
                let extra = diff % *base_multiple;
                *base += diff - extra;
            }
            if iter % 100000000 == 0 {
                println!("next_intersection temp is {:?}", next_intersection);
            }
        }
        for (base_multiple, base) in next_intersection.iter_mut() {
            while *base < max {
                *base += *base_multiple;
            }
            if *base != max {
                max = *base;
                iter += 1;
                continue 'start;
            }
        }
        break;
    }

    let (_, variable) = next_intersection.iter()
        .find(|entry| entry.0 == orig_first_multiple)
        .unwrap();

    println!("next_intersection {:?}", next_intersection);

    println!("For {}, Variable is: {}, Start time is: {}", file_name, variable, variable * first - first_diff);
    
}

fn main() {
    test_input("sample.txt");
    test_input("input.txt");
}
