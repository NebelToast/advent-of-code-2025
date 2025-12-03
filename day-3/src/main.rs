use std::fs;

fn get_input() -> Vec<Vec<u8>> {
    let input = fs::read_to_string("input.txt").expect("File couldn't be read");
    let vec: Vec<Vec<u8>> = input
        .lines()
        .map(|s| s.bytes().map(|b| b - b'0').collect())
        .collect();
    vec
}

fn get_biggest_battery(batteries: Vec<Vec<u8>>) -> Vec<u8> {
    let mut battery_lines = vec![];
    for line in batteries {
        let mut first_digit: (usize, &u8) = (0, &0);
        let mut second_digit: (usize, &u8) = (0, &0);
        for (i, digit) in line.iter().enumerate() {
            if digit > &second_digit.1 {
                if digit > &first_digit.1 && i != line.len() - 1 {
                    first_digit = (i, digit);
                    continue;
                }
            }
        }
        let remaining = &line[first_digit.0 + 1..];
        for digit in remaining {
            if digit > second_digit.1 {
                second_digit.1 = digit;
            }
        }

        battery_lines.push((first_digit.1 * 10) + second_digit.1);
    }
    battery_lines
}

fn main() {
    let input = get_input();
    let batteries = get_biggest_battery(input);

    println!("{}", batteries.iter().map(|x| *x as u16).sum::<u16>())
}
