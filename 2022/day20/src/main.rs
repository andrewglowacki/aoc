use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

struct Number {
    orig_pos: usize,
    value: i64
}

impl Number {
    fn new(orig_pos: usize, value: i64) -> Number {
        Number {
            orig_pos,
            value
        }
    }
}

struct Numbers {
    numbers: Vec<Number>
}

impl Numbers {
    fn from_file(file_name: &str) -> Numbers {
        let mut numbers = Vec::new();

        get_file_lines(file_name)
            .flat_map(|line| line.ok())
            .map(|line| line.parse::<i64>().unwrap())
            .for_each(|number| numbers.push(Number::new(numbers.len(), number)));

        Numbers { numbers }
    }

    fn from_vec(values: Vec::<i64>) -> Numbers {
        let mut numbers = Vec::new();
        
        values.into_iter()
            .for_each(|number| numbers.push(Number::new(numbers.len(), number)));
        
        Numbers { numbers }
    }

    fn find_orig_index(&self, index: usize) -> usize {
        for j in 0..self.numbers.len() {
            if self.numbers[j].orig_pos == index {
                return j;
            }
        }
        panic!("Index not found: {}", index);
    }

    fn decrypt_orig_index(&mut self, index: usize) {
        let count = self.numbers.len();
        let cur_pos = self.find_orig_index(index);
        let value = self.numbers[cur_pos].value;
        let mut new_pos = cur_pos as i64 + value;
        if new_pos == 0 {
            new_pos = count as i64 - 1;
        } else if new_pos < 0 {
            new_pos = -1 * new_pos;
            new_pos = new_pos % (count as i64 - 1);
            new_pos = (count as i64 - 1) - new_pos;
            if new_pos == 0 {
                new_pos = count as i64 - 1;
            }
        } else {
            if new_pos >= count as i64 {
                new_pos = new_pos % (count as i64 - 1);
                if new_pos == 0 {
                    new_pos = count as i64 - 1;
                }
            }
        }
        let new_pos = new_pos as usize;

        if new_pos > cur_pos {
            for j in cur_pos..new_pos {
                self.numbers.swap(j, j + 1);
            }
        } else if new_pos < cur_pos {
            let len = cur_pos - new_pos;
            for j in 0..len {
                let j = cur_pos - j;
                self.numbers.swap(j, j - 1);
            }
        }
    }

    fn decrypt(&mut self) {
        let count = self.numbers.len();

        for i in 0..count {
            self.decrypt_orig_index(i);
        }
    }

    fn sum_grove_coordinates(&self) -> i64 {
        let mut zero_pos = -1;
        for i in 0..self.numbers.len() {
            if self.numbers[i].value == 0 {
                zero_pos = i as i64;
                break;
            }
        }
        let zero_pos = zero_pos as usize;
        self.numbers[(zero_pos + 1000) % self.numbers.len()].value +
        self.numbers[(zero_pos + 2000) % self.numbers.len()].value + 
        self.numbers[(zero_pos + 3000) % self.numbers.len()].value
    }

    fn get_values(&self) -> Vec<i64> {
        self.numbers.iter()
            .map(|number| number.value)
            .collect::<Vec<i64>>()
    }

}

fn part_one(file_name: &str) {
    let mut numbers = Numbers::from_file(file_name);
    
    numbers.decrypt();

    let sum = numbers.sum_grove_coordinates();
    
    println!("Part 1: {}", sum);
}

fn part_two(file_name: &str) {
    let mut numbers = Numbers::from_file(file_name);

    numbers.numbers.iter_mut().for_each(|number| {
        number.value *= 811589153;
    });
    
    for _ in 0..10 {
        numbers.decrypt();
    }
    
    let sum = numbers.sum_grove_coordinates();
    
    println!("Part 2: {}", sum);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

    println!("Done!");
}


#[cfg(test)]
mod tests {
    use crate::Numbers;

    #[test]
    fn move_forward_by_one_positive() {
        let mut numbers = Numbers::from_vec(vec![1, -2, -3, 4, -5]);
        
        numbers.decrypt_orig_index(0);
        assert_eq!(vec![-2, 1, -3, 4, -5], numbers.get_values());

        numbers.decrypt_orig_index(0);
        assert_eq!(vec![-2, -3, 1, 4, -5], numbers.get_values());

        numbers.decrypt_orig_index(0);
        assert_eq!(vec![-2, -3, 4, 1, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(0);
        assert_eq!(vec![-2, -3, 4, -5, 1], numbers.get_values());
        
        numbers.decrypt_orig_index(0);
        assert_eq!(vec![-2, 1, -3, 4, -5], numbers.get_values());
    }
    
    #[test]
    fn move_by_two_positive() {
        let mut numbers = Numbers::from_vec(vec![1, 2, -3, 4, -5]);
        
        numbers.decrypt_orig_index(1);
        assert_eq!(vec![1, -3, 4, 2, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(1);
        assert_eq!(vec![1, 2, -3, 4, -5], numbers.get_values());
    }
    
    #[test]
    fn move_by_two_positive_edge() {
        let mut numbers = Numbers::from_vec(vec![2, 1, -3, 4, -5]);
        
        numbers.decrypt_orig_index(0);
        assert_eq!(vec![1, -3, 2, 4, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(0);
        assert_eq!(vec![1, -3, 4, -5, 2], numbers.get_values());
    }
    
    #[test]
    fn move_by_three_positive() {
        let mut numbers = Numbers::from_vec(vec![1, 3, -3, 4, -5]);
        
        numbers.decrypt_orig_index(1);
        assert_eq!(vec![1, -3, 4, -5, 3], numbers.get_values());
        
        numbers.decrypt_orig_index(1);
        assert_eq!(vec![1, -3, 4, 3, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(1);
        assert_eq!(vec![1, -3, 3, 4, -5], numbers.get_values());
    }
    
    #[test]
    fn move_by_positive_large() {
        let mut numbers = Numbers::from_vec(vec![1, 13, -3, 4, -5]);
        
        numbers.decrypt_orig_index(1);
        assert_eq!(vec![1, -3, 13, 4, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(1);
        assert_eq!(vec![1, -3, 4, 13, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(1);
        assert_eq!(vec![1, -3, 4, -5, 13], numbers.get_values());

        numbers.decrypt_orig_index(1);
        assert_eq!(vec![1, 13, -3, 4, -5], numbers.get_values());
    }

    #[test]
    fn move_by_negative_one() {
        let mut numbers = Numbers::from_vec(vec![-1, 2, -3, 4, -5]);
        
        numbers.decrypt_orig_index(0);
        assert_eq!(vec![2, -3, 4, -1, -5], numbers.get_values());

        numbers.decrypt_orig_index(0);
        assert_eq!(vec![2, -3, -1, 4, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(0);
        assert_eq!(vec![2, -1, -3, 4, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(0);
        assert_eq!(vec![2, -3, 4, -5, -1], numbers.get_values());
        
        numbers.decrypt_orig_index(0);
        assert_eq!(vec![2, -3, 4, -1, -5], numbers.get_values());
    }
    
    #[test]
    fn move_by_negative_two() {
        let mut numbers = Numbers::from_vec(vec![1, -2, -3, 4, -5]);
        
        numbers.decrypt_orig_index(1);
        assert_eq!(vec![1, -3, 4, -2, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(1);
        assert_eq!(vec![1, -2, -3, 4, -5], numbers.get_values());
    }
    
    #[test]
    fn move_by_negative_two_edge() {
        let mut numbers = Numbers::from_vec(vec![-2, 1, -3, 4, -5]);
        
        numbers.decrypt_orig_index(0);
        assert_eq!(vec![1, -3, -2, 4, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(0);
        assert_eq!(vec![1, -3, 4, -5, -2], numbers.get_values());
        
        numbers.decrypt_orig_index(0);
        assert_eq!(vec![1, -3, -2, 4, -5], numbers.get_values());
    }
    
    #[test]
    fn move_by_negative_three() {
        let mut numbers = Numbers::from_vec(vec![1, 2, -3, 4, -5]);
        
        numbers.decrypt_orig_index(2);
        assert_eq!(vec![1, 2, 4, -3, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(2);
        assert_eq!(vec![1, 2, 4, -5, -3], numbers.get_values());
        
        numbers.decrypt_orig_index(2);
        assert_eq!(vec![1, -3, 2, 4, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(2);
        assert_eq!(vec![1, 2, -3, 4, -5], numbers.get_values());
    }
    
    #[test]
    fn move_by_negative_large() {
        let mut numbers = Numbers::from_vec(vec![1, 2, -13, 4, -5]);
        
        numbers.decrypt_orig_index(2);
        assert_eq!(vec![1, -13, 2, 4, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(2);
        assert_eq!(vec![1, 2, 4, -5, -13], numbers.get_values());
        
        numbers.decrypt_orig_index(2);
        assert_eq!(vec![1, 2, 4, -13, -5], numbers.get_values());
        
        numbers.decrypt_orig_index(2);
        assert_eq!(vec![1, 2, -13, 4, -5], numbers.get_values());
    }
}