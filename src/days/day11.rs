use std::{
    cmp::{max, min},
    collections::VecDeque,
};

use super::Day;

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Operation {
    fn apply_to(&self, a: usize) -> usize {
        match self {
            Self::Add(b) => a + b,
            Self::Multiply(b) => a * b,
            Self::Square => a * a,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    inspected: usize,
    items: VecDeque<usize>,
    operation: Operation,
    divisor: usize,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn inspect(&mut self) -> usize {
        self.operation.apply_to(self.items.pop_front().unwrap())
    }
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let lines: Vec<_> = input
            .lines()
            .skip(1)
            .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
            .collect();

        let items: VecDeque<_> = lines[0][2..]
            .iter()
            .map(|num_str| num_str.trim_end_matches(",").parse::<usize>().unwrap())
            .collect();

        let operation = match lines[1][4..] {
            ["+", num] => Operation::Add(num.parse().unwrap()),
            ["*", "old"] => Operation::Square,
            ["*", num] => Operation::Multiply(num.parse().unwrap()),
            _ => panic!("Invalid operation string"),
        };

        let divisor: usize = lines[2][3].parse().unwrap();
        let true_target: usize = lines[3][5].parse().unwrap();
        let false_target: usize = lines[4][5].parse().unwrap();

        Self {
            inspected: 0,
            items,
            operation,
            divisor,
            true_target,
            false_target,
        }
    }
}

pub struct DayEleven;

impl DayEleven {
    // Courtesy of https://rosettacode.org/wiki/Least_common_multiple#Rust
    fn gcd(a: usize, b: usize) -> usize {
        match ((a, b), (a & 1, b & 1)) {
            ((x, y), _) if x == y => y,
            ((0, x), _) | ((x, 0), _) => x,
            ((x, y), (0, 1)) | ((y, x), (1, 0)) => Self::gcd(x >> 1, y),
            ((x, y), (0, 0)) => Self::gcd(x >> 1, y >> 1) << 1,
            ((x, y), (1, 1)) => {
                let (x, y) = (min(x, y), max(x, y));
                Self::gcd((y - x) >> 1, x)
            }
            _ => unreachable!(),
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        a * b / Self::gcd(a, b)
    }

    fn solve(input: &str, rounds: usize, worried: bool) -> usize {
        let mut monkeys: Vec<_> = input
            .split("\n\n")
            .map(|monkey_str| Monkey::from(monkey_str))
            .collect();

        let modulus = monkeys
            .iter()
            .map(|monkey| monkey.divisor)
            .reduce(|acc, cur| Self::lcm(acc, cur))
            .unwrap();

        for _ in 0..rounds {
            for m in 0..monkeys.len() {
                let monkey = &mut monkeys[m];
                monkey.inspected += monkey.items.len();

                let mut moves = Vec::new();
                while !monkey.items.is_empty() {
                    let item = if worried {
                        monkey.inspect()
                    } else {
                        monkey.inspect() / 3
                    } % modulus;

                    if item % monkey.divisor == 0 {
                        moves.push((item, monkey.true_target));
                    } else {
                        moves.push((item, monkey.false_target));
                    };
                }

                for (item, to) in moves {
                    monkeys[to].items.push_front(item);
                }
            }
        }

        let mut inspected_nums = monkeys
            .iter()
            .map(|monkey| monkey.inspected)
            .collect::<Vec<_>>();

        inspected_nums.sort();
        inspected_nums.reverse();

        inspected_nums[0] * inspected_nums[1]
    }
}

impl Day for DayEleven {
    fn part_one(&self, input: &str) {
        let result = Self::solve(input, 20, false);
        println!("{}", result);
    }

    fn part_two(&self, input: &str) {
        let result = Self::solve(input, 10000, true);
        println!("{}", result);
    }
}
