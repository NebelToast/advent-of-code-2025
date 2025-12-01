use std::fs;

fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input.txt").expect("File couldn't be read");
    let vec: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    vec
}

fn get_values(input: Vec<String>) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();
    for elements in input {
        let value = if elements.starts_with('L') {
            -elements[1..].parse::<i32>().unwrap()
        } else {
            elements[1..].parse::<i32>().unwrap()
        };
        values.push(value);
    }
    values
}

fn get_zeros(values: Vec<i32>) -> u32 {
    let mut dial = 50;
    let mut zeros = 0;
    for value in values {
        dial = (dial + value) % 100;
        if dial == 0 {
            zeros += 1;
        }
    }
    zeros
}


fn get_zeros_part_two(values: Vec<i32>) -> u32 {
    let mut dial = 50;
    let mut zeros = 0;
    
    for value in values {
        let step = if value < 0 { -1 } else { 1 };
        for _ in 0..value.abs() {
            dial = (dial + step) % 100;
            if dial == 0 {
                zeros += 1;
            }
        }
    }

    zeros
}
fn main() {
    let input = get_input();
    let values = get_values(input);

    println!("Part one: {}", get_zeros(values.clone()));

    println!("Part two: {}", get_zeros_part_two(values));
}
