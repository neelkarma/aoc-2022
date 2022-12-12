use std::collections::{HashMap, HashSet};

use super::Day;

pub struct DayTwelve;

impl DayTwelve {
    fn dijkstra(graph: HashMap<usize, HashSet<usize>>, start: usize) -> HashMap<usize, usize> {
        let mut distances: HashMap<usize, usize> =
            HashMap::from_iter(graph.keys().map(|&key| (key, usize::MAX - 1)));
        distances.insert(start, 0);

        let mut unvisited: HashSet<usize> = HashSet::from_iter(graph.keys().map(|&key| key));

        while !unvisited.is_empty() {
            let current = *unvisited
                .iter()
                .min_by_key(|&node| distances[node])
                .unwrap();

            unvisited.remove(&current);

            for &other in graph[&current]
                .iter()
                .filter(|node| unvisited.contains(node))
            {
                let potential_dist = distances[&current] + 1;
                if potential_dist < distances[&other] {
                    distances.insert(other, potential_dist);
                };
            }
        }

        distances
    }
}

impl Day for DayTwelve {
    fn part_one(&self, input: &str) {
        let mut start = None;
        let mut end = None;

        let grid: Vec<_> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, chr)| match chr {
                        'S' => {
                            start = Some(y * line.len() + x);
                            0
                        }
                        'E' => {
                            end = Some(y * line.len() + x);
                            25
                        }
                        _ => chr as usize - 'a' as usize,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let start = start.unwrap();
        let end = end.unwrap();

        let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
        let (width, height) = (grid[0].len(), grid.len());

        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let mut connected: HashSet<usize> = HashSet::new();
                if x > 0 {
                    if grid[y][x - 1] as isize - cell as isize <= 1 {
                        connected.insert(y * width + (x - 1));
                    }
                };
                if x < width - 1 {
                    if grid[y][x + 1] as isize - cell as isize <= 1 {
                        connected.insert(y * width + (x + 1));
                    }
                };
                if y > 0 {
                    if grid[y - 1][x] as isize - cell as isize <= 1 {
                        connected.insert((y - 1) * width + x);
                    }
                }
                if y < height - 1 {
                    if grid[y + 1][x] as isize - cell as isize <= 1 {
                        connected.insert((y + 1) * width + x);
                    }
                }
                graph.insert(y * width + x, connected);
            }
        }

        let distances = Self::dijkstra(graph, start);
        let result = distances[&end];

        println!("{}", result);
    }

    fn part_two(&self, input: &str) {
        let mut start = None;
        let mut ends = Vec::new();

        let grid: Vec<_> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, chr)| match chr {
                        'S' | 'a' => {
                            ends.push(y * line.len() + x);
                            0
                        }
                        'E' => {
                            start = Some(y * line.len() + x);
                            25
                        }
                        _ => chr as usize - 'a' as usize,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let start = start.unwrap();

        let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
        let (width, height) = (grid[0].len(), grid.len());

        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let mut connected: HashSet<usize> = HashSet::new();
                if x > 0 {
                    if cell as isize - grid[y][x - 1] as isize <= 1 {
                        connected.insert(y * width + (x - 1));
                    }
                };
                if x < width - 1 {
                    if cell as isize - grid[y][x + 1] as isize <= 1 {
                        connected.insert(y * width + (x + 1));
                    }
                };
                if y > 0 {
                    if cell as isize - grid[y - 1][x] as isize <= 1 {
                        connected.insert((y - 1) * width + x);
                    }
                }
                if y < height - 1 {
                    if cell as isize - grid[y + 1][x] as isize <= 1 {
                        connected.insert((y + 1) * width + x);
                    }
                }
                graph.insert(y * width + x, connected);
            }
        }

        let distances = Self::dijkstra(graph, start);
        let result = ends.iter().map(|end| distances[end]).min().unwrap();

        println!("{}", result);
    }
}
