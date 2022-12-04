use std::collections::HashSet;

use super::Day;

pub struct DayThree;

impl DayThree {
    fn type_to_priority(typ: char) -> usize {
        if typ.is_ascii_uppercase() {
            typ as usize - 'A' as usize + 27
        } else {
            typ as usize - 'a' as usize + 1
        }
    }
}

impl Day for DayThree {
    fn part_one(&self, input: &str) {
        let result: usize = input
            .lines()
            .map(|line| {
                let (comp_one, comp_two) = line.split_at(line.len() / 2);
                let comp_one: HashSet<_> = HashSet::from_iter(comp_one.chars());
                let comp_two: HashSet<_> = HashSet::from_iter(comp_two.chars());
                let common = HashSet::intersection(&comp_one, &comp_two).nth(0).unwrap();
                Self::type_to_priority(*common)
            })
            .sum();

        println!("{}", result)
    }

    fn part_two(&self, input: &str) {
        let result: usize = input
            .lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|lines| {
                let sets: Vec<_> = lines
                    .iter()
                    .map(|line| HashSet::from_iter(line.chars()))
                    .collect();

                let set = sets
                    .iter()
                    .skip(1)
                    .fold(sets[0].clone(), |acc: HashSet<_>, set| {
                        acc.intersection(set).cloned().collect()
                    });

                let common = set.iter().nth(0).unwrap();

                Self::type_to_priority(*common)
            })
            .sum();

        println!("{}", result)
    }
}
