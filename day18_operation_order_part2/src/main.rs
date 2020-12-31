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

enum Operation {
    Addition,
    Multiplication,
}

fn solve_line(line: &str) -> u64 {
    sum_right_hand_side(0,line)
}

fn process_operation(left_hand_side: u64, remaining_line: &str) -> u64 {
    let operation = match remaining_line.chars().nth(0).unwrap() {
        '+' => Operation::Addition,
        '*' => Operation::Multiplication,
        _ => panic!("Unable to parse operation from {}", remaining_line)
    };

    let right_hand_side = &remaining_line[2..];
    match operation {
        Operation::Addition => sum_right_hand_side(left_hand_side, right_hand_side),
        Operation::Multiplication => {
            let right_hand_side_value = solve_line(right_hand_side);
            left_hand_side * right_hand_side_value
        }
    }
}

fn sum_right_hand_side(left_hand_side: u64, right_hand_side: &str) -> u64 {
    let (next_number, remaining_line) = match right_hand_side.chars().nth(0).unwrap() {
        '0'..='9' => parse_next_number(right_hand_side),
        '(' => parse_parentheses(right_hand_side),
        _ => panic!("Unexpected string: {}", right_hand_side)
    };

    let left_hand_side = left_hand_side + next_number;

    match remaining_line.len() {
        0 => left_hand_side,
        1 => panic!("We shouldn't have exactly 1 byte left"),
        _ => {
            let after_space = &remaining_line[1..];
            process_operation(left_hand_side, after_space)
        },
    }
}

fn parse_next_number(remaining_line: &str) -> (u64, &str) {
    if remaining_line.len() == 0 {
        panic!("We shouldn't try to parse an empty slice");
    }
    
    let mut index_past_number = 0;

    while index_past_number < remaining_line.len() {
        let byte = remaining_line.bytes().nth(index_past_number).unwrap();
        if (byte < '0' as u8) || (byte > '9' as u8) {
            break;
        }

        index_past_number += 1;
    }
    
    let number = &remaining_line[0..index_past_number];
    let number = number.parse().unwrap();

    let remaining_line = &remaining_line[index_past_number..];

    (number, remaining_line)
}

fn parse_parentheses(remaining_line: &str) -> (u64, &str) {
    let mut depth = 1;

    let mut index_of_closing_paren = 1;

    while index_of_closing_paren < remaining_line.len() {
        let byte = remaining_line.bytes().nth(index_of_closing_paren).unwrap();
        if byte == '(' as u8 {
            depth += 1;
        } else if byte == ')' as u8 {
            depth -= 1;
        }

        if depth == 0 {
            break;
        }

        index_of_closing_paren += 1;
    }

    if index_of_closing_paren == remaining_line.len() {
        panic!("Couldn't find closing paren");
    }

    let inside_of_parentheses = &remaining_line[1..index_of_closing_paren];
    let remaining_line = &remaining_line[index_of_closing_paren+1..];
    let value_of_parentheses = solve_line(inside_of_parentheses);

    (value_of_parentheses, remaining_line)
}
