use anyhow::{Result};
use itertools::izip;
use crate::util::parse_lines;

pub fn solution1(input: &str) -> Result<String> {
    let numbers = parse_lines::<usize>(input);

    let r = numbers.iter().skip(1)
        .zip(numbers.iter())
        .filter(|(now,before)| now > before).count();

    Ok(format!("{}", r))
}


pub fn solution2(input: &str) -> Result<String> {
    let numbers = parse_lines::<usize>(input);

    let xs:Vec<usize> = izip!(numbers.iter().skip(2), numbers.iter().skip(1), numbers.iter())
        .map(|(x1, x2, x3)| x1 + x2 + x3).collect();

    let r = xs.iter().skip(1)
        .zip(xs.iter())
        .filter(|(now,before)| now > before).count();

    Ok(format!("{}", r))
}


mod tests {
    use indoc::indoc;
    use crate::day1::{solution1, solution2};

    #[test]
    fn test_part1() {
        assert_eq!(solution1(indoc!("199
                    200
                    208
                    210
                    200
                    207
                    240
                    269
                    260
                    263")).unwrap(), "7")
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution2(indoc!("199
                    200
                    208
                    210
                    200
                    207
                    240
                    269
                    260
                    263")).unwrap(), "5")
    }
}

