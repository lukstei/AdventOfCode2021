#![allow(dead_code)]
#![allow(unused_imports)]

use std::{fs};
use std::path::{Path};

mod day1;
mod day2;
mod util;


type Solution = fn(&str) -> anyhow::Result<String>;

fn main() -> anyhow::Result<()> {
    let solution: Solution = day2::solution2;
    let input_file = "day2.txt";

    // ---

    let input_path = Path::new("inputs").join(input_file);
    let input = fs::read_to_string(&input_path)?;

    println!("Solution with input {}: {}", input_file, solution(input.trim())?);
    Ok(())
}


