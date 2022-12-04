use std::{env::args, fs::read_to_string};

use days::get_solution;
mod days;

fn main() {
    let day = args().nth(1).expect("No day provided");
    let day = day.parse::<u8>().expect("Invalid day provided");
    let sol = get_solution(day);

    let input = read_to_string(format!("inputs/day{}.txt", day)).expect("Failed to read input");

    println!("--- PART ONE ---");
    sol.part_one(&input);

    println!("--- PART TWO ---");
    sol.part_two(&input);
}
