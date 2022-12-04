use super::Day;

pub struct DayOne;

impl Day for DayOne {
    fn part_one(&self, input: &str) {
        let result: u32 = input
            .split("\n\n")
            .map(|chunk| chunk.lines().map(|line| line.parse::<u32>().unwrap()).sum())
            .max()
            .unwrap();

        println!("{}", result);
    }

    fn part_two(&self, input: &str) {
        let mut cals = input
            .split("\n\n")
            .map(|chunk| chunk.lines().map(|line| line.parse::<u32>().unwrap()).sum())
            .collect::<Vec<_>>();

        cals.sort();
        cals.reverse();

        let result: u32 = cals.iter().take(3).sum();

        println!("{}", result);
    }
}
