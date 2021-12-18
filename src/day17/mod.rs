use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use std::collections::{HashMap, HashSet};

struct State {
    pos: (i32, i32),
    vel: (i32, i32),
    target_area: ((i32, i32), (i32, i32)),
}

impl State {
    pub fn step(&mut self) {
        self.pos = (self.pos.0 + self.vel.0, self.pos.1 + self.vel.1);

        if self.vel.0.abs() <= 1 {
            self.vel.0 = 0;
        } else {
            self.vel.0 = self.vel.0.signum() * (self.vel.0.abs() - 1)
        }
        self.vel.1 = self.vel.1 - 1;
    }

    pub fn outside_of_target_zone(&self) -> bool {
        self.pos.0 > self.target_area.0 .0.max(self.target_area.0 .1)
            || self.pos.1 < self.target_area.1 .0.min(self.target_area.1 .1)
    }

    pub fn finished(&self) -> bool {
        self.pos.0 >= self.target_area.0 .0.min(self.target_area.0 .1)
            && self.pos.0 <= self.target_area.0 .0.max(self.target_area.0 .1)
            && self.pos.1 <= self.target_area.1 .0.max(self.target_area.1 .1)
            && self.pos.1 >= self.target_area.1 .0.min(self.target_area.1 .1)
    }
}

fn solution1(input: &str) -> Result<String> {
    let mut highest_y_glob = (i32::MIN);
    let mut successes: Vec<(i32, i32)> = Vec::new();

    for x in -1000..1000 {
        for y in -1000..1000 {
            let mut s = State {
                pos: (0, 0),
                vel: (x, y),
                target_area: ((32, 65), (-225, -177)),
            };

            let mut highest_y = (i32::MIN);

            loop {
                s.step();
                highest_y = highest_y.max(s.pos.1);

                if s.finished() {
                    highest_y_glob = highest_y.max(highest_y_glob);
                    successes.push((x, y));
                    println!("{:?}: highy = {}", (x, y), highest_y);
                    break;
                    //   return Ok(format!("{}", highest_y))
                }
                if s.outside_of_target_zone() {
                    break;
                }
            }
        }
    }

    Ok(format!("{}", successes.iter().count()))
}

fn solution2(input: &str) -> Result<String> {
    Ok(format!("{}", "?"))
}

mod tests {
    use crate::day17::{solution1, solution2};
    use crate::run_solution;
    use indoc::indoc;
    #[test]
    fn test_part1() {
        assert_eq!("45", solution1(indoc!("")).unwrap());
    }

    const INPUT: &'static str = "day17.txt";

    // PART 1

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2

    #[test]
    fn test_part2() {
        assert_eq!("45", solution1(indoc!("")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}
