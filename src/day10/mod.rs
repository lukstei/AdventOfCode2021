use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use std::collections::{HashMap, HashSet, VecDeque};

fn opening_char(c: char) -> Option<char> {
    match c {
        '}' => Some('{'),
        ')' => Some('('),
        ']' => Some('['),
        '>' => Some('<'),
        _ => None,
    }
}

fn closing_char(c: char) -> Option<char> {
    match c {
        '{' => Some('}'),
        '(' => Some(')'),
        '[' => Some(']'),
        '<' => Some('>'),
        _ => None,
    }
}

fn solution1(input: &str) -> Result<String> {
    let mut stack: VecDeque<char> = Default::default();

    let result: i32 = input
        .lines()
        .map(|line| {
            let mut error = None;

            for c in line.chars() {
                match opening_char(c) {
                    Some(open_char) => match stack.pop_back() {
                        None => panic!("unexpected end"),
                        Some(actual) => {
                            if actual != open_char {
                                println!("Expected {} got {} instead", open_char, actual);
                                error = Some(match c {
                                    ')' => 3,
                                    ']' => 57,
                                    '}' => 1197,
                                    '>' => 25137,
                                    _ => panic!("unexpected error char"),
                                });
                                return error;
                            }
                        }
                    },
                    _ => stack.push_back(c),
                };
            }

            error
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .sum();

    Ok(format!("{}", result))
}

fn solution2(input: &str) -> Result<String> {
    let mut result = input
        .lines()
        .map(|line| {
            let mut stack: VecDeque<char> = Default::default();

            for c in line.trim().chars() {
                match opening_char(c) {
                    Some(open_char) => match stack.pop_back() {
                        None => panic!("unexpected end"),
                        Some(actual) => {
                            if actual != open_char {
                                println!("Expected {} got {} instead", open_char, actual);
                                return None;
                            }
                        }
                    },
                    _ => stack.push_back(c),
                };
            }

            println!("left is: {}", stack.iter().join(""));

            let mut score: u64 = 0;
            loop {
                match stack.pop_back() {
                    None => break,
                    Some(c) => {
                        let closing_char = closing_char(c).unwrap();

                        score *= 5;
                        score += match closing_char {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => panic!("unexpected error char"),
                        }
                    }
                }
            }

            Some(score)
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect_vec();

    result.sort();
    Ok(format!("{}", result[result.len() / 2]))
}

mod tests {
    use crate::day10::{solution1, solution2};
    use crate::run_solution;
    use indoc::indoc;

    const INPUT: &'static str = "day10.txt";

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!(
            "1197",
            solution1(indoc!("{([(<{}[<>[]}>{[]{[(<()>")).unwrap()
        );
    }

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2

    #[test]
    fn test_part2() {
        assert_eq!(
            "288957",
            solution2(indoc!(
                "
    [({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    (((({<>}<{<{<>}{[]{[]{}
    {<[[]]>}<{[{[{[]{()[[[]
    <{([{{}}[<[[[<>{}]]]>[]]"
            ))
            .unwrap()
        );
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}
