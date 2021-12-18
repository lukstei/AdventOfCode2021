use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use std::collections::{HashMap, HashSet};

fn solution1(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?)\\|(.+?)$")?;

    let ys = xs
        .iter()
        .map(|l| {
            let (v1, v2) = (
                l[1].as_str().trim().split(" ").collect_vec(),
                l[2].as_str().trim().split(" ").collect_vec(),
            );

            (v1, v2)
        })
        .collect_vec();

    let sum: usize = ys
        .iter()
        .map(|x| {
            x.1.iter()
                .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
                .count()
        })
        .sum();

    Ok(format!("{}", sum))
}

struct SegmentHypothesis {
    // index is from top to bottom, left to right
    hypothesis: [HashSet<char>; 7],
}

impl SegmentHypothesis {
    pub fn new() -> SegmentHypothesis {
        let mut sets: [HashSet<char>; 7] = Default::default();
        for x in sets.iter_mut() {
            "abcdefg".chars().for_each(|c| {
                x.insert(c);
            })
        }
        SegmentHypothesis { hypothesis: sets }
    }
}

impl SegmentHypothesis {
    fn num_by_len(len: usize) -> Vec<usize> {
        match len {
            2 => vec![1],
            3 => vec![7],
            4 => vec![4],
            5 => vec![2, 3, 5],
            6 => vec![9, 6, 0],
            7 => vec![8],
            _ => panic!(),
        }
    }

    fn idxs_by_num(num: usize) -> Vec<usize> {
        match num {
            0 => vec![0, 1, 2, 4, 5, 6],
            1 => vec![2, 5],
            2 => vec![0, 2, 3, 4, 6],
            3 => vec![0, 2, 3, 5, 6],
            4 => vec![1, 2, 3, 5],
            5 => vec![0, 1, 3, 5, 6],
            6 => vec![0, 1, 3, 4, 5, 6],
            7 => vec![0, 2, 5],
            8 => vec![0, 1, 2, 3, 4, 5, 6],
            9 => vec![0, 1, 2, 3, 5, 6],
            _ => panic!("unexpected number"),
        }
    }

    pub fn add(&mut self, wires: &str) {
        let potential_nums = Self::num_by_len(wires.len());

        //çalculate intersection of indexes
        let mut set: HashSet<usize> = (0..=9).collect();
        for x in potential_nums.iter() {
            set = HashSet::from_iter(
                set.intersection(&HashSet::from_iter(Self::idxs_by_num(*x).iter().cloned()))
                    .cloned(),
            );
        }

        for i in set {
            let p = self.hypothesis[i].clone();
            self.hypothesis[i] =
                HashSet::from_iter(p.intersection(&HashSet::from_iter(wires.chars())).cloned());
            //println!("Updating hypothesis for position {} with {} (potential nums = {}, current hypothesis = {}) -> new Hypthothesis is {}", i, wires, potential_nums.iter().join(","), p.iter().join(""), self.hypothesis[i].iter().join(""));
        }
    }

    // removes chars from other indexes where we we are sure for one index
    fn infer_trivial(&mut self) {
        for i in 0..self.hypothesis.len() {
            if self.hypothesis[i].len() == 1 {
                let sure_char = *self.hypothesis[i].iter().nth(0).unwrap();
                (0..self.hypothesis.len()).for_each(|ix| {
                    if ix != i {
                        self.hypothesis[ix].remove(&sure_char);
                    }
                })
            }
        }
    }

    pub fn decode(&self, wires: &str) -> usize {
        let translation = self
            .hypothesis
            .iter()
            .map(|h| {
                assert_eq!(h.len(), 1);
                h.iter().nth(0).unwrap().clone()
            })
            .join("");

        let mut idxs = wires
            .chars()
            .map(|c| translation.find(c).unwrap())
            .collect_vec();
        idxs.sort();
        (0..=9)
            .filter(|n| idxs == Self::idxs_by_num(*n as usize))
            .nth(0)
            .unwrap()
    }
}

fn solution2(input: &str) -> Result<String> {
    let xs = parse_lines_regex(input, "^(.+?)\\|(.+?)$")?;

    let ys = xs
        .iter()
        .map(|l| {
            let (v1, v2) = (
                l[1].as_str().trim().split(" ").collect_vec(),
                l[2].as_str().trim().split(" ").collect_vec(),
            );

            (v1, v2)
        })
        .collect_vec();

    let sum: usize = ys
        .iter()
        .map(|x| {
            let mut hypothesis = SegmentHypothesis::new();
            x.0.iter().for_each(|x| hypothesis.add(x));
            x.1.iter().for_each(|x| hypothesis.add(x));

            hypothesis.infer_trivial();
            hypothesis.infer_trivial(); // two runs are needed

            let num_str =
                x.1.iter()
                    .map(|x| hypothesis.decode(x).to_string())
                    .join("");
            num_str.parse::<usize>().unwrap()
        })
        .sum();

    Ok(format!("{}", sum))
}

mod tests {
    use crate::day8::{solution1, solution2};
    use crate::run_solution;
    use indoc::indoc;

    const INPUT: &'static str = "day8.txt";

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!("26",
                   solution1(indoc!("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce")).unwrap());
    }

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2

    #[test]
    fn test_part2() {
        assert_eq!("5353", solution2(indoc!("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |cdfeb fcadb cdfeb cdbaf")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}
