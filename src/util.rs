use itertools::Itertools;
use regex::{Captures, Regex};
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct MyError(String);

impl std::error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn parse_lines<T: FromStr>(s: &str) -> Result<Vec<T>, MyError> {
    s.lines()
        .map(|x| {
            x.parse::<T>()
                .map_err(|_x| MyError(format!("Unable to parse {:?}", x)))
        })
        .try_collect()
}

pub fn parse_lines_regex(s: &str, re: &str) -> anyhow::Result<Vec<Vec<String>>> {
    let re = Regex::new(re)?;
    Ok(s.lines()
        .map(|x| {
            re.captures(x)
                .ok_or_else(|| MyError(format!("Unable to parse {:?}", x)))
                .map(|x| x.iter().map(|y| y.unwrap().as_str().into()).collect())
        })
        .try_collect()?)
}

mod tests {
    use crate::util::{parse_lines, parse_lines_regex};

    #[test]
    fn test_parse_lines() {
        assert_eq!(parse_lines::<usize>("123\n456").unwrap(), vec![123, 456])
    }

    #[test]
    fn test_parse_lines_regex() {
        let strs = parse_lines_regex("abc 123\ncde 345", r"^([a-z]+) ([0-9]+)$").unwrap();
        assert_eq!(strs[0], vec!["abc 123", "abc", "123"])
    }
}
