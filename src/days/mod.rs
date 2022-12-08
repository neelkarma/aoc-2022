mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

pub trait Day {
    fn part_one(&self, input: &str);
    fn part_two(&self, input: &str);
}

pub fn get_solution(day: u8) -> Box<dyn Day> {
    match day {
        1 => Box::new(day1::DayOne),
        2 => Box::new(day2::DayTwo),
        3 => Box::new(day3::DayThree),
        4 => Box::new(day4::DayFour),
        5 => Box::new(day5::DayFive),
        6 => Box::new(day6::DaySix),
        7 => Box::new(day7::DaySeven),
        8 => Box::new(day8::DayEight),
        _ => panic!("Invalid day provided"),
    }
}
