use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use std::collections::{HashMap, HashSet};

fn fold_x(along_x: usize, marks: &mut HashSet<(usize, usize)>, max_x: usize, max_y: usize) {
    (0..=max_y).for_each(|y| {
        marks.remove(&(along_x, y));
    });

    (along_x + 1..=max_x).for_each(|x| {
        (0..=max_y).for_each(|y| {
            let fold_to = (along_x as i32 - (x as i32 - along_x as i32));
            //println!("Folding x={} to x={}", x, fold_to);

            if fold_to >= 0 {
                if marks.remove(&(x, y)) {
                    marks.insert((fold_to as usize, y));
                }
            }
        })
    })
}

fn fold_y(along_y: usize, marks: &mut HashSet<(usize, usize)>, max_x: usize, max_y: usize) {
    (0..=max_x).for_each(|x| {
        marks.remove(&(x, along_y));
    });

    (0..=max_x).for_each(|x| {
        (along_y + 1..=max_y).for_each(|y| {
            let fold_to = (along_y as i32 - (y as i32 - along_y as i32));
            //println!("Folding y={} to y={}", y, fold_to);

            if fold_to >= 0 {
                if marks.remove(&(x, y)) {
                    marks.insert((x, fold_to as usize));
                }
            }
        })
    })
}

fn solution1(input: &str, folds: Vec<(&str, usize)>) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?),(.+?)$")?;

    let mut marks: HashSet<(usize, usize)> = Default::default();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    xs.iter().for_each(|l| {
        let (v1, v2) = (
            l[1].as_str().parse::<usize>().unwrap(),
            l[2].as_str().parse::<usize>().unwrap(),
        );

        marks.insert((v1, v2));
        max_x = max_x.max(v1);
        max_y = max_y.max(v2);
    });

    for (dir, loc) in folds {
        //for y in 0..=max_y {
        //    for x in 0..=max_x {
        //        print!("{}", if marks.contains(&(x, y)) { "X" } else { "." });
        //    }
        //    println!();
        //}

        match dir {
            "x" => fold_x(loc, &mut marks, max_x, max_y),
            "y" => fold_y(loc, &mut marks, max_x, max_y),
            _ => {}
        }
    }

    Ok(format!("{}", marks.len()))
}

fn solution2(input: &str, folds: Vec<(&str, usize)>) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?),(.+?)$")?;

    let mut marks: HashSet<(usize, usize)> = Default::default();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    xs.iter().for_each(|l| {
        let (v1, v2) = (
            l[1].as_str().parse::<usize>().unwrap(),
            l[2].as_str().parse::<usize>().unwrap(),
        );

        marks.insert((v1, v2));
        max_x = max_x.max(v1);
        max_y = max_y.max(v2);
    });

    for (dir, loc) in folds {
        //for y in 0..=max_y {
        //    for x in 0..=max_x {
        //        print!("{}", if marks.contains(&(x, y)) { "X" } else { "." });
        //    }
        //    println!();
        //}

        match dir {
            "x" => {
                fold_x(loc, &mut marks, max_x, max_y);
                //max_x = loc;
            }
            "y" => {
                fold_y(loc, &mut marks, max_x, max_y);
                //max_y = loc;
            }
            _ => {}
        }
    }

    for y in 0..=20 {
        for x in 0..=100 {
            print!("{}", if marks.contains(&(x, y)) { "X" } else { "." });
        }
        println!();
    }

    Ok(format!("{}", marks.len()))
}

mod tests {
    use crate::day13::{solution1, solution2};
    use crate::run_solution;
    use indoc::indoc;
    use std::fs;
    use std::path::Path;

    const INPUT: &'static str = "day13.txt";

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!(
            "17",
            solution1(
                indoc!(
                    "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0"
                ),
                vec![("y", 7)]
            )
            .unwrap()
        );
    }

    #[test]
    fn run_solution1() {
        let input_path = Path::new("inputs").join("day13.txt");
        let input = fs::read_to_string(&input_path).unwrap();

        println!(
            "Solution with input: {}",
            solution1(input.trim(), vec![("x", 655)]).unwrap()
        );
    }

    // PART 2

    #[test]
    fn run_solution2() {
        let input_path = Path::new("inputs").join("day13.txt");
        let input = fs::read_to_string(&input_path).unwrap();

        println!(
            "Solution with input: {}",
            solution2(
                input.trim(),
                vec![
                    ("x", 655),
                    ("y", 447),
                    ("x", 327),
                    ("y", 223),
                    ("x", 163),
                    ("y", 111),
                    ("x", 81),
                    ("y", 55),
                    ("x", 40),
                    ("y", 27),
                    ("y", 13),
                    ("y", 6)
                ]
            )
            .unwrap()
        );
    }
}
