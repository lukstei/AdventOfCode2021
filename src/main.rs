#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_comparisons)]

use crate::day24::day24;
use std::fs;
use std::path::Path;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod template;
mod util;

type Solution = fn(&str) -> anyhow::Result<String>;

pub fn run_solution(input_file: &str, solution: Solution) -> anyhow::Result<()> {
    let input_path = Path::new("inputs").join(input_file);
    let input = fs::read_to_string(&input_path)?;

    println!(
        "Solution with input {}: {}",
        input_file,
        solution(input.trim())?
    );
    Ok(())
}

fn main() -> anyhow::Result<()> {
    Ok(())
}
