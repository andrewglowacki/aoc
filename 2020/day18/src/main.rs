use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::slice::Iter;
use std::iter::Peekable;

fn get_file_lines(file_name: &str) -> Lines<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Number(i32),
    Open,
    Close, 
    Add,
    Multiply
}

fn do_math(operator: Token, left: i64, right: i64) -> i64 {
    match operator {
        Token::Add => left + right,
        Token::Multiply => left * right,
        _ => panic!("Invalid operator {:?}", operator)
    }
}

fn evaluate(tokens: Vec<Token>) -> i64 {
    let mut scope = Vec::<(i64, Token)>::new();
    let mut value = 0;
    let mut operator = Token::Add;
    for token in tokens {
        match token {
            Token::Open => {
                scope.push((value, operator));
                value = 0;
                operator = Token::Add;
            },
            Token::Close => {
                let (prev_value, prev_operator) = scope.pop().unwrap();
                operator = prev_operator;
                value = do_math(operator, value, prev_value);
            },
            Token::Add | Token::Multiply => operator = token,
            Token::Number(x) => value = do_math(operator, value, x as i64)
        };
    }
    value
}
fn parse_tokens(string: String) -> Vec<Token> {
    let chars = string.chars().collect::<Vec<char>>();
    let mut chars = chars.iter().peekable();
    
    let mut tokens = vec![];
    while let Some(token) = next_token(&mut chars) {
        tokens.push(token);
    };
    return tokens;
}
fn skip_whitespace(chars: &mut Peekable<Iter<char>>) {
    while let Some(c) = chars.peek() {
        if **c != ' ' {
            return;
        }
        chars.next();
    }
}
fn read_number(chars: &mut Peekable<Iter<char>>) -> i32 {
    let mut number_str = String::new();
    loop {
        if let Some(c) = chars.peek() {
            if !c.is_numeric() {
                break;
            }
            number_str.push(**c);
        } else {
            break;
        }
        chars.next();
    }
    number_str.parse::<i32>().unwrap()
}
fn next_token(chars: &mut Peekable<Iter<char>>) -> Option<Token> {
    skip_whitespace(chars);
    if let Some(c) = chars.peek() {
        let token = match c {
            '(' => Token::Open,
            ')' => Token::Close,
            '+' => Token::Add,
            '*' => Token::Multiply,
            _ => Token::Number(read_number(chars))
        };
        match token {
            Token::Number(_) => (),
            _ => { chars.next(); }
        }
        skip_whitespace(chars);
        Some(token)
    } else {
        None
    }
}

fn apply_part_two_rules(tokens: Vec<Token>) -> Vec<Token> {
    let mut new_tokens = Vec::new();
    let mut scope = Vec::<(usize, Token, bool)>::new();
    let mut operator = Token::Multiply;
    let mut last_start = 0;
    let mut started = false;
    for token in tokens {
        match token {
            Token::Open => {
                last_start = new_tokens.len();
                scope.push((last_start, operator, started));
                operator = Token::Multiply;
                last_start = new_tokens.len() + 1;
                started = false;
            },
            Token::Close => {
                if started {
                    new_tokens.push(Token::Close);
                }
                let (prev_scope_start, prev_operator, prev_started) = scope.pop().unwrap();
                last_start = prev_scope_start;
                operator = prev_operator;
                started = prev_started;
            },
            Token::Add => {
                if operator != Token::Add {
                    new_tokens.insert(last_start, Token::Open);
                    started = true;
                }
                operator = Token::Add;
            },
            Token::Multiply => {
                if operator != Token::Multiply {
                    new_tokens.push(Token::Close);
                    started = false;
                }
                operator = Token::Multiply;
            }
            _ => {
                if !started {
                    last_start = new_tokens.len();
                }
            }
        };
        new_tokens.push(token);
    }
    if started {
        new_tokens.push(Token::Close);
    }

    new_tokens
}

fn test_input(file_name: &str, print_each: bool) {
    let results = get_file_lines(file_name)
        .flat_map(|line| line.ok())
        .map(|line| parse_tokens(line))
        .map(|tokens| apply_part_two_rules(tokens))
        .map(|tokens| evaluate(tokens));
    
    if print_each {
        println!("Results for {}:", file_name);
        results.for_each(|result| println!("{}", result));
    } else {
        println!("For {}, Sum of all: {}", file_name, results.sum::<i64>());
    }
}

fn main() {
    test_input("sample.txt", true);
    test_input("input.txt", false);
}
