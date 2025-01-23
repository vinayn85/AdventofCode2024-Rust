mod utils;

use crate::day3::utils::calculate_mul_expression_result;
use regex::Regex;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

pub fn day3a() {
    let mut multiply_expr_sum: i64 = 0;
    let mut mul_expression_vec: Vec<String> = Vec::new();
    let mul_expression_regex = Regex::new("mul\\(\\d{1,5},\\d{1,5}\\)").unwrap();
    let mut input_file = File::open("src/day3/input.txt").unwrap();

    let mut input_data = String::new();
    let file_read_result = input_file.read_to_string(&mut input_data);

    if file_read_result.is_err() {
        println!("Error reading file: {}", file_read_result.unwrap_err());
        exit(1)
    }

    mul_expression_regex
        .find_iter(&input_data)
        .for_each(|mul_expr_match| {
            mul_expression_vec.push(mul_expr_match.as_str().to_string());
        });

    mul_expression_vec.iter().for_each(|mul_expr| {
        multiply_expr_sum += calculate_mul_expression_result(&mul_expr);
    });

    println!("Sum of mul expressions: {}", multiply_expr_sum);
}
pub fn day3b() {
    let mut multiply_expr_sum: i64 = 0;
    let mut do_mul_expression_vec: Vec<String> = Vec::new();
    let mut mul_expression_vec: Vec<String> = Vec::new();

    let mul_expression_regex = Regex::new("mul\\(\\d{1,5},\\d{1,5}\\)").unwrap();

    let mut input_file = File::open("src/day3/input.txt").unwrap();

    let mut input_data = String::new();
    let file_read_result = input_file.read_to_string(&mut input_data);

    if file_read_result.is_err() {
        println!("Error reading file: {}", file_read_result.unwrap_err());
        exit(1)
    }

    let mut cleaned_input_data = input_data.replace("\n", " ");
    cleaned_input_data = cleaned_input_data.replace("don't()", "\ndon't()");
    cleaned_input_data = cleaned_input_data.replace("do()", "\ndo()");

    for line in cleaned_input_data.lines() {
        if !line.starts_with("don't()") {
            do_mul_expression_vec.push(line.to_string());
        }
    }

    do_mul_expression_vec.iter().for_each(|do_mul_expr| {
        mul_expression_regex
            .find_iter(do_mul_expr)
            .for_each(|mul_expr_match| {
                mul_expression_vec.push(mul_expr_match.as_str().to_string());
            })
    });

    mul_expression_vec.iter().for_each(|mul_mul_expr| {
        multiply_expr_sum += calculate_mul_expression_result(&mul_mul_expr);
    });

    println!("Sum of mul expressions: {}", multiply_expr_sum);
}
