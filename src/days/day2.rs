use super::Day;

enum Outcome {
    Win,
    Tie,
    Loss,
}

impl Outcome {
    fn from_str(c: &str) -> Self {
        match c {
            "X" => Self::Loss,
            "Y" => Self::Tie,
            "Z" => Self::Win,
            _ => panic!("Invalid outcome {}", c),
        }
    }
    fn to_points(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Tie => 3,
            Self::Loss => 0,
        }
    }

    fn move_needed(&self, other: &Move) -> Move {
        match (other, self) {
            (Move::Rock, Self::Win) => Move::Paper,
            (Move::Rock, Self::Tie) => Move::Rock,
            (Move::Rock, Self::Loss) => Move::Scissors,
            (Move::Paper, Self::Win) => Move::Scissors,
            (Move::Paper, Self::Tie) => Move::Paper,
            (Move::Paper, Self::Loss) => Move::Rock,
            (Move::Scissors, Self::Win) => Move::Rock,
            (Move::Scissors, Self::Tie) => Move::Scissors,
            (Move::Scissors, Self::Loss) => Move::Paper,
        }
    }
}

enum Move {
    Paper,
    Scissors,
    Rock,
}

impl Move {
    fn from_str(c: &str) -> Self {
        match c {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => panic!("Invalid move {}", c),
        }
    }

    fn vs(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Rock) => Outcome::Tie,
            (Self::Rock, Self::Paper) => Outcome::Loss,
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Paper) => Outcome::Tie,
            (Self::Paper, Self::Scissors) => Outcome::Loss,
            (Self::Scissors, Self::Rock) => Outcome::Loss,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            (Self::Scissors, Self::Scissors) => Outcome::Tie,
        }
    }

    fn to_points(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

pub struct DayTwo;

impl Day for DayTwo {
    fn part_one(&self, input: &str) {
        let result: usize = input
            .lines()
            .map(|line| {
                let collected: Vec<_> = line.split_whitespace().collect();

                let them = Move::from_str(collected[0]);
                let us = Move::from_str(collected[1]);

                let outcome = us.vs(&them);

                us.to_points() + outcome.to_points()
            })
            .sum();

        println!("{}", result);
    }

    fn part_two(&self, input: &str) {
        let result: usize = input
            .lines()
            .map(|line| {
                let collected: Vec<_> = line.split_whitespace().collect();

                let them = Move::from_str(collected[0]);
                let outcome = Outcome::from_str(collected[1]);

                let us = outcome.move_needed(&them);

                us.to_points() + outcome.to_points()
            })
            .sum();

        println!("{}", result);
    }
}
