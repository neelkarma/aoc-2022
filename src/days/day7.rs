use super::Day;

#[derive(Debug)]

pub struct DaySeven;

impl DaySeven {
    fn sizes(input: &str) -> Vec<usize> {
        let mut stack: Vec<usize> = Vec::new();
        let mut sizes: Vec<usize> = Vec::new();

        input.lines().for_each(|line| {
            match line.split_ascii_whitespace().collect::<Vec<_>>()[..] {
                ["$", "cd", ".."] => {
                    // Add dir size to sizes
                    sizes.push(stack.pop().unwrap());

                    // add child dir size to parent dir size if present
                    if !stack.is_empty() {
                        *stack.last_mut().unwrap() += sizes.last().unwrap();
                    }
                }
                ["$", "cd", _] => {
                    // create new dir in stack
                    stack.push(0);
                }
                ["$", _] => {}
                ["dir", _] => {}
                [size, _] => {
                    // add file size to current dir in stack
                    *stack.last_mut().unwrap() += size.parse::<usize>().unwrap();
                }
                _ => unreachable!(),
            };
        });

        // Travel back up the stack, computing missing sizes as we go
        while !stack.is_empty() {
            sizes.push(stack.pop().unwrap());
            if !stack.is_empty() {
                *stack.last_mut().unwrap() += sizes.last().unwrap();
            }
        }

        sizes
    }
}

impl Day for DaySeven {
    fn part_one(&self, input: &str) {
        let result: usize = Self::sizes(input)
            .iter()
            .filter(|size| size <= &&100000)
            .sum();

        println!("{}", result);
    }

    fn part_two(&self, input: &str) {
        let sizes = Self::sizes(input);
        let max = sizes.iter().max().unwrap();
        let unused = 70000000 - max;
        let required = 30000000;
        let result = sizes
            .iter()
            .filter(|size| unused + **size >= required)
            .min()
            .unwrap();

        println!("{}", result)
    }
}
