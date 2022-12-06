use super::Day;

pub struct DaySix;

impl DaySix {
    fn is_unique(input: &str) -> bool {
        input
            .chars()
            .enumerate()
            .find_map(|(i, chr)| {
                input
                    .chars()
                    .enumerate()
                    .skip(i + 1)
                    .find(|(_, other)| chr == *other)
            })
            .is_none()
    }
}

impl Day for DaySix {
    fn part_one(&self, input: &str) {
        let chars = input.trim_end();
        let (i, _) = (0..chars.len() - 3)
            .map(|i| &chars[i..i + 4])
            .enumerate()
            .find(|(_, chunk)| Self::is_unique(chunk))
            .unwrap();
        println!("{}", i + 4);
    }

    fn part_two(&self, input: &str) {
        let chars = input.trim_end();
        let (i, _) = (0..chars.len() - 13)
            .map(|i| &chars[i..i + 14])
            .enumerate()
            .find(|(_, chunk)| Self::is_unique(chunk))
            .unwrap();
        println!("{}", i + 14);
    }
}
