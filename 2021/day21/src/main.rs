
use std::cmp::max;
use std::collections::HashMap;
use std::cmp::min;

const PLAYER_1_INIT: u32 = 2;
const PLAYER_2_INIT: u32 = 8;
const END_SCORE_PT1: u32 = 1000;
const END_SCORE_PT2: usize = 21;

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
        if player1_total >= END_SCORE_PT1 {
            break;
        }
        
        let player2_new_pos = player2_pos + player2_step_series[step % 5];
        player2_pos = (player2_new_pos % 11) + player2_new_pos / 11;
        player2_total += player2_pos;
        if player2_total >= END_SCORE_PT1 {
            break;
        }
        step += 1;
    }
    let step = (step + 1) as u32;

    let min_score = min(player1_total, player2_total);
    
    let rolls = match player1_total >= END_SCORE_PT1 {
        true => ((step * 2) - 1) * 3,
        false => step * 2 * 3
    };


    println!("Part 1: {} * {} = {}", min_score, rolls, (min_score * rolls));
}

struct Quantum {
    ways_to_roll: Vec<u64>,
    paths_to_end: HashMap<(usize, usize, usize, usize, bool), (u64, u64)>
}

impl Quantum {
    fn new() -> Quantum {
        // can only roll a number from 3 to 9 with 3 x 3-sided-die
        let ways_to_roll = vec![
            0, // 0
            0, // 1
            0, // 2
            1, // 3 - 1+1+1
            3, // 4 - 1+1+2
            6, // 5 - 1+1+3, 1+2+2
            7, // 6 - 2+2+2, 1+2+3
            6, // 7 - 1+3+3, 3+2+2
            3, // 8 - 2+3+3
            1, // 9 - 3+3+3
        ];

        Quantum { 
            ways_to_roll, 
            paths_to_end: HashMap::new()
        }
    }

    fn get_win_count(&mut self, p1_cur_tile: usize, p1_cur_sum: usize, p2_cur_tile: usize, p2_cur_sum: usize, player1_turn: bool) -> (u64, u64) {
        if let Some(count) = self.paths_to_end.get(&(p1_cur_tile, p1_cur_sum, p2_cur_tile, p2_cur_sum, player1_turn)) {
            return *count;
        } else if player1_turn {
            if p2_cur_sum >= END_SCORE_PT2 {
                return (0, 1);
            }
        } else {
            if p1_cur_sum >= END_SCORE_PT2 {
                return (1, 0);
            }
        }

        let mut p1_total = 0;
        let mut p2_total = 0;

        // holy local variables batman, ewww
        if player1_turn {
            for roll in 3..10 {
                let to_tile = p1_cur_tile + roll;
                let to_tile = (to_tile % 11) + (to_tile / 11);
                let to_sum = p1_cur_sum + to_tile;
                let ways_to_roll = self.ways_to_roll[roll];

                let (p1_wins, p2_wins) = self.get_win_count(to_tile, to_sum, p2_cur_tile, p2_cur_sum, false);
                p1_total += p1_wins * ways_to_roll;
                p2_total += p2_wins * ways_to_roll;
            }
        } else {
            for roll in 3..10 {
                let to_tile = p2_cur_tile + roll;
                let to_tile = (to_tile % 11) + (to_tile / 11);
                let to_sum = p2_cur_sum + to_tile;
                let ways_to_roll = self.ways_to_roll[roll];
    
                let (p1_wins, p2_wins) = self.get_win_count(p1_cur_tile, p1_cur_sum, to_tile, to_sum, true);
                p1_total += p1_wins * ways_to_roll;
                p2_total += p2_wins * ways_to_roll;
            }
        }

        let count = (p1_total, p2_total);
        self.paths_to_end.insert((p1_cur_tile, p1_cur_sum, p2_cur_tile, p2_cur_sum, player1_turn), count);
        count
    }
}

fn part_two() {
    let mut quantum = Quantum::new();

    let (player1, player2) = quantum.get_win_count(PLAYER_1_INIT as usize, 0, PLAYER_2_INIT as usize, 0, true);

    println!("Part 2: max({}, {}) = {}", player1, player2, max(player1, player2));
}

fn main() {
    part_one();
    part_two();

    println!("Done!");
}
