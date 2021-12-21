
use std::cmp::min;

const PLAYER_1_INIT: u32 = 2;
const PLAYER_2_INIT: u32 = 8;
const END_SCORE: u32 = 1000;

fn part_one() {
    
    let player1_step_series = vec![
        6, 4, 2, 0, 8 // steps
    ];
    let player2_step_series = vec![
        5, 3, 1, 9, 7 // steps
    ];

    let mut step = 0;
    let mut player1_total = 0;
    let mut player2_total = 0;
    let mut player1_pos = PLAYER_1_INIT;
    let mut player2_pos = PLAYER_2_INIT;
    loop {
        let player1_new_pos = player1_pos + player1_step_series[step % 5];
        player1_pos = (player1_new_pos % 11) + player1_new_pos / 11;
        player1_total += player1_pos;
        if player1_total >= END_SCORE {
            break;
        }
        
        let player2_new_pos = player2_pos + player2_step_series[step % 5];
        player2_pos = (player2_new_pos % 11) + player2_new_pos / 11;
        player2_total += player2_pos;
        if player2_total >= END_SCORE {
            break;
        }
        step += 1;
    }
    let step = (step + 1) as u32;

    let min_score = min(player1_total, player2_total);
    
    let rolls = match player1_total >= END_SCORE {
        true => ((step * 2) - 1) * 3,
        false => step * 2 * 3
    };


    println!("Part 1: {} * {} = {}", min_score, rolls, (min_score * rolls));
}

fn part_two() {
    println!("Part 2: {}", "incomplete");
}

fn main() {
    part_one();
    part_two();

    println!("Done!");
}
