#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{fs};
use std::path::{Path};

mod day1;
mod day2;
mod util;
mod template;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

type Solution = fn(&str) -> anyhow::Result<String>;

pub fn run_solution(input_file: &str, solution: Solution) -> anyhow::Result<()> {
    let input_path = Path::new("inputs").join(input_file);
    let input = fs::read_to_string(&input_path)?;

    println!("Solution with input {}: {}", input_file, solution(input.trim())?);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    run_solution("day2.txt", day2::solution2)
}


