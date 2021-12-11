use std::collections::{HashMap, HashSet};
use anyhow::{Result};
use itertools::{Itertools, izip};
use crate::util::{parse_lines, parse_lines_regex};

fn solution2(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?)$")?;

    let mut ys = xs.iter()
        .map(|l| {
            l[1].as_str().chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect_vec()
        })
        .collect::<Vec<Vec<i32>>>();

    let w = ys[0].len();
    let h = ys.len();

    let mut flashes = 0;
    let mut step = 1;

    for x in 0..100000 {
        (0..w).for_each(|x| {
            (0..h).for_each(|y| {
                ys[y][x] += 1;
            })
        });

        let mut was_flash = true;

        while was_flash {
            was_flash = false;
            (0..w).for_each(|x| {
                (0..h).for_each(|y| {
                    if ys[y][x] > 9 {
                        ys[y][x] = -1;
                        was_flash = true;
                        flashes += 1;


                        if x >= 1 && ys[y][x - 1] >= 0 {
                            ys[y][x - 1] += 1;
                        }
                        if x < (w - 1) && ys[y][x + 1] >= 0 {
                            ys[y][x + 1] += 1;
                        }
                        if y >= 1 && ys[y - 1][x] >= 0 {
                            ys[y - 1][x] += 1;
                        }
                        if y < (h - 1) && ys[y + 1][x] >= 0 {
                            ys[y + 1][x] += 1;
                        }

                        if x >= 1 && y >= 1 && ys[y - 1][x - 1] >= 0 {
                            ys[y - 1][x - 1] += 1;
                        }
                        if x < (w - 1) && y >= 1 && ys[y - 1][x + 1] >= 0 {
                            ys[y - 1][x + 1] += 1;
                        }
                        if y < (h - 1) && x < (w - 1) && ys[y + 1][x + 1] >= 0 {
                            ys[y + 1][x + 1] += 1;
                        }
                        if y < (h - 1) && x >= 1 && ys[y + 1][x - 1] >= 0 {
                            ys[y + 1][x - 1] += 1;
                        }
                    }
                })
            });
        }

        let mut all_flashed = true;
        (0..w).for_each(|x| {
            (0..h).for_each(|y| {
                if ys[y][x] == -1 {
                    ys[y][x] = 0;
                } else {
                    all_flashed = false;
                }
            })
        });
        if all_flashed {
            break;
        }

        step += 1;
    }

    Ok(format!("{}", step))
}


fn solution1(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?)$")?;

    let mut ys = xs.iter()
        .map(|l| {
            l[1].as_str().chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect_vec()
        })
        .collect::<Vec<Vec<i32>>>();

    let w = ys[0].len();
    let h = ys.len();

    let mut flashes = 0;

    (0..100).for_each(|step| {
        (0..w).for_each(|x| {
            (0..h).for_each(|y| {
                ys[y][x] += 1;
            })
        });

        let mut was_flash = true;

        while was_flash {
            was_flash = false;
            (0..w).for_each(|x| {
                (0..h).for_each(|y| {
                    if ys[y][x] > 9 {
                        ys[y][x] = -1;
                        was_flash = true;
                        flashes += 1;


                        if x >= 1 && ys[y][x - 1] >= 0 {
                            ys[y][x - 1] += 1;
                        }
                        if x < (w - 1) && ys[y][x + 1] >= 0 {
                            ys[y][x + 1] += 1;
                        }
                        if y >= 1 && ys[y - 1][x] >= 0 {
                            ys[y - 1][x] += 1;
                        }
                        if y < (h - 1) && ys[y + 1][x] >= 0 {
                            ys[y + 1][x] += 1;
                        }

                        if x >= 1 && y >= 1 && ys[y - 1][x - 1] >= 0 {
                            ys[y - 1][x - 1] += 1;
                        }
                        if x < (w - 1) && y >= 1 && ys[y - 1][x + 1] >= 0 {
                            ys[y - 1][x + 1] += 1;
                        }
                        if y < (h - 1) && x < (w - 1) && ys[y + 1][x + 1] >= 0 {
                            ys[y + 1][x + 1] += 1;
                        }
                        if y < (h - 1) && x >= 1 && ys[y + 1][x - 1] >= 0 {
                            ys[y + 1][x - 1] += 1;
                        }
                    }
                })
            });
        }

        (0..w).for_each(|x| {
            (0..h).for_each(|y| {
                if ys[y][x] == -1 {
                    ys[y][x] = 0;
                }
            })
        })
    });

    Ok(format!("{}", flashes))
}

mod tests {
    use indoc::indoc;
    use crate::run_solution;
    use crate::day11::{solution1, solution2};

    const INPUT: &'static str = "day11.txt";

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!("1656",
                   solution1(indoc!("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526")).unwrap());
    }


    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2


    #[test]
    fn test_part2() {
        assert_eq!("195", solution2(indoc!("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}

