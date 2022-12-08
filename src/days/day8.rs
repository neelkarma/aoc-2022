use super::Day;

pub struct DayEight;

impl DayEight {
    fn parse_grid(input: &str) -> Vec<Vec<u32>> {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect()
            })
            .collect()
    }
}

impl Day for DayEight {
    fn part_one(&self, input: &str) {
        let grid = Self::parse_grid(input);

        let mut visible = 0;

        for (y, line) in grid.iter().enumerate() {
            for (x, &tree) in line.iter().enumerate() {
                if x == 0 || x == line.len() || y == 0 || y == grid.len() {
                    visible += 1;
                    continue;
                };

                if [
                    (x + 1..line.len()).map(|x| (x, y)).collect::<Vec<_>>(),
                    (0..x).map(|x| (x, y)).collect(),
                    (y + 1..grid.len()).map(|y| (x, y)).collect(),
                    (0..y).map(|y| (x, y)).collect(),
                ]
                .iter()
                .any(|path| path.iter().all(|&(x, y)| grid[y][x] < tree))
                {
                    visible += 1;
                }
            }
        }

        println!("{}", visible);
    }

    fn part_two(&self, input: &str) {
        let grid = Self::parse_grid(input);

        let result = grid
            // Transform Vec<Vec<height>> into Vec<(x, y, height)>
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, &tree)| (x, y, tree)))
            // Calculate scenic score for each tree
            .map(|(x, y, tree)| {
                [
                    (x + 1..grid[0].len()).map(|x| (x, y)).collect::<Vec<_>>(),
                    (0..x).map(|x| (x, y)).rev().collect(),
                    (y + 1..grid.len()).map(|y| (x, y)).collect(),
                    (0..y).map(|y| (x, y)).rev().collect(),
                ]
                .iter()
                // Measure viewing distance for each direction
                .map(|iter| {
                    let mut viewing_distance = 0;
                    for &(x, y) in iter {
                        viewing_distance += 1;
                        if grid[y][x] >= tree {
                            break;
                        }
                    }
                    viewing_distance
                })
                // Multiply the viewing distances together
                .reduce(|acc, dist| acc * dist)
                .unwrap()
            })
            // Find max
            .max()
            .unwrap();

        println!("{}", result);
    }
}
