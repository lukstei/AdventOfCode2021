use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use std::collections::{HashMap, HashSet};

fn solution_regex(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?) (.+?) (.+?) (.+?)$")?;

    let ys = xs
        .iter()
        .map(|l| {
            let (v1, v2, v3, v4) = (
                l[1].as_str(),
                l[2].as_str(),
                l[3].as_str(),
                l[4].parse::<i32>().unwrap(),
            );

            dbg!(v1.as_ref())
        })
        .collect::<Vec<&str>>();

    Ok(format!("{}", "??"))
}

fn solution_numbers(input: &str) -> Result<String> {
    let xs = parse_lines::<i32>(input)?;

    let ys = xs.iter().map(|x| dbg!(*x)).collect::<Vec<i32>>();

    Ok(format!("{}", "??"))
}

fn solution1(input: &str) -> Result<String> {
    Ok(format!("{}", "?"))
}

fn solution2(input: &str) -> Result<String> {
    Ok(format!("{}", "?"))
}

mod tests {
    use crate::run_solution;
    use crate::template::{solution1, solution2};
    use indoc::indoc;
    const INPUT: &'static str = "dayXX.txt";

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!("??", solution1(indoc!("")).unwrap());
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
