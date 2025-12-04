use std::fs;

fn get_input() -> Vec<Vec<bool>> {
    let input = fs::read_to_string("input.txt").expect("File couldn't be read");
    let vec: Vec<Vec<bool>> = input
        .lines()
        .map(|s| s.chars().map(|x| x == '@').collect())
        .collect();
    dbg!(&vec);
    vec
}
//for lines
//for char
//[y-1..y+1][x-1..x+1]
//if postion is same
//continue
//            if input[i - 1..i + 1][j - 1+1]{}

fn get_neighbors(input: Vec<Vec<bool>>) -> u32 {
    let mut forks: u32 = 0;
    for i in 0..=input.len() - 1 {
        for j in 0..=input[i].len() - 1 {
            let mut count = 0;
            if !input[i][j] {
                continue;
            }

            for ii in i.saturating_sub(1)..=(i + 1).min(input.len() - 1) {
                for jj in j.saturating_sub(1)..=(j + 1).min(input[ii].len() - 1) {
                    if ii == i && jj == j {
                        continue;
                    }
                    if input[ii][jj] == true {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                forks += 1;
            }
        }
    }
    forks
}
fn main() {
    println!("{}", get_neighbors(get_input()));
}
