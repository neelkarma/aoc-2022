use std::{collections::HashSet, iter::repeat};

use super::Day;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn touching(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn nudge_towards(&mut self, other: &Self) {
        if other.y > self.y {
            self.y += 1;
        };

        if other.y < self.y {
            self.y -= 1;
        };

        if other.x > self.x {
            self.x += 1;
        };

        if other.x < self.x {
            self.x -= 1;
        };
    }
}

impl Default for Coords {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply_to(&self, coords: &mut Coords) {
        match self {
            Self::Up => coords.y += 1,
            Self::Down => coords.y -= 1,
            Self::Left => coords.x -= 1,
            Self::Right => coords.x += 1,
        };
    }
}

impl From<&str> for Direction {
    fn from(chr: &str) -> Self {
        match chr {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }
}

pub struct DayNine;

impl DayNine {
    fn solve(input: &str, rope_length: usize) -> usize {
        let directions = input.lines().flat_map(|line| {
            let parts: Vec<_> = line.split_ascii_whitespace().collect();
            repeat(Direction::from(parts[0])).take(parts[1].parse().unwrap())
        });

        let mut segments: Vec<Coords> = vec![Coords::default(); rope_length];
        let mut visited: HashSet<Coords> = HashSet::new();
        visited.insert(segments.last().unwrap().clone());

        for dir in directions {
            // Apply direction to head
            dir.apply_to(segments.first_mut().unwrap());

            // Save previous segment for comparison
            let mut previous = segments.first().unwrap().clone();

            for segment in segments.iter_mut().skip(1) {
                if !segment.touching(&previous) {
                    segment.nudge_towards(&previous);
                };

                // Update previous segment
                previous = segment.clone();
            }

            // Insert new tail position into visited set
            visited.insert(segments.last().unwrap().clone());
        }

        visited.len()
    }
}

impl Day for DayNine {
    fn part_one(&self, input: &str) {
        println!("{}", Self::solve(input, 2));
    }

    fn part_two(&self, input: &str) {
        println!("{}", Self::solve(input, 10));
    }
}
