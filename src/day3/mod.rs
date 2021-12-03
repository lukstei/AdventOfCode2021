use std::collections::{HashMap, HashSet};
use anyhow::{Result};
use itertools::{Itertools, izip};
use crate::util::{parse_lines, parse_lines_regex};


fn solution1(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?)$")?;

    let ys = xs.iter()
        .map(|l| {
            let x = l[1].clone();

            x
        }).collect::<Vec<String>>();

    let len = ys[0].len();

    let n1_str = (0..len).map(|i| ys.iter().fold((0, 0), |mut b, x1| {
        let x3 = x1.as_str();
        match x3.chars().nth(i).unwrap() {
            '1' => b.1 += 1,
            '0' => b.0 += 1,
            _ => panic!()
        }
        b
    })).map(|x2| if x2.0 > x2.1 { "0" } else { "1" })
        .collect::<Vec<&str>>().join("");


    let n2_str = (0..len).map(|i| ys.iter().fold((0, 0), |mut b, x1| {
        let x3 = x1.as_str();
        match x3.chars().nth(i).unwrap() {
            '1' => b.1 += 1,
            '0' => b.0 += 1,
            _ => panic!()
        }
        b
    })).map(|x2| if x2.1 > x2.0 { "0" } else { "1" })
        .collect::<Vec<&str>>().join("");

    let n1 = isize::from_str_radix(&n1_str, 2).unwrap();

    let n2 = isize::from_str_radix(&n2_str, 2).unwrap();

    Ok(format!("{}", n1 * n2))
}

fn solution2(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?)$")?;

    let ys = xs.iter()
        .map(|l| {
            let x = l[1].clone();

            x
        }).collect::<Vec<String>>();

    let len = ys[0].len();

    let mut ys1 = ys.clone();
    let mut winner1: Option<String> = None; // 23
    (0..len).for_each(|i| {
        let xx = ys1.iter().fold((0, 0), |mut b, x1| {
            let x3 = x1.as_str();
            match x3.chars().nth(i).unwrap() {
                '1' => b.1 += 1,
                '0' => b.0 += 1,
                _ => panic!()
            }
            b
        });
        let winner_char = if xx.1 >= xx.0 { '1' } else { '0' };

        ys1 = ys1.iter().filter(|s| s.chars().nth(i).unwrap() == winner_char).cloned().collect();
        if ys1.len() == 1 {
            winner1 = Some(ys1[0].clone())
        }
    });

    let mut ys2 = ys.clone();
    let mut winner2: Option<String> = None; // 23
    (0..len).for_each(|i| {
        let xx = ys2.iter().fold((0, 0), |mut b, x1| {
            let x3 = x1.as_str();
            match x3.chars().nth(i).unwrap() {
                '1' => b.1 += 1,
                '0' => b.0 += 1,
                _ => panic!()
            }
            b
        });
        let winner_char = if xx.0 <= xx.1 { '0' } else { '1' };

        ys2 = ys2.iter().filter(|s| s.chars().nth(i).unwrap() == winner_char).cloned().collect();
        if ys2.len() == 1 {
            winner2 = Some(ys2[0].clone())
        }
    });

    let n1 = isize::from_str_radix(&winner1.unwrap(), 2).unwrap();
    let n2 = isize::from_str_radix(&winner2.unwrap(), 2).unwrap();

    Ok(format!("{}", n1 * n2))
}

mod tests {
    use indoc::indoc;
    use crate::run_solution;
    use crate::day3::{solution1, solution2};

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!("198",
                   solution1(indoc!("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010")).unwrap());
    }

    const INPUT: &'static str = "day3.txt";

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2


    #[test]
    fn test_part2() {
        assert_eq!("230", solution2(indoc!("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}