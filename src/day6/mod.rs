use std::collections::{HashMap, HashSet};
use anyhow::{Result};
use itertools::{Itertools, izip};
use crate::util::{parse_lines, parse_lines_regex};

#[derive(Copy, Clone)]
struct Lanternfish(u32);

impl Lanternfish {
    pub fn step(&mut self) -> Option<Lanternfish> {
        if self.0 == 0 {
            self.0 = 6;
            Some(Lanternfish(8))
        } else {
            self.0 -= 1;
            None
        }
    }
}

struct LaternfishPopulation {
    by_age: [u64; 9],
}

impl LaternfishPopulation {
    pub fn from_list(xs: Vec<usize>) -> LaternfishPopulation {
        let mut n: [u64; 9] = [0; 9];
        for x in xs {
            n[x] += 1;
        }
        LaternfishPopulation { by_age: n }
    }

    pub fn step(&mut self) {
        let mut n: [u64; 9] = [0; 9];

        (1..=8).for_each(|age| {
            n[age - 1] = self.by_age[age];
        });

        n[6] += self.by_age[0];
        n[8] += self.by_age[0];
        self.by_age = n;
    }

    pub fn len(&self) -> u64 {
        self.by_age.iter().sum()
    }
}


fn solution1(input: &str) -> Result<String> {
    let mut xs = input.split(",").map(|x| x.parse::<u32>().unwrap()).map(|x| Lanternfish(x)).collect_vec();

    for x in 0..80 {
        xs = xs.into_iter().flat_map(|mut x| x.step().map_or_else(|| vec![x], |n| vec![x, n])).collect_vec();
    }

    Ok(format!("{}", xs.len()))
}

fn solution2(input: &str) -> Result<String> {
    let xs = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect_vec();

    let mut pop = LaternfishPopulation::from_list(xs);

    for x in 0..256 {
        pop.step();
    }

    Ok(format!("{}", pop.len()))
}


mod tests {
    use indoc::indoc;
    use crate::run_solution;
    use crate::day6::{solution1, solution2};

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!("5934",
                   solution1(indoc!("3,4,3,1,2")).unwrap());
    }

    const INPUT: &'static str = "day6.txt";

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2


    #[test]
    fn test_part2() {
        assert_eq!("26984457539", solution2(indoc!("3,4,3,1,2")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}

