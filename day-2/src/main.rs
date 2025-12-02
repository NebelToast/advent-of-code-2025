use std::fs;

fn get_input() -> Vec<(u64, u64)> {
    let input = fs::read_to_string("input.txt").expect("File couldn't be read");
    input
        .trim()
        .split(",")
        .map(|s| {
            let (start, end) = s.split_once("-").unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect()
}

fn is_mirrorable(number: u64) -> bool {
    let number_str = number.to_string();

    let (first_half, second_half) = number_str.split_at(number_str.len() / 2);
    first_half == second_half
}

fn sum_of_invalid(value: Vec<(u64, u64)>) -> u64 {
    let mut sum = 0;
    for (val1, val2) in value {
        for numbers in val1..=val2 {
            if is_mirrorable(numbers) {
                sum += numbers
            }
        }
    }
    sum
}
fn main() {
    println!("{}", sum_of_invalid(get_input()));
}
