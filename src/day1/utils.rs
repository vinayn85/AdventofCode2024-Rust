use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_input_data(input_file_path: &str) -> File {
    let input_file_open_result = File::open(input_file_path);
    match input_file_open_result {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file {}: {}", input_file_path, e);
            std::process::exit(1);
        }
    }
}

pub fn generate_left_and_right_source_lists<'a>(
    input_file_path: &str,
    vec1: &'a mut Vec<i64>,
    vec2: &'a mut Vec<i64>,
) -> (&'a mut Vec<i64>, &'a mut Vec<i64>) {
    let input_file_buffered_reader = BufReader::new(load_input_data(input_file_path));
    input_file_buffered_reader.lines().for_each(|line| {
        let tmp_line = line.unwrap();
        let line_split_values = tmp_line.split_whitespace().collect::<Vec<&str>>();
        (*vec1).push(
            line_split_values
                .get(0)
                .unwrap()
                .trim()
                .to_string()
                .parse::<i64>()
                .unwrap(),
        );
        (*vec2).push(
            line_split_values
                .get(1)
                .unwrap()
                .trim()
                .to_string()
                .parse::<i64>()
                .unwrap(),
        );
    });
    (vec1, vec2)
}
