mod day1;
mod day2;
mod day3;
mod day4;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let filename = &args[2];

    println!("In file {}", filename);

    match day.parse::<u8>().unwrap_or(0) {
        1 => day1::solution(filename),
        2 => day2::solution(filename),
        3 => day3::solution(filename),
        4 => day4::solution(filename),
        _ => panic!("unknown day number")
    }

}
