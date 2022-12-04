use super::Day;

pub struct DayFour;

impl Day for DayFour {
    fn part_one(&self, input: &str) {
        let result = input
            .lines()
            .filter(|line| {
                let collected: Vec<_> = line
                    .split(",")
                    .map(|part| {
                        let collected: Vec<_> = part
                            .split("-")
                            .map(|seg| seg.parse::<usize>().unwrap())
                            .collect();
                        (collected[0], collected[1])
                    })
                    .collect();
                let pairs = (collected[0], collected[1]);

                (pairs.0 .0 <= pairs.1 .0 && pairs.0 .1 >= pairs.1 .1)
                    || (pairs.1 .0 <= pairs.0 .0 && pairs.1 .1 >= pairs.0 .1)
            })
            .count();

        println!("{}", result);
    }

    fn part_two(&self, input: &str) {
        let result = input
            .lines()
            .filter(|line| {
                let collected: Vec<_> = line
                    .split(",")
                    .map(|part| {
                        let collected: Vec<_> = part
                            .split("-")
                            .map(|seg| seg.parse::<usize>().unwrap())
                            .collect();
                        (collected[0], collected[1])
                    })
                    .collect();
                let pairs = (collected[0], collected[1]);

                pairs.0 .1 >= pairs.1 .0 && pairs.1 .1 >= pairs.0 .0
            })
            .count();

        println!("{}", result);
    }
}
