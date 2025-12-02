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

fn is_mirrorable(number_str: &String) -> bool {
    if number_str.len() % 2 != 0 {
        return false;
    }

    let (first_half, second_half) = number_str.split_at(number_str.len() / 2);
    first_half == second_half
}

fn is_invalid_part_two(number_str: &String) -> bool {
    let len = number_str.len();

    for chunk_size in 1..len {
        if len % chunk_size != 0 {
            continue;
        }
        let pattern = &number_str[..chunk_size];
        let repeated = pattern.repeat(len / chunk_size);
        if repeated == *number_str {
            return true;
        }
    }
    false
}
fn sum_of_invalid(value: Vec<(u64, u64)>) -> (u64, u64) {
    let mut sum_part_one = 0;
    let mut sum_part_two = 0;
    for (val1, val2) in value {
        for number in val1..=val2 {
            let number_str = number.to_string();
            if is_invalid_part_two(&number_str) {
                sum_part_two += number
            }
            if is_mirrorable(&number_str) {
                sum_part_one += number
            }
        }
    }
    (sum_part_one, sum_part_two)
}
fn main() {
    println!("Part one: {}", sum_of_invalid(get_input()).0);

    println!("Part two: {}", sum_of_invalid(get_input()).1);
}
