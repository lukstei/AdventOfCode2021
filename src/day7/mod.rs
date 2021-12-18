use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use std::collections::{HashMap, HashSet};

fn solution1(input: &str) -> Result<String> {
    Ok(format!("{}", "??"))
}

fn solution2(input: &str) -> Result<String> {
    let xs = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .sorted()
        .collect_vec();

    let res: i32 = (*xs.first().unwrap()..=*xs.last().unwrap())
        .map(|res| {
            let ss: i32 = xs
                .iter()
                .map(|x| {
                    (0..(res - x).abs())
                        .enumerate()
                        .map(|(i, _x)| (i + 1) as i32)
                        .fold(0, |b, x| b + x)
                })
                .fold(0, |b, x| b + x);
            ss
        })
        .sorted()
        .nth(0)
        .unwrap();

    Ok(format!("{}", res))
}

mod tests {
    use crate::day7::{solution1, solution2};
    use crate::run_solution;
    use indoc::indoc;

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!("37", solution1(indoc!("16,1,2,0,4,2,7,1,2,14")).unwrap());
    }

    const INPUT: &'static str = "day7.txt";

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2

    #[test]
    fn test_part2() {
        assert_eq!("168", solution2(indoc!("16,1,2,0,4,2,7,1,2,14")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}
