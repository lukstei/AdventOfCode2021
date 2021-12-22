use std::collections::VecDeque;
use std::mem::swap;
use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};

type HashSet<T> = rustc_hash::FxHashSet<T>;

fn solution1(input: &str) -> Result<String> {
    let mut cubes: HashSet<(i32, i32, i32)> = Default::default();

    let xs = parse_lines_regex(input, r"^(on|off) x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)$")?;

    let limit = 50;

    let ys = xs
        .iter()
        .map(|l| {
            let (on_off, x1, x2, y1, y2, z1, z2) = (
                l[1].as_str(),
                l[2].parse::<i32>().unwrap(),
                l[3].parse::<i32>().unwrap(),
                l[4].parse::<i32>().unwrap(),
                l[5].parse::<i32>().unwrap(),
                l[6].parse::<i32>().unwrap(),
                l[7].parse::<i32>().unwrap(),
            );

            (on_off == "on", (x1.max(-limit)..=x2.min(limit)), (y1.max(-limit)..=y2.min(limit)), (z1.max(-limit)..=z2.min(limit)))
        })

        .for_each(|(on, xr, yr, zr)| {
            for x in xr.clone() {
                for y in yr.clone() {
                    for z in zr.clone() {
                        if on {
                            cubes.insert((x, y, z));
                        } else {
                            cubes.remove(&(x, y, z));
                        }
                    }
                }
            }
        });

    Ok(format!("{:?}", cubes.len()))
}


struct Cube((i32, i32), (i32, i32), (i32, i32));

impl Cube {
    fn new(x: (i32, i32), y: (i32, i32), z: (i32, i32)) -> Cube {
        assert!(x.0<=x.1);
        assert!(y.0<=y.1);
        assert!(z.0<=z.1);
        Cube(x, y, z)
    }

    fn point_intersection(mut p1: (i32, i32), mut p2: (i32, i32)) -> Option<(i32, i32)> {
        if p1.0 > p2.0 {
            swap(&mut p1, &mut p2);
        }

        if p1.1 <= p2.0 {
            None
        } else if p1.0 <= p2.0 && p1.1 >= p2.1 {
            Some(p2)
        } else {
            Some((p2.0, p1.1))
        }
    }

    fn volume(&self) -> u64 {
        (1  + self.0.1 - self.0.0) as u64 * (1 + self.1.1 - self.1.0) as u64 * (1 + self.2.1 - self.2.0) as u64
    }

    fn intersection(&self, o: &Cube) -> Option<Cube> {
        Self::point_intersection(self.0, o.0).zip(
            Self::point_intersection(self.1, o.1).zip(
                Self::point_intersection(self.2, o.2))
        ).map(|(x, (y, z))| Cube::new(x, y, z))
    }
}

struct CubeStack {}

impl CubeStack {
    fn area(stack: Vec<(bool, Cube)>) -> u64 {
        let mut area = 0;
        let mut counted: Vec<&Cube> = Default::default();
        let mut i = 0;
        for (on, cube) in stack.iter().rev() {
            println!("Num {}", i);
            if *on {
                area += Self::get_additional_volume(cube, &counted);
            }
            counted.push(cube);
            i += 1;
        }
        area
    }

    fn get_additional_volume(x: &Cube, counted: &Vec<&Cube>) ->u64 {
        let mut volume = x.volume();
        let mut set: HashSet<(i32, i32, i32)> = Default::default();

        let intersections = counted.iter().filter_map(|c| x.intersection(c)).collect_vec();
        let mut removed: Vec<&Cube> = Default::default();
        for intersection in intersections.iter() {
            //volume -= intersection.volume();
            volume -= Self::get_additional_volume(intersection, &removed);
            removed.push(intersection);
        }

        volume
    }
}

pub(crate) fn solution2(input: &str) -> Result<String> {
    let mut cubes: HashSet<(i32, i32, i32)> = Default::default();

    let limit = 50;


    let xs = parse_lines_regex(input, r"^(on|off) x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)$")?;

    let ys = xs
        .iter()
        .map(|l| {
            let (on_off, x1, x2, y1, y2, z1, z2) = (
                l[1].as_str(),
                l[2].parse::<i32>().unwrap(),
                l[3].parse::<i32>().unwrap(),
                l[4].parse::<i32>().unwrap(),
                l[5].parse::<i32>().unwrap(),
                l[6].parse::<i32>().unwrap(),
                l[7].parse::<i32>().unwrap(),
            );

            //(on_off == "on", (x1..=x2), (y1..=y2), (z1..=z2))
            (on_off == "on", (x1.max(-limit)..=x2.min(limit)), (y1.max(-limit)..=y2.min(limit)), (z1.max(-limit)..=z2.min(limit)))
        })
        .filter(|(on, xr, yr, zr)| {
            !xr.is_empty()&&!yr.is_empty()&&!zr.is_empty()
        })
        .map(|(on, xr, yr, zr)| {
            (on, Cube::new((*xr.start(), *xr.end()), (*yr.start(), *yr.end()), (*zr.start(), *zr.end())))
        }).collect_vec();

    Ok(format!("{:?}", CubeStack::area(ys)))
}

#[cfg(test)]
mod tests {
    use crate::run_solution;
    use crate::day22::{solution1, solution2};
    use indoc::indoc;

    const INPUT: &'static str = "day22.txt";

    // PART 1

    #[test]
    fn test_part1() {
        assert_eq!("590784", solution1(indoc!("on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682")).unwrap());
    }

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2

    #[test]
    fn test_part2() {
        assert_eq!("590784", solution2(indoc!("on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682")).unwrap());
    }

    #[test]
    fn test_part222() {
        assert_eq!("39", solution2(indoc!("on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10")).unwrap());
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}
