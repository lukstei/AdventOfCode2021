#![allow(dead_code)]

use std::{fs};
use std::path::{Path};

mod day1;
mod day2;
mod util;


type Solution = fn(&str) -> anyhow::Result<String>;

fn main() -> anyhow::Result<()> {
    let solution: Solution = day1::solution2;
    let input_file = "day1.txt";

    // ---

    let input_path = Path::new("inputs").join(input_file);
    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Error reading file {:?}", &input_path));

    println!("Solution with input {}: {}", input_file, solution(&input)?);
    Ok(())
}


