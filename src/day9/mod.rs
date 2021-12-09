use std::collections::{HashMap, HashSet};
use anyhow::{Result};
use itertools::{Itertools, izip};
use crate::util::{parse_lines, parse_lines_regex};

fn solution1(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?)$")?;

    let ys = xs.iter()
        .map(|l| {
            l[1].as_str().chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect_vec()
        })
        .collect::<Vec<Vec<usize>>>();

    let w = ys[0].len();
    let h = ys.len();

    let mut lowest_is: Vec<(usize, usize)> = Default::default();

    (0..w).for_each(|x| {
        (0..h).for_each(|y| {
            let el = ys[y][x];

            let mut lowest = true;
            if x >= 1 && (ys[y][x - 1] <= el) {
                lowest = false;
            }
            if x < (w - 1) && (ys[y][x + 1] <= el) {
                lowest = false;
            }
            if y >= 1 && (ys[y - 1][x] <= el) {
                lowest = false;
            }
            if y < (h - 1) && (ys[y + 1][x] <= el) {
                lowest = false;
            }

            if lowest {
                lowest_is.push((x, y))
            }
        })
    });

    let res: usize = lowest_is.iter().map(|(x, y)| {
        let xx = ys[*y][*x];
        println!("{}", xx);
        xx + 1
    }).sum();

    Ok(format!("{}", res))
}

fn basin(ys: &Vec<Vec<usize>>, counted: &mut HashSet<(usize, usize)>, x: usize, y: usize) -> usize {
    if x >= 0 && x < ys[0].len() && y >= 0 && y < ys.len() && ys[y][x] < 9 && !counted.contains(&(x, y)) {
        counted.insert((x, y));
        1 + basin(ys, counted, x + 1, y) + basin(ys, counted, (x as i32 - 1).max(0) as usize, y) + basin(ys, counted, x, y + 1) + basin(ys, counted, x, (y as i32 - 1).max(0) as usize)
    } else {
        0
    }
}

fn solution2(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?)$")?;

    let ys = xs.iter()
        .map(|l| {
            l[1].as_str().chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect_vec()
        })
        .collect::<Vec<Vec<usize>>>();

    let w = ys[0].len();
    let h = ys.len();

    let mut basins: Vec<usize> = Default::default();

    (0..w).for_each(|x| {
        (0..h).for_each(|y| {
            let mut set = HashSet::new();
            let basin = basin(&ys, &mut set, x, y);
            basins.push(basin);
        })
    });

    basins.sort();

    let res_maybe: usize = basins.iter().unique().rev().take(3).product();

    Ok(format!("{}", res_maybe))
}


mod tests {
    use indoc::indoc;
    use crate::run_solution;
    use crate::day9::{solution1, solution2};

    const INPUT: &'static str = "day9.txt";

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!("15",
                   solution1(indoc!("2199943210
3987894921
9856789892
8767896789
9899965678")).unwrap());
    }


    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2


    #[test]
    fn test_part2() {
        assert_eq!("??", solution2(indoc!("2199943210
3987894921
9856789892
8767896789
9899965678")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}

