use super::Day;

pub struct DayOne;

impl Day for DayOne {
    fn part_one(&self, input: &str) {
        let result: usize = input
            .split("\n\n")
            .map(|chunk| {
                chunk
                    .lines()
                    .map(|line| line.parse::<usize>().unwrap())
                    .sum()
            })
            .max()
            .unwrap();

        println!("{}", result);
    }

    fn part_two(&self, input: &str) {
        let mut cals = input
            .split("\n\n")
            .map(|chunk| {
                chunk
                    .lines()
                    .map(|line| line.parse::<usize>().unwrap())
                    .sum()
            })
            .collect::<Vec<_>>();

        cals.sort();
        cals.reverse();

        let result: usize = cals.iter().take(3).sum();

        println!("{}", result);
    }
}
