use std::vec::Vec;

use rayon::prelude::*;

#[allow(unused_imports)]
use day18_operation_order_common::{SAMPLE_DATA_1, SAMPLE_DATA_2, SAMPLE_DATA_3, REAL_DATA};

fn main() {
    let result = do_work(&REAL_DATA);
    println!("{}", result);
}

fn do_work(data: &[&str]) -> u64 {
    let sum = data.par_iter().map(|line| solve_line(line)).sum();

    sum
}

#[derive(Copy, Clone, PartialEq)]
enum Operation {
    Addition,
    Multiplication,
    Exponent,
    Parentheses,
    EndOfLine,
}

enum Token {
    Space,
    Number(u64),
    LeftParenthesis,
    RightParenthesis,
    Operator(Operation),
}

fn solve_line(line: &str) -> u64 {
    let mut values = Vec::new();
    let mut operators = Vec::new();
    let mut remaining_line = line;

    while remaining_line.len() > 0 {
        let parsed = parse_next_token(remaining_line);
        let next_token = parsed.0;
        remaining_line = parsed.1;

        match next_token {
            Token::Space => (),
            Token::Number(value) => values.push(value),
            Token::LeftParenthesis => operators.push(Operation::Parentheses),
            Token::RightParenthesis => process_operators(&mut values, &mut operators, Operation::Parentheses),
            Token::Operator(operator) => {
                process_operators(&mut values, &mut operators, operator);
                operators.push(operator);
            }
        }
    }

    process_operators(&mut values, &mut operators, Operation::EndOfLine);

    if values.len() != 1 {
        panic!("Unexpected number of values");
    }

    if operators.len() != 0 {
        panic!("Unexpected number of operators");
    }

    *values.get(0).unwrap()
}

fn parse_next_token(line: &str) -> (Token, &str) {
    match line.chars().nth(0).unwrap() {
        '0'..='9' => parse_number(line),
        '(' => parse_single_char_token(Token::LeftParenthesis, line),
        ')' => parse_single_char_token(Token::RightParenthesis, line),
        '+' => parse_single_char_token(Token::Operator(Operation::Addition), line),
        '*' => parse_single_char_token(Token::Operator(Operation::Multiplication), line),
        '^' => parse_single_char_token(Token::Operator(Operation::Exponent), line),
        ' ' => parse_single_char_token(Token::Space, line),
        _ => panic!("Unable to parse token from {}", line)
    }
}

fn parse_number(line: &str) -> (Token, &str) {
    if line.len() == 0 {
        panic!("Unable to parse_number on an empty line");
    }

    let mut index_past_number = 0;
    while index_past_number < line.len() {
        let byte = line.bytes().nth(index_past_number).unwrap();
        if (byte < '0' as u8) || (byte > '9' as u8) {
            break;
        }

        index_past_number += 1;
    }

    let number = &line[0..index_past_number];
    let number = number.parse().unwrap();

    let remaining_line = &line[index_past_number..];

    (Token::Number(number), remaining_line)
}

fn parse_single_char_token(token: Token, line: &str) -> (Token, &str) {
    (token, &line[1..])
}

fn process_operators(values: &mut Vec::<u64>, operators: &mut Vec::<Operation>, reason: Operation) {
    while operators.len() != 0 {
        let operator = operators.pop().unwrap();
        match operator {
            Operation::Addition => {
                if reason != Operation::Exponent {
                    let right_hand_side = values.pop().unwrap();
                    let left_hand_side = values.pop().unwrap();

                    let sum = left_hand_side + right_hand_side;
                    values.push(sum);
                } else {
                    operators.push(operator);
                    break;
                }
            },
            Operation::Multiplication => {
                if (reason != Operation::Exponent) && 
                    (reason != Operation::Addition) {
                    let right_hand_side = values.pop().unwrap();
                    let left_hand_side = values.pop().unwrap();

                    let product = left_hand_side * right_hand_side;
                    values.push(product);
                } else {
                    operators.push(operator);
                    break;
                }
            },
            Operation::Exponent => {
                let right_hand_side = values.pop().unwrap() as u32;
                let left_hand_side = values.pop().unwrap();

                let power = left_hand_side.pow(right_hand_side);
                values.push(power);
            },
            Operation::Parentheses => {
                if reason != Operation::Parentheses {
                    operators.push(operator);
                }
                break;
            },
            Operation::EndOfLine => panic!("EndOfLine shouldn't be in the operations queue")
        };
    }
}
