use super::Day;

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

pub struct DayTen;

impl DayTen {
    fn inc_cycle_p1(cycle: &mut usize, x: i32) -> i32 {
        *cycle += 1;

        if (*cycle as isize - 20) % 40 == 0 {
            (*cycle as i32) * x
        } else {
            0
        }
    }

    fn inc_cycle_p2(crt: &mut [bool], cycle: &mut usize, x: i32) {
        *cycle += 1;

        for px in [x - 1, x, x + 1] {
            if (*cycle - 1) % 40 == px as usize {
                crt[*cycle - 1] = true;
            }
        }
    }

    fn fmt_crt(crt: &[bool]) -> String {
        let mut buf = String::new();

        for y in 0..CRT_HEIGHT {
            for x in 0..CRT_WIDTH {
                buf.push(match crt[y * CRT_WIDTH + x] {
                    true => '#',
                    false => '.',
                });
            }
            buf.push('\n');
        }

        buf
    }
}

impl Day for DayTen {
    fn part_one(&self, input: &str) {
        let mut x = 1;
        let mut cycle = 1;
        let mut result = 0;

        for cmd in input
            .lines()
            .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        {
            match cmd[..] {
                ["noop"] => {
                    result += Self::inc_cycle_p1(&mut cycle, x);
                }
                ["addx", add] => {
                    result += Self::inc_cycle_p1(&mut cycle, x);
                    x += add.parse::<i32>().unwrap();
                    result += Self::inc_cycle_p1(&mut cycle, x);
                }
                _ => unreachable!(),
            }
        }

        println!("{}", result);
    }

    fn part_two(&self, input: &str) {
        let mut x = 1;
        let mut cycle = 1;
        let mut crt = [false; CRT_WIDTH * CRT_HEIGHT];

        for cmd in input
            .lines()
            .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        {
            match cmd[..] {
                ["noop"] => {
                    Self::inc_cycle_p2(&mut crt, &mut cycle, x);
                }
                ["addx", add] => {
                    Self::inc_cycle_p2(&mut crt, &mut cycle, x);
                    x += add.parse::<i32>().unwrap();
                    Self::inc_cycle_p2(&mut crt, &mut cycle, x);
                }
                _ => unreachable!(),
            }
        }

        println!("{}", Self::fmt_crt(&crt));
    }
}
