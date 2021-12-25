use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use regex::internal::Char;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

struct Grid {
    xs: Vec<Vec<char>>,
    width: usize,
    height: usize,
    step: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            write!(f, "{}\n", self.xs[y].iter().join(""))?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(xs: Vec<Vec<char>>) -> Self {
        let width = xs[0].len();
        let height = xs.len();
        Grid {
            xs,
            width,
            height,
            step: 0,
        }
    }
    fn x(&self, x: usize) -> usize {
        x % self.width
    }
    fn y(&self, y: usize) -> usize {
        y % self.height
    }
    fn step(&mut self) -> bool {
        let mut was_moved = false;
        self.step += 1;

        let mut new = self.xs.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                // move east
                let target = self.x(x + 1);
                if is_east(self.xs[y][x]) && is_empty(self.xs[y][target]) {
                    new[y][x] = '.';
                    new[y][target] = '>';
                    was_moved = true;
                }
            }
        }

        self.xs = new;
        let mut new = self.xs.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                // move east
                let target = self.y(y + 1);
                if is_south(self.xs[y][x]) && is_empty(self.xs[target][x]) {
                    new[y][x] = '.';
                    new[target][x] = 'v';
                    was_moved = true;
                }
            }
        }
        self.xs = new;
        was_moved
    }
}

fn is_east(c: char) -> bool {
    c == '>'
}

fn is_south(c: char) -> bool {
    c == 'v'
}

fn is_empty(c: char) -> bool {
    c == '.'
}

fn solution1(input: &str) -> Result<String> {
    let xs = input.lines().map(|c| c.chars().collect_vec()).collect_vec();

    let mut grid = Grid::new(xs);

    loop {
        let was_moved = grid.step();
        //println!("{}", grid);
        if !was_moved {
            break;
        }
    }

    Ok(format!("{}", grid.step))
}

fn solution2(input: &str) -> Result<String> {
    Ok(format!("{}", "?"))
}

mod tests {
    use crate::day25::{solution1, solution2, Grid};
    use crate::run_solution;
    use indoc::indoc;
    use itertools::Itertools;

    const INPUT: &'static str = "day25.txt";

    // PART 1

    #[test]
    fn test_1() {
        let mut grid = Grid::new(
            "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
                .lines()
                .map(|c| c.chars().collect_vec())
                .collect_vec(),
        );

        grid.step();
        assert_eq!(
            format!("{}", grid).trim(),
            "....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v"
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            "58",
            solution1(indoc!(
                "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
            ))
            .unwrap()
        );
    }

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2

    #[test]
    fn test_part2() {
        assert_eq!("??", solution2(indoc!("")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}
