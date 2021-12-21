use crate::util::{parse_lines, parse_lines_regex};
use anyhow::Result;
use itertools::{izip, Itertools};
use std::fmt::{Display, Formatter};

type HashSet<T> = rustc_hash::FxHashSet<T>;
//type HashSet<T> = std::collections::HashSet<T>;

impl Vec3 {
    pub fn shift(&self, n: i32) -> Vec3 {
        match n {
            0 => Vec3(self.0, self.1, self.2),
            1 => Vec3(self.0, self.2, self.1),

            2 => Vec3(self.1, self.0, self.2),
            3 => Vec3(self.1, self.2, self.0),

            4 => Vec3(self.2, self.0, self.1),
            5 => Vec3(self.2, self.1, self.0),

            _ => panic!("Invalid"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Vec3(i32, i32, i32);

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{},{}]", self.0, self.1, self.2)
    }
}

struct Scanner {
    pub pos: Vec3,
    pub rot: Vec3,
    pub shift: i32,

    pub beacons: HashSet<Vec3>, // those are relative to the 0 scanner
}

impl Scanner {
    fn zero(beacons: HashSet<Vec3>) -> Scanner {
        Scanner {
            rot: Vec3(1, 1, 1),
            pos: Vec3(0, 0, 0),
            shift: 0,
            beacons,
        }
    }

    fn normalize(pos: Vec3, rot: Vec3, shift: i32, beacon: Vec3) -> Vec3 {
        beacon.shift(shift) * rot + pos
    }

    fn align(&self, detected_beacons: &HashSet<Vec3>) -> Option<Scanner> {
        let rotations = vec![
            Vec3(1, 1, 1),
            Vec3(-1, 1, 1),
            Vec3(1, -1, 1),
            Vec3(-1, -1, 1),
            Vec3(1, 1, -1),
            Vec3(-1, 1, -1),
            Vec3(1, -1, -1),
            Vec3(-1, -1, -1),
        ];

        for rot in rotations {
            for &alignment_beacon in detected_beacons {
                for &self_alignment in &self.beacons {
                    for shift in 0..6 {
                        // assumption alignment_beacon == self_alignment

                        let alignment_beacon = alignment_beacon.shift(shift);

                        let pos = self_alignment - alignment_beacon * rot;
                        let intersections = detected_beacons
                            .iter()
                            .filter(|x| {
                                self.beacons
                                    .contains(&Scanner::normalize(pos, rot, shift, **x))
                            })
                            .count();

                        assert!(intersections >= 1);
                        if intersections >= 12 {
                            return Some(Scanner {
                                pos,
                                beacons: detected_beacons
                                    .iter()
                                    .map(|x| Scanner::normalize(pos, rot, shift, *x))
                                    .collect(),
                                shift,
                                rot,
                            });
                        }
                    }
                }
            }
        }

        None
    }
}

fn read_list(s: &str) -> HashSet<Vec3> {
    s.trim()
        .lines()
        .map(|x| {
            let vec = x.trim().split(',').collect_vec();
            Vec3(
                vec[0].parse().unwrap(),
                vec[1].parse().unwrap(),
                vec[2].parse().unwrap(),
            )
        })
        .collect()
}

fn solution1(input: &str) -> Result<String> {
    let mut r: Vec<HashSet<Vec3>> = vec![];
    let mut c: String = "".to_string();
    for l in input.lines().skip(1) {
        if l.starts_with("---") {
            r.push(read_list(&c));
            c = String::new();
        } else {
            c += l;
            c += "\n";
        }
    }
    r.push(read_list(&c));

    let mut finished = vec![Scanner::zero(r.remove(0))];

    loop {
        let mut new_finished: Vec<Scanner> = Vec::new();

        for scanner in &finished {
            r = r
                .into_iter()
                .filter(|x| match scanner.align(x) {
                    None => true,
                    Some(successes) => {
                        new_finished.push(successes);
                        false
                    }
                })
                .collect_vec();
        }

        if new_finished.len() == 0 {
            break;
        }
        finished.append(&mut new_finished);
    }

    Ok(format!(
        "{}",
        finished
            .iter()
            .flat_map(|x| x.beacons.clone())
            .unique()
            .count()
    ))
}

pub(crate) fn solution2(input: &str) -> Result<String> {
    let mut r: Vec<HashSet<Vec3>> = vec![];
    let mut c: String = "".to_string();
    for l in input.lines().skip(1) {
        if l.starts_with("---") {
            r.push(read_list(&c));
            c = String::new();
        } else {
            c += l;
            c += "\n";
        }
    }
    r.push(read_list(&c));

    let mut finished = vec![Scanner::zero(r.remove(0))];

    loop {
        let mut new_finished: Vec<Scanner> = Vec::new();

        for scanner in &finished {
            r = r
                .into_iter()
                .filter(|x| match scanner.align(x) {
                    None => true,
                    Some(successes) => {
                        new_finished.push(successes);
                        false
                    }
                })
                .collect_vec();
        }

        if new_finished.len() == 0 {
            break;
        }
        finished.append(&mut new_finished);
    }

    let max = finished
        .iter()
        .tuple_combinations()
        .map(|(x1, x2)| {
            let dis: Vec3 = x1.pos - x2.pos;
            dis.0.abs() + dis.1.abs() + dis.2.abs()
        })
        .max()
        .unwrap();

    Ok(format!("{}", max))
}

mod tests {
    use crate::day19::{read_list, solution1, solution2, Scanner, Vec3};
    use crate::run_solution;
    use indoc::indoc;

    const INPUT: &'static str = "day19.txt";

    // PART 1

    #[test]
    fn test_1() {
        let l1 = read_list(
            "404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401",
        );

        let l2 = read_list(
            "686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390",
        );

        let l4 = read_list(
            "727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14",
        );

        let zero = Scanner::zero(l1);
        let scanner2 = zero.align(&l2).unwrap();
        assert_eq!("[68,-1246,-43]", format!("{}", scanner2.pos));

        assert_eq!(
            "[-618,-824,-621]",
            format!(
                "{}",
                Scanner::normalize(
                    scanner2.pos,
                    scanner2.rot,
                    scanner2.shift,
                    Vec3(686, 422, 578)
                )
            )
        );

        let scanner3 = scanner2.align(&l4).unwrap();
        assert_eq!("[-20,-1133,1061]", format!("{}", scanner3.pos));
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            "79",
            solution1(indoc!(
                "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"
            ))
            .unwrap()
        );
    }

    #[test]
    fn run_solution1() {
        run_solution(INPUT, solution1).unwrap()
    }

    // PART 2
    #[test]
    fn test_part2() {
        assert_eq!(
            "3621",
            solution2(indoc!(
                "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"
            ))
            .unwrap()
        );
    }

    #[test]
    fn run_solution2() {
        run_solution(INPUT, solution2).unwrap()
    }
}
