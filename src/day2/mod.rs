mod utils;

use crate::day2::utils::{generate_source_list, safety_check};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn day2a() {
    let mut safety_status: Vec<bool> = Vec::new();
    let mut level_int_data: Vec<Vec<i64>> = Vec::new();
    let mut safe_report_count=0;
    generate_source_list(&mut level_int_data);

    safety_check(level_int_data, &mut safety_status);

    for status in safety_status {
        if status {
            safe_report_count+=1;
        }
    }
    println!("Safe Report Count: {}", safe_report_count);
}

fn day2b() {}
