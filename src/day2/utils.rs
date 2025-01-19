use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn generate_source_list(source_int_vec: &mut Vec<Vec<i64>>) {
    let levels_string_data = load_vec_string_values();
    for current_level in levels_string_data {
        let temp_line_split: Vec<&str> = current_level.split(" ").collect();
        let mut temp_int_vec: Vec<i64> = Vec::new();
        for elem in temp_line_split {
            let temp_int: i64 = elem.parse::<i64>().unwrap();
            temp_int_vec.push(temp_int);
        }
        source_int_vec.push(temp_int_vec);
    }
}

fn load_vec_string_values() -> Vec<String> {
    let mut levels_string_data: Vec<String> = Vec::new();
    let input_data = File::open(Path::new("src/day2/input.txt"));
    let input_data_buffered_reader = BufReader::new(input_data.unwrap());
    for line in input_data_buffered_reader.lines() {
        let line = line.unwrap();
        levels_string_data.push(line);
    }
    levels_string_data
}
pub fn do_safety_check(vec: Vec<Vec<i64>>, vec1: &mut Vec<bool>) {
    for current_level in vec {
        let is_increasing = is_increasing_levels(&current_level);
        let is_decreasing = is_decreasing_levels(&current_level);
        let is_allowable_level_diff = is_allowable_level_diff(&current_level);

        if (is_increasing || is_decreasing) && is_allowable_level_diff {
            vec1.push(true);
        } else {
            vec1.push(do_safety_check_with_damper(&current_level));
        }
    }
}

pub fn do_safety_check_with_damper(vec: &Vec<i64>) -> bool {
    let temp_vec_as_array = vec.as_slice();
    for i in 0..temp_vec_as_array.len() {
        let part1 = &temp_vec_as_array[..i];
        let part2 = &temp_vec_as_array[i + 1..];

        let temp_damped_level = [part1, part2].concat();

        if (is_increasing_levels(&temp_damped_level) || is_decreasing_levels(&temp_damped_level))
            && is_allowable_level_diff(&temp_damped_level)
        {
            return true;
        } else {
            continue;
        }
    }
    false
}
fn is_increasing_levels(vec: &Vec<i64>) -> bool {
    vec.is_sorted_by(|a, b| a < b)
}
fn is_decreasing_levels(vec: &Vec<i64>) -> bool {
    vec.is_sorted_by(|a, b| a > b)
}

fn is_allowable_level_diff(vec: &Vec<i64>) -> bool {
    for (index, current_value) in vec.iter().enumerate() {
        if index == vec.len() - 1 {
            break;
        }

        if (*current_value - vec[index + 1]).abs() >= 1
            && (*current_value - vec[index + 1]).abs() <= 3
        {
            continue;
        } else {
            return false;
        }
    }
    true
}
