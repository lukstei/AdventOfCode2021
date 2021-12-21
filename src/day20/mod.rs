use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use bit_set::BitSet;
use itertools::{izip, Itertools};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::Index;

pub struct ImageEnhancementAlgorithm {
    bits: BitSet,
    len: usize,
}

impl ImageEnhancementAlgorithm {
    pub fn from_str(s: &str) -> ImageEnhancementAlgorithm {
        assert_eq!(512, s.len());
        let mut bits = BitSet::with_capacity(s.len());
        for (i, c) in s.chars().enumerate() {
            if c == '#' {
                bits.insert(i);
            }
        }
        ImageEnhancementAlgorithm { bits, len: s.len() }
    }

    pub fn get_pixel(&self, i: usize) -> bool {
        self.bits.contains(i)
    }
}

impl Display for ImageEnhancementAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.len {
            write!(f, "{}", if self.bits.contains(i) { '#' } else { '.' })?
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Image {
    bits: BitSet,
    width: usize,
    height: usize,
    infinity_value: bool,
}

impl Image {
    fn is_set(&self, x: usize, y: usize) -> bool {
        self.bits
            .contains(Self::index(self.width, self.height, x, y))
    }

    pub fn index(width: usize, height: usize, x: usize, y: usize) -> usize {
        assert!(x >= 0 && x < width);
        assert!(y >= 0 && y < height);
        y * width + x
    }

    pub fn get_9_pixel_around(&self, x: usize, y: usize) -> u32 {
        let mut set = BitSet::with_capacity(9);
        for y_del in 0..=2i32 {
            for x_del in 0..=2i32 {
                let i = 8 - (x_del + y_del * 3);
                assert!(i >= 0 && i < 9);

                let my_x = x as i32 + x_del - 1;
                let my_y = y as i32 + y_del - 1;

                if my_x >= 0 && my_x < self.width as i32 && my_y >= 0 && my_y < self.height as i32 {
                    if self.is_set(my_x as usize, my_y as usize) {
                        set.insert(i as usize);
                    }
                } else {
                    if self.infinity_value {
                        set.insert(i as usize);
                    }
                }
            }
        }
        set.get_ref().storage()[0]
    }

    pub fn apply(&self, algo: &ImageEnhancementAlgorithm) -> Image {
        let mut n = self.clone();
        n.bits.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                if algo.get_pixel(self.get_9_pixel_around(x, y) as usize) {
                    n.bits.insert(Self::index(self.width, self.height, x, y));
                }
            }
        }

        n.infinity_value = n.is_set(0, 0);

        n
    }

    pub fn with_margin(&self, margin: usize) -> Image {
        let new_width = self.width + 2 * margin;
        let new_height = self.height + 2 * margin;

        let mut new = BitSet::with_capacity(new_width * new_height);

        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_set(x, y) {
                    new.insert(Self::index(new_width, new_height, x + margin, y + margin));
                }
            }
        }

        Image {
            width: new_width,
            height: new_height,
            bits: new,
            infinity_value: false,
        }
    }

    pub fn from_lines(s: &str) -> Image {
        let s = s.trim();
        let width = s.lines().next().unwrap().chars().count();
        let height = s.lines().count();

        let mut bits = BitSet::with_capacity(width * height);

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    bits.insert(Self::index(width, height, x, y));
                }
            }
        }

        Image {
            bits,
            width,
            height,
            infinity_value: false,
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self.is_set(x, y) { '#' } else { '.' })?
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn solution1(input: &str) -> Result<String> {
    let algo = ImageEnhancementAlgorithm::from_str(input.lines().next().unwrap());
    let img = Image::from_lines(input.split_once('\n').unwrap().1.trim());

    let img = img.with_margin(10);
    println!("{}", img);
    let img = img.apply(&algo);
    println!("{}", img);
    let img = img.apply(&algo);
    println!("{}", img);

    Ok(format!("{}", img.bits.len()))
}

fn solution2(input: &str) -> Result<String> {
    let algo = ImageEnhancementAlgorithm::from_str(input.lines().next().unwrap());
    let img = Image::from_lines(input.split_once('\n').unwrap().1.trim());

    let mut img = img.with_margin(120);

    for x in 0..50 {
        img = img.apply(&algo);
    }

    println!("{}", img);

    Ok(format!("{}", img.bits.len()))
}

#[cfg(test)]
mod tests {
    use crate::day20::{solution1, solution2, Image, ImageEnhancementAlgorithm};
    use crate::run_solution;
    use indoc::indoc;

    const INPUT: &'static str = "day20.txt";

    // PART 1

    #[test]
    fn test_d9() {
        let img = "#..#.
#....
##..#
..#..
..###";
        let image = Image::from_lines(img);
        assert_eq!(34, image.get_9_pixel_around(2, 2));
    }
    #[test]
    fn test_display() {
        let img = "#..#.
#....
##..#
..#..
..###";
        let algo = "..#.#..#####.#.#.#.###.##";

        assert_eq!(img, format!("{}", Image::from_lines(img)).trim());
        assert_eq!(
            algo,
            format!("{}", ImageEnhancementAlgorithm::from_str(algo))
        );
        assert_eq!(
            ".........
.........
..#..#...
..#......
..##..#..
....#....
....###..
.........
.........
",
            format!("{}", Image::from_lines(img).with_margin(2))
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!("35", solution1(indoc!("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
        #..#.
        #....
        ##..#
        ..#..
        ..###")).unwrap());
    }

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2

    #[test]
    fn test_part2() {
        assert_eq!("??", solution2(indoc!("")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}
