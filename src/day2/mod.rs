mod utils;

use crate::day2::utils::{do_safety_check, generate_source_list};

pub fn day2a() {
    let mut safety_status: Vec<bool> = Vec::new();
    let mut level_int_data: Vec<Vec<i64>> = Vec::new();
    let mut safe_report_count = 0;
    generate_source_list(&mut level_int_data);

    do_safety_check(level_int_data, &mut safety_status);

    for status in safety_status {
        if status {
            safe_report_count += 1;
        }
    }
    println!("Safe Report Count: {}", safe_report_count);
}

pub fn day2b() {
    let mut safety_status: Vec<bool> = Vec::new();
    let mut level_int_data: Vec<Vec<i64>> = Vec::new();
    let mut safe_report_count = 0;
    generate_source_list(&mut level_int_data);

    do_safety_check(level_int_data, &mut safety_status);
    for status in safety_status {
        if status {
            safe_report_count += 1;
        }
    }
    println!("Safe Report Count: {}", safe_report_count);
}
