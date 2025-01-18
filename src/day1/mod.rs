mod utils;

use crate::day1::utils::generate_left_and_right_source_lists;

pub fn day1a() {
    //Variables
    let mut left_input_list: Vec<i64> = Vec::new();
    let mut right_input_list: Vec<i64> = Vec::new();
    let mut total_distance: i64 = 0;

    generate_left_and_right_source_lists(
        "./src/day1/input.txt",
        &mut left_input_list,
        &mut right_input_list,
    );
    left_input_list.sort();
    right_input_list.sort();

    for (_, value) in left_input_list
        .iter()
        .zip(right_input_list.iter())
        .enumerate()
    {
        total_distance = total_distance + (value.0 - value.1).abs();
    }
    println!("Day 1a: {}", total_distance);
}

pub fn day1b() {
    let mut left_input_list: Vec<i64> = Vec::new();
    let mut right_input_list: Vec<i64> = Vec::new();
    let mut similarity_score:i64 = 0;

    generate_left_and_right_source_lists(
        "./src/day1/input.txt",
        &mut left_input_list,
        &mut right_input_list,
    );

    for left_list_value in left_input_list{
        let mut similarity_tracker:i64 = 0;
        right_input_list.iter().for_each(|x|{
            if *x == left_list_value {
                similarity_tracker += 1;
            }
        });
        similarity_score = similarity_score + (left_list_value * similarity_tracker);
    }

    println!("Similarity score: {}", similarity_score);

}