use std::fs;

fn get_input() -> Vec<(u64, u64)> {
    let input = fs::read_to_string("input.txt").expect("File couldn't be read");
    let vec: Vec<String> = input.trim().split(",").map(|s| s.to_string()).collect();

    vec.iter()
        .map(|x| {
            let val = x.split_once("-").unwrap();
            (val.0.parse::<u64>().unwrap(), val.1.parse::<u64>().unwrap())
        })
        .collect()
}
fn is_mirrorable(number: u64) -> bool {
    let number_str = number.to_string();

    let (first_half, second_half) = number_str.split_at(number_str.len() / 2);
    first_half == second_half
}

fn value_to_numbers(value: Vec<(u64, u64)>) -> Vec<u64> {
    let mut numbers_vec: Vec<u64> = Vec::new();
    for (val1, val2) in value {
        for numbers in val1..=val2 {
            if is_mirrorable(numbers) {
                dbg!(numbers);
                numbers_vec.push(numbers);
            }
        }
    }
    numbers_vec
}
fn main() {
    println!("{}", value_to_numbers(get_input()).iter().sum::<u64>());
}
