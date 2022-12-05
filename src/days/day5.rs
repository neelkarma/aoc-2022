use std::collections::BTreeMap;

use super::Day;

pub struct DayFive;

impl DayFive {
    fn parse_ship(input: &str) -> BTreeMap<usize, Vec<char>> {
        let mut ship: BTreeMap<usize, Vec<char>> = BTreeMap::new();

        input
            .lines()
            .filter(|line| line.contains("["))
            .rev()
            .for_each(|line| {
                line.chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .for_each(|(stack, crt)| {
                        if crt.is_ascii_whitespace() {
                            return;
                        };

                        if ship.contains_key(&(stack + 1)) {
                            ship.get_mut(&(stack + 1)).unwrap().push(crt);
                        } else {
                            ship.insert(stack + 1, vec![crt]);
                        };
                    });
            });

        ship
    }
}

impl Day for DayFive {
    fn part_one(&self, input: &str) {
        let mut split = input.split("\n\n");
        let mut ship = Self::parse_ship(split.next().unwrap());

        split.next().unwrap().lines().for_each(|line| {
            let words: Vec<_> = line.split_ascii_whitespace().collect();

            let quantity: usize = words[1].parse().unwrap();
            let from: usize = words[3].parse().unwrap();
            let to: usize = words[5].parse().unwrap();

            for _ in 0..quantity {
                let popped = ship.get_mut(&from).unwrap().pop().unwrap();
                ship.get_mut(&to).unwrap().push(popped);
            }
        });

        let result: String = ship
            .iter()
            .map(|(_, crates)| crates.last().unwrap())
            .collect();

        println!("{}", result);
    }

    fn part_two(&self, input: &str) {
        let mut split = input.split("\n\n");
        let mut ship = Self::parse_ship(split.next().unwrap());

        split.next().unwrap().lines().for_each(|line| {
            let words: Vec<_> = line.split_ascii_whitespace().collect();

            let quantity: usize = words[1].parse().unwrap();
            let from: usize = words[3].parse().unwrap();
            let to: usize = words[5].parse().unwrap();

            let from_stack = ship.get_mut(&from).unwrap();
            let mut popped = from_stack.split_off(from_stack.len() - quantity);
            ship.get_mut(&to).unwrap().append(&mut popped);
        });

        let result: String = ship
            .iter()
            .map(|(_, crates)| crates.last().unwrap())
            .collect();

        println!("{}", result);
    }
}
