use std::collections::{HashMap, HashSet};
use anyhow::{Result};
use itertools::{Itertools, izip};
use crate::util::{parse_lines, parse_lines_regex};


fn visit1<'a>(visited: Vec<&'a str>, current: &'a str, all: &Vec<(&'a str, &'a str)>) -> Vec<Vec<&'a str>> {
    if current == "end" {
        vec!(visited)
    } else {
        all.iter().filter(|x| {
            x.0 == current && (x.1.chars().next().unwrap().is_uppercase() || !visited.contains(&x.1)) ||
                x.1 == current && (x.0.chars().next().unwrap().is_uppercase() || !visited.contains(&x.0))
        })
            .flat_map(|x| {
                let mut vec1 = visited.clone();
                let to = if x.0 == current { x.1 } else { x.0 };
                vec1.push(to);
                visit1(vec1, to, all)
            }).collect_vec()
    }
}

fn visit<'a>(visited: Vec<&'a str>, current: &'a str, all: &Vec<(&'a str, &'a str)>, small_cave_visited_twice: Option<&'a str>) -> Vec<Vec<&'a str>> {
    if current == "end" {
        vec!(visited)
    } else {
        all.iter().filter(|x| {
            let (from, to) = (if x.0 == current { x.0 } else { x.1 }, if x.0 == current { x.1 } else { x.0 });

            from == current && to != "start" && (to.chars().next().unwrap().is_uppercase() || !visited.contains(&to) || small_cave_visited_twice == None)
        })
            .flat_map(|x| {
                let mut vec1 = visited.clone();
                let to = if x.0 == current { x.1 } else { x.0 };
                vec1.push(to);
                let small_cave_visited_twice = match small_cave_visited_twice {
                    None => {
                        if !to.chars().next().unwrap().is_uppercase() && visited.contains(&to) {
                            Some(to)
                        } else {
                            None
                        }
                    }
                    Some(x) => Some(x)
                };

                visit(vec1, to, all, small_cave_visited_twice)
            }).collect_vec()
    }
}

fn solution1(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?)-(.+?)$")?;

    let ys = xs.iter()
        .map(|l| {
            let (v1, v2) = (
                l[1].as_str(),
                l[2].as_str()
            );

            (v1, v2)
        })
        .collect::<Vec<(&str, &str)>>();

    let visited = visit1(vec!["start"], "start", &ys);

    println!("{:?}", visited);

    Ok(format!("{}", visited.len()))
}


fn solution2(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?)-(.+?)$")?;

    let ys = xs.iter()
        .map(|l| {
            let (v1, v2) = (
                l[1].as_str(),
                l[2].as_str()
            );

            (v1, v2)
        })
        .collect::<Vec<(&str, &str)>>();

    let visited = visit(vec!["start"], "start", &ys, None);
    let visited = visited.iter().filter(|x| *x.last().unwrap() == "end").collect_vec();

    //   println!("{:?}", visited);

    Ok(format!("{}", visited.len()))
}


mod tests {
    use indoc::indoc;
    use crate::run_solution;
    use crate::day12::{solution1, solution2};

    const INPUT: &'static str = "day12.txt";

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!("10",
                   solution1(indoc!("start-A
start-b
A-c
A-b
b-d
A-end
b-end")).unwrap());
    }


    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2


    #[test]
    fn test_part2() {
        assert_eq!("36", solution2(indoc!("start-A
start-b
A-c
A-b
b-d
A-end
b-end")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}

