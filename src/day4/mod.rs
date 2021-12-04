use std::collections::{HashMap, HashSet};
use anyhow::{Result};
use itertools::{Itertools, izip};
use regex::{Regex, RegexBuilder};
use crate::util::{parse_lines, parse_lines_regex};

struct Bingo {
    nums: Vec<Vec<i32>>,
    drawn: HashSet<i32>,
}

impl Bingo {
    pub fn new(nums: Vec<Vec<i32>>) -> Self {
        Bingo { nums, drawn: HashSet::new() }
    }

    pub fn complete(&self) -> bool {
        (0..5).any(|y| (0..5).all(|x| self.drawn.contains(&self.nums[x][y])))
            || (0..5).any(|y| (0..5).all(|x| self.drawn.contains(&self.nums[y][x])))
    }

    pub fn add(&mut self, n: i32) -> Option<i32> {
        self.drawn.insert(n);
        if self.complete() {
            let sum: i32 = self.nums.iter().flat_map(|xs| xs.iter().filter(|x| !self.drawn.contains(*x))).sum();
            Some(sum * n)
        } else {
            None
        }
    }
}

fn solution1(input: &str) -> Result<String> {
    let split: Vec<&str> = input.lines().nth(0).unwrap().split(",").collect();
    let numbers: Vec<i32> = split.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let bingos_str: Vec<&str> = input.lines().skip(1).collect();
    let re = RegexBuilder::new(r"([\d ]+)\n([\d ]+)\n([\d ]+)\n([\d ]+)\n([\d ]+)").build();
    let bingos_tmp: Vec<&str> = bingos_str.into_iter().flat_map(|x| x.trim().split(" ").filter(|x| x.len() > 0).collect::<Vec<&str>>()).collect();
    let mut bingos: Vec<Bingo> = bingos_tmp.chunks_exact(25).map(|xs| {
        let xx: Vec<i32> = xs.iter().map(|y| y.parse::<i32>().unwrap()).collect();
        Bingo::new(xx.chunks(5).map(|x| x.iter().cloned().collect()).collect())
    }).collect();

    loop {
        for x in &numbers {
            for mut b in bingos.iter_mut() {
                match &b.add(*x) {
                    None => {}
                    Some(score) => return Ok(format!("{}", *score))
                }
            }
        }
    }
}

fn solution_numbers(input: &str) -> Result<String> {
    let xs = parse_lines::<i32>(input)?;

    let ys = xs.iter()
        .map(|x| {
            dbg!(*x)
        })
        .collect::<Vec<i32>>();

    Ok(format!("{}", "??"))
}

fn solution2(input: &str) -> Result<String> {
    let split: Vec<&str> = input.lines().nth(0).unwrap().split(",").collect();
    let numbers: Vec<i32> = split.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let bingos_str: Vec<&str> = input.lines().skip(1).collect();
    let re = RegexBuilder::new(r"([\d ]+)\n([\d ]+)\n([\d ]+)\n([\d ]+)\n([\d ]+)").build();
    let bingos_tmp: Vec<&str> = bingos_str.into_iter().flat_map(|x| x.trim().split(" ").filter(|x| x.len() > 0).collect::<Vec<&str>>()).collect();
    let mut bingos: Vec<Bingo> = bingos_tmp.chunks_exact(25).map(|xs| {
        let xx: Vec<i32> = xs.iter().map(|y| y.parse::<i32>().unwrap()).collect();
        Bingo::new(xx.chunks(5).map(|x| x.iter().cloned().collect()).collect())
    }).collect();

    let mut winners: HashSet<usize> = HashSet::new();
    let lenn = bingos.len();

    loop {
        for x in &numbers {
            for (i, mut b) in bingos.iter_mut().enumerate() {
                match &b.add(*x) {
                    None => {}
                    Some(score) => {
                        winners.insert(i);
                        if winners.len() == lenn {
                            return Ok(format!("{}", *score));
                        }
                    }
                }
            }
        }
    }
}


mod tests {
    use indoc::indoc;
    use crate::run_solution;

    use crate::day4::{solution1, solution2};

    const INPUT: &'static str = "day4.txt";

    // PART 1

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2
    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}

