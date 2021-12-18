use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use std::collections::{HashMap, HashSet};

fn solution1<'a>(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?)$")?;

    let ys = xs
        .iter()
        .map(|l| {
            let (v1) = (l[1]
                .as_str()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect_vec());

            v1
        })
        .collect::<Vec<Vec<i32>>>();

    use pathfinding::prelude::dijkstra;

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos<'a>(i32, i32, i32, &'a Vec<Vec<i32>>);

    impl<'a> Pos<'a> {
        fn successors(&self) -> Vec<(Pos<'a>, usize)> {
            let &Pos(x, y, c, xs) = self;

            vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
                .into_iter()
                .filter(|(x, y)| {
                    *x >= 0
                        && *y >= 0
                        && (*x < self.3[0].len() as i32)
                        && (*y < self.3.len() as i32)
                })
                .map(|(x, y)| {
                    let weight = self.3[y as usize][x as usize];
                    (Pos(x, y, weight, xs), weight as usize)
                })
                .collect()
        }
    }

    let GOAL: Pos = Pos(
        (ys[0].len() - 1) as i32,
        ys.len() as i32,
        ys[ys.len() - 1][ys[0].len() - 1],
        &ys,
    );
    let result = dijkstra(
        &Pos(0, 0, ys[0][0], &ys),
        |p: &Pos| p.successors(),
        |p| p.0 == (ys[0].len() - 1) as i32 && p.1 == (ys.len() - 1) as i32,
    );
    Ok(format!("{}", result.unwrap().1))
}

fn solution2<'a>(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?)$")?;

    let ys = xs
        .iter()
        .map(|l| {
            let (v1) = (l[1]
                .as_str()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect_vec());

            v1
        })
        .collect::<Vec<Vec<i32>>>();

    let x_len = ys[0].len();
    let y_len = ys.len();

    let mut nys = (0..y_len * 5)
        .map(|y| (0..x_len * 5).map(|x| 0).collect_vec())
        .collect_vec();

    for y in 0..y_len * 5 {
        for x in 0..x_len * 5 {
            let mul_x = (x % x_len) as i32;
            let mul_y = (y % y_len) as i32;

            let mut w = ys[mul_y as usize][mul_x as usize];
            w = w + (x / x_len) as i32 + (y / y_len) as i32;
            if w > 9 {
                w -= 9;
            }

            nys[y][x] = w;
            print!("{}", w);
        }
        println!();
    }

    let ys = nys;

    use pathfinding::prelude::dijkstra;

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos<'a>(i32, i32, i32, &'a Vec<Vec<i32>>);

    impl<'a> Pos<'a> {
        fn successors(&self) -> Vec<(Pos<'a>, usize)> {
            let &Pos(x, y, c, xs) = self;

            vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
                .into_iter()
                .filter(|(x, y)| {
                    *x >= 0
                        && *y >= 0
                        && (*x < self.3[0].len() as i32)
                        && (*y < self.3.len() as i32)
                })
                .map(|(x, y)| {
                    let weight = self.3[y as usize][x as usize];
                    (Pos(x, y, weight, xs), weight as usize)
                })
                .collect()
        }
    }

    let GOAL: Pos = Pos(
        (ys[0].len() - 1) as i32,
        ys.len() as i32,
        ys[ys.len() - 1][ys[0].len() - 1],
        &ys,
    );
    let result = dijkstra(
        &Pos(0, 0, ys[0][0], &ys),
        |p: &Pos| p.successors(),
        |p| p.0 == (ys[0].len() - 1) as i32 && p.1 == (ys.len() - 1) as i32,
    );
    Ok(format!("{}", result.unwrap().1))
}

mod tests {
    use crate::day15::{solution1, solution2};
    use crate::run_solution;
    use indoc::indoc;
    const INPUT: &'static str = "day15.txt";

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!(
            "40",
            solution1(indoc!(
                "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
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
        assert_eq!(
            "40",
            solution2(indoc!(
                "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            ))
            .unwrap()
        );
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}
