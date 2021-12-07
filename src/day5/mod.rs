use std::collections::{HashMap, HashSet};
use anyhow::{Result};
use itertools::{Itertools, izip};
use crate::util::{parse_lines, parse_lines_regex};

#[derive(Debug, Copy, Clone)]
struct Line((i32,i32), (i32,i32));

impl Line{
    pub fn is_straight(&self) -> bool {
        self.0.0==self.1.0||self.0.1==self.1.1
    }
    pub fn iter(&self) -> impl Iterator<Item=(i32,i32)> +'_{
            (self.0.0.min(self.1.0)..=self.0.0.max(self.1.0)).flat_map(|x|{
                (self.0.1.min(self.1.1)..=self.0.1.max(self.1.1)).map(move |y| (x,y))
            })
    }
}

fn solution1(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?),(.+?) -> (.+?),(.+?)$")?;

    let ys = xs.iter()
        .map(|l| {
            let (v1, v2, v3, v4) = (
                l[1].parse::<i32>().unwrap(),
                l[2].parse::<i32>().unwrap(),
                l[3].parse::<i32>().unwrap(),
                l[4].parse::<i32>().unwrap()
            );

            dbg!(Line((v1,v2), (v3,v4)))
        })
        .filter(|l|l.is_straight())
        .collect::<Vec<Line>>();

    let mut field: HashMap<(i32, i32), i32> = HashMap::new();

    for line in ys {
        for point in line.iter() {
            match field.get(&point) {
                None => field.insert(point, 1),
                Some(r) => field.insert(point, r+1)
            };
        }
    }

    let i = field.values().filter(|x| **x >= 2).count();

    Ok(format!("{}", i))
}

fn solution2(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?),(.+?) -> (.+?),(.+?)$")?;

    let ys = xs.iter()
        .map(|l| {
            let (v1, v2, v3, v4) = (
                l[1].parse::<i32>().unwrap(),
                l[2].parse::<i32>().unwrap(),
                l[3].parse::<i32>().unwrap(),
                l[4].parse::<i32>().unwrap()
            );

            (Line((v1,v2), (v3,v4)))
        })
        .collect::<Vec<Line>>();

    let mut field: HashMap<(i32, i32), i32> = HashMap::new();

    for line in ys {
        if line.is_straight(){
            (line.0.0.min(line.1.0)..=line.0.0.max(line.1.0)).flat_map(|x|{
                (line.0.1.min(line.1.1)..=line.0.1.max(line.1.1)).map(move |y| (x,y))
            }).for_each(|point|{
                match field.get(&point) {
                    None => field.insert(point, 1),
                    Some(r) => field.insert(point, r+1)
                };
            })
        } else {
            let x_dir = (line.1.0-line.0.0).signum();
            let y_dir = (line.1.1-line.0.1).signum();

            let distance = (line.0.0.max(line.1.0)-line.0.0.min(line.1.0))
                .max(line.0.1.max(line.1.1)-line.0.1.min(line.1.1));

            (0..=distance).map(|i|(line.0.0+i*x_dir,line.0.1+i*y_dir)).for_each(|point|{
                match field.get(&point) {
                    None => field.insert(point, 1),
                    Some(r) => field.insert(point, r+1)
                };
            })
        }

        //dbg!(field.keys());
    }

    (0..12).for_each(|y|{
        (0..12).for_each(|x| {
            print!("{}", field.get(&(x,y)).map_or(".".into(), |x| x.to_string()));
        });
        println!();
    });


    let i = field.values().filter(|x| **x >= 2).count();

    Ok(format!("{}", i))
}


mod tests {
    use indoc::indoc;
    use crate::run_solution;
    use crate::day5::{Line, solution1, solution2};

    // PART 1

    #[test]
    fn test_line() {
        let x: Vec<(i32,i32)> = Line((2, 3), (2, 7)).iter().collect();
        assert_eq!(x, vec![(2, 3), (2, 4), (2, 5), (2, 6), (2, 7)]);

        let y: Vec<(i32,i32)> = Line((5, 3), (1, 3)).iter().collect();
        assert_eq!(y, vec![(1, 3), (2, 3), (3, 3), (4, 3), (5, 3)]);
    }


    #[test]
    fn test_part1() {
        assert_eq!("5",
                   solution1(indoc!("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2")).unwrap());
    }

    const INPUT: &'static str = "day5.txt";

    #[test]
    fn run_solution1(){
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2


    #[test]
    fn test_part2() {
        assert_eq!("12", solution2(indoc!("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2")).unwrap());
    }

    #[test]
    fn run_solution2(){
        run_solution(INPUT, solution2).unwrap()
    }

}

