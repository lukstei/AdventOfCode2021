use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use std::collections::{HashMap, HashSet};

pub fn solution1(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?) ([0-9]+)$")?;
    let mut hor = 0;
    let mut vert = 0;

    xs.iter().for_each(|l| {
        let dir = &l[1];
        let am = l[2].parse::<i32>().unwrap();

        match dir.as_str() {
            "forward" => hor += am,
            "down" => vert += am,
            "up" => vert -= am,
            _ => panic!(),
        }
    });

    Ok(format!("{}", hor * vert))
}

pub fn solution2(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?) ([0-9]+)$")?;
    let mut hor = 0;
    let mut vert = 0;
    let mut aim = 0;

    xs.iter().for_each(|l| {
        let dir = &l[1];
        let am = l[2].parse::<i32>().unwrap();

        match dir.as_str() {
            "forward" => {
                hor += am;
                vert += am * aim;
            }
            "down" => aim += am,
            "up" => aim -= am,
            _ => panic!(),
        }
    });

    Ok(format!("{}", hor * vert))
}

mod tests {
    use crate::day2::{solution1, solution2};
    use indoc::indoc;

    #[test]
    fn test_part1() {
        assert_eq!(
            solution1(indoc!(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            ))
            .unwrap(),
            "150"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solution2(indoc!(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            ))
            .unwrap(),
            "900"
        );
    }
}
