use regex::Regex;

pub fn calculate_mul_expression_result(expr: &String) -> i64 {
    let (mut a, mut b): (i64, i64) = (0, 0);
    let get_operands_regex = Regex::new("\\d{1,5}").unwrap();
    get_operands_regex
        .find_iter(expr)
        .enumerate()
        .for_each(|(i, o)| {
            if i == 0 {
                a = o.as_str().parse::<i64>().unwrap();
            } else if i == 1 {
                b = o.as_str().parse::<i64>().unwrap();
            }
        });
    a * b
}
