use std::str::FromStr;

pub fn parse_lines<T: FromStr>(s: &str) -> Vec<T> {
    s.lines().map(|x| x.parse::<T>().map_err(|_x| "Unable to parse").unwrap()).collect()
}